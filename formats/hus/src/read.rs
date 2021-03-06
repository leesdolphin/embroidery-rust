use std::io::Read;

use archivelib::{do_decompress_level, CompressionLevel};
use embroidery_lib::format::PatternReader;
use embroidery_lib::prelude::*;

use crate::colors::read_threads;
use crate::header::PatternHeader;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum HusVipStitchType {
    Normal,
    Jump,
    ColorChange,
    LastStitch,
}

#[derive(Default)]
pub struct HusVipPatternReader {}

impl PatternReader for HusVipPatternReader {
    fn is_loadable(&self, item: &mut dyn Read) -> Result<bool, ReadError> {
        // Load the header
        // Check the last byte of the file? maybe
        match PatternHeader::build(item) {
            Err(ReadError::InvalidFormat(_, _)) => Ok(false),
            Err(error) => Err(error),
            Ok(_) => Ok(true),
        }
    }

    fn read_pattern(&self, item: &mut dyn Read) -> Result<Pattern, ReadError> {
        // Read the header
        let header = PatternHeader::build(item)?;
        let threads = read_threads(&header, item)?;
        let attributes = read_attributes(&header, item)?;
        let x_coords = read_x_coords(&header, item)?;
        let y_coords = read_y_coords(&header, item)?;
        debug!(
            "attributes: {}, x_coords: {}, y_coords: {}",
            attributes.len(),
            x_coords.len(),
            y_coords.len()
        );
        if attributes.len() != x_coords.len() || attributes.len() != y_coords.len() {
            return Err(ReadError::invalid_format(format!(
                "Different numbers of attributes({}), x coordinates({}) and y coordinates({})",
                attributes.len(),
                x_coords.len(),
                y_coords.len()
            )));
        }

        // let color_groups = read_stitches(&mut iter)?;
        // let (title, attributes) = extract_title(attributes);
        let mut pattern_attrs = vec![];

        if !header.title.is_empty() {
            pattern_attrs.push(PatternAttribute::Title(header.title.clone()));
        }
        Ok(Pattern {
            name: header.title,
            attributes: pattern_attrs,
            color_groups: convert_stitches(threads, &attributes, &x_coords, &y_coords),
        })
    }
}

fn decompress(item: &mut dyn Read, len_opt: Option<usize>) -> Result<Box<[u8]>, ReadError> {
    let data = if let Some(len) = len_opt {
        let mut d = vec![0; len];
        item.read_exact(&mut d)?;
        d
    } else {
        let mut d = Vec::with_capacity(256);
        item.read_to_end(&mut d)?;
        d
    };
    println!("{:X?}", data);
    match do_decompress_level(&data, CompressionLevel::Level4) {
        Ok(d) => Ok(d),
        Err(e) => Err(ReadError::invalid_format(format!("Decompression failed: {:?}", e))),
    }
}

fn read_attributes(header: &PatternHeader, item: &mut dyn Read) -> Result<Vec<HusVipStitchType>, ReadError> {
    let data = decompress(item, Some(header.attribute_len()))?;
    let mut attrs = Vec::with_capacity(data.len());
    for (i, attr) in data.iter().enumerate() {
        attrs.push(match attr {
            0x80 => HusVipStitchType::Normal,      // Normal stitch
            0x81 => HusVipStitchType::Jump,        // Jump stitch
            0x84 => HusVipStitchType::ColorChange, // Color change
            0x90 => HusVipStitchType::LastStitch,  // Last stitch in pattern

            0x88 => {
                // Additional stitch type. Likely to be a cut stitch.
                HusVipStitchType::Jump
            },
            _ => {
                return Err(ReadError::invalid_format(format!(
                    "Invalid attribute({}) at stitch {}",
                    attr, i
                )));
            },
        });
    }
    if !attrs.is_empty() {
        if attrs.last() != Some(&HusVipStitchType::LastStitch) {
            return Err(ReadError::invalid_format(format!(
                "Invalid last stitch type: {:?}",
                attrs.last()
            )));
        }
        let mut invalid_stitches = attrs
            .iter()
            .enumerate()
            .filter(|(_, &attr)| attr == HusVipStitchType::LastStitch)
            .collect::<Vec<_>>();
        if invalid_stitches.len() > 1 {
            invalid_stitches.truncate(invalid_stitches.len() - 1);
            let mut error = "".to_string();
            for (i, _) in invalid_stitches {
                if !error.is_empty() {
                    error += ", ";
                }
                error += &format!("{}", i);
            }
            return Err(ReadError::invalid_format(format!(
                "Last stitches not at the last position:\n{}",
                error
            )));
        }
    }
    Ok(attrs)
}

fn read_x_coords(header: &PatternHeader, item: &mut dyn Read) -> Result<Vec<i32>, ReadError> {
    let data = decompress(item, Some(header.x_offset_len()))?;
    // x coordinates are in 0.1mm increments
    let mut curr_x: i32 = 0;
    let mut xs = Vec::with_capacity(data.len());
    for &x_u8 in data.iter() {
        let delta: i32 = i8::from_be_bytes([x_u8]).into();
        curr_x += delta;
        xs.push(curr_x);
    }
    Ok(xs)
}

fn read_y_coords(_header: &PatternHeader, item: &mut dyn Read) -> Result<Vec<i32>, ReadError> {
    let data = decompress(item, None)?;
    // x coordinates are in 0.1mm increments
    let mut curr_y = 0;
    let mut ys = Vec::with_capacity(data.len());
    for &y_u8 in data.iter() {
        let delta: i32 = i8::from_be_bytes([y_u8]).into();
        curr_y += delta;
        ys.push(curr_y);
    }
    Ok(ys)
}

fn convert_stitches(
    threads: Vec<Thread>,
    attributes: &[HusVipStitchType],
    x_coords: &[i32],
    y_coords: &[i32],
) -> Vec<ColorGroup> {
    let combi_iter = attributes.iter().zip(x_coords.iter().zip(y_coords));
    let mut color_groups = Vec::new();
    let mut stitch_groups = Vec::new();
    let mut stitches = Vec::new();
    let mut last_jump: Option<(i32, i32)> = None;

    for (attr, (&x, &y)) in combi_iter {
        match attr {
            HusVipStitchType::Normal => {
                if let Some((jx, jy)) = last_jump {
                    stitches.push(Stitch {
                        x: f64::from(jx) / 10.0,
                        y: f64::from(jy) / 10.0,
                    });
                    last_jump = None;
                }
                stitches.push(Stitch {
                    x: f64::from(x) / 10.0,
                    y: f64::from(y) / 10.0,
                });
            },
            HusVipStitchType::Jump => {
                if !stitches.is_empty() {
                    let old_stitches = stitches;
                    stitches = Vec::new();
                    stitch_groups.push(old_stitches);
                }
                last_jump = Some((x, y));
            },
            HusVipStitchType::ColorChange => {
                if !stitches.is_empty() {
                    let old_stitches = stitches;
                    stitches = Vec::new();
                    stitch_groups.push(old_stitches);
                }
                if !stitch_groups.is_empty() {
                    let old_sg = stitch_groups;
                    stitch_groups = Vec::new();
                    color_groups.push(old_sg);
                }
                last_jump = Some((x, y));
            },
            HusVipStitchType::LastStitch => {
                break;
            },
        }
    }
    if !stitches.is_empty() {
        stitch_groups.push(stitches);
    }
    if !stitch_groups.is_empty() {
        color_groups.push(stitch_groups);
    }
    let mut thread_iter = threads.into_iter();
    color_groups
        .into_iter()
        .map(|stitch_groups| ColorGroup {
            thread: thread_iter.next(),
            stitch_groups: stitch_groups
                .into_iter()
                .map(|stitches| StitchGroup {
                    stitches,
                    cut: true,
                    trim: true,
                })
                .collect(),
        })
        .collect()
}
