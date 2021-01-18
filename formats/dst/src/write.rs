use std::io::Write;

use embroidery_lib::format::PatternWriter;
use embroidery_lib::prelude::*;
use embroidery_lib::utils::{c_trim, char_truncate};

use crate::stitch_info::{StitchInformation, StitchType};

const MAX_JUMP: i32 = 121;

pub struct DstPatternWriter {}

impl Default for DstPatternWriter {
    fn default() -> Self {
        DstPatternWriter {}
    }
}

impl PatternWriter for DstPatternWriter {
    fn write_pattern(&self, pattern: &Pattern, writer: &mut dyn Write) -> Result<(), WriteError> {
        let stitches = into_dst_stitches(pattern)?;
        write_header(pattern, &stitches, writer)?;
        write_stitches(&stitches, writer)?;

        Ok(())
    }
}

fn write_header(
    pattern: &Pattern,
    dst_stitches: &[StitchInformation],
    writer: &mut dyn Write,
) -> Result<(), WriteError> {
    let mut header: Vec<u8> = Vec::with_capacity(512);
    header.extend(build_header(pattern, dst_stitches)?);
    let rem_space = 512 - header.len();
    header.extend(build_extended_header(pattern, rem_space)?);
    assert!(header.len() <= 512);
    header.resize(512, 0_u8);

    writer.write_all(&header)?;
    Ok(())
}

fn write_stitches(dst_stitches: &[StitchInformation], writer: &mut dyn Write) -> Result<(), WriteError> {
    assert_eq!(Some(&StitchInformation::End), dst_stitches.last());
    assert_eq!(1, dst_stitches.iter().filter(|&&s| s == StitchInformation::End).count());
    for &st in dst_stitches {
        // Use unwrap because any stitch that's invalid here is definately a program error.
        writer.write_all(&st.to_bytes().unwrap())?;
        if StitchInformation::End == st {
            break;
        }
    }
    Ok(())
}

fn build_header(pattern: &Pattern, dst_stitches: &[StitchInformation]) -> Result<Vec<u8>, WriteError> {
    let mut data: Vec<u8> = Vec::with_capacity(128);
    let color_count = pattern.color_groups.len();
    let stitch_count = dst_stitches.len();
    let (minx, miny, maxx, maxy) = pattern.get_bounds();

    let title = char_truncate(&c_trim(&pattern.name), 17);

    let title_padding: String = std::iter::repeat(' ').take(17 - title.bytes().len()).collect();

    // Cannot use `{: >17}` as it does not handle multi-byte characters in the way we need to
    write!(data, "LA:{}{}\r", title, title_padding)?;
    write!(data, "ST:{: >7}\r", stitch_count)?;
    // `CO` represents the number of color changes.
    write!(data, "CO:{: >3}\r", color_count.saturating_sub(1))?;
    write!(data, "+X:{: <5}\r", (10. * maxx) as i64)?;
    write!(data, "-X:{: <5}\r", (10. * minx) as i64)?;
    write!(data, "+Y:{: <5}\r", (10. * maxy) as i64)?;
    write!(data, "-Y:{: <5}\r", (10. * miny) as i64)?;

    // Required fields; but not actually needing to be calculated
    write!(data, "AX:{: <+6}\r", 0)?;
    write!(data, "AY:{: <+6}\r", 0)?;
    write!(data, "MX:{: <+6}\r", 0)?;
    write!(data, "MY:{: <+6}\r", 0)?;
    write!(data, "PD:{: <6}\r\0\0\0", ['*'; 6].iter().collect::<String>())?;

    assert!(data.len() == 128);
    Ok(data)
}

fn build_extended_header(pattern: &Pattern, rem: usize) -> Result<Vec<u8>, WriteError> {
    let mut data: Vec<u8> = Vec::with_capacity(128);
    let author = pattern
        .attributes
        .iter()
        .filter_map(|attr| match attr {
            PatternAttribute::Author(title) => Some(title.to_string()),
            _ => None,
        })
        .next();
    let copyright = pattern
        .attributes
        .iter()
        .filter_map(|attr| match attr {
            PatternAttribute::Copyright(title) => Some(title.to_string()),
            _ => None,
        })
        .next();

    if let Some(a) = author {
        write!(data, "AU:{: <17}\r", char_truncate(&c_trim(&a), 17))?;
    }
    if let Some(c) = copyright {
        write!(data, "CP:{: <17}\r", char_truncate(&c_trim(&c), 17))?;
    }
    assert!(data.len() <= rem);
    Ok(data)
}

fn into_dst_stitches(pattern: &Pattern) -> Result<Vec<StitchInformation>, WriteError> {
    let mut re = vec![];
    let mut inter_group_jumps = vec![];
    let mut ox: i32 = 0;
    let mut oy: i32 = 0;
    let mut idx: usize = 0;
    let mut last_was_stop = false;

    for cg in &pattern.color_groups {
        for sg in &cg.stitch_groups {
            let mut iter = sg.stitches.iter();
            if let Some(s) = iter.next() {
                inter_group_jumps.append(&mut safe_jump_to(ox, oy, s));
                ox = (s.x * 10.) as i32;
                oy = (s.y * 10.) as i32;
            }
            if last_was_stop {
                if let Some(&StitchInformation::Move(dx, dy, typ)) = inter_group_jumps.first() {
                    inter_group_jumps.push(StitchInformation::Move(dx, dy, typ.with_stop()));
                    inter_group_jumps.swap_remove(0);
                } else {
                    inter_group_jumps.push(StitchInformation::Move(0, 0, StitchType::Stop))
                }
            }
            re.append(&mut inter_group_jumps);
            for s in iter {
                let dx = ((s.x * 10.).trunc() as i32) - ox;
                let dy = ((s.y * 10.).trunc() as i32) - oy;
                if dx.abs() > MAX_JUMP || dy.abs() > MAX_JUMP {
                    return Err(WriteError::unsupported_stitch_msg(
                        *s,
                        Some(idx),
                        "Stitch jump is too big for the format",
                    ));
                }
                ox += dx;
                oy += dy;
                re.push(StitchInformation::Move(dx as i8, dy as i8, StitchType::Regular));
                idx += 1;
            }
            if sg.cut {
                inter_group_jumps.append(&mut generate_cut());
            }
        }
        last_was_stop = true;
    }
    inter_group_jumps.append(&mut safe_jump_to(ox, oy, &Stitch::zero()));
    if let Some(&StitchInformation::Move(dx, dy, typ)) = inter_group_jumps.first() {
        inter_group_jumps.push(StitchInformation::Move(dx, dy, typ.with_stop()));
        inter_group_jumps.swap_remove(0);
    } else {
        inter_group_jumps.push(StitchInformation::Move(0, 0, StitchType::Stop))
    }
    re.append(&mut inter_group_jumps);
    re.push(StitchInformation::End);
    Ok(re)
}

fn safe_jump_to(ox: i32, oy: i32, s: &Stitch) -> Vec<StitchInformation> {
    let delta_x = ((s.x * 10.) as i32) - ox;
    let delta_y = ((s.y * 10.) as i32) - oy;

    if delta_x == 0 && delta_y == 0 {
        Vec::new()
    } else if delta_x.abs() <= MAX_JUMP && delta_y.abs() <= MAX_JUMP {
        vec![StitchInformation::Move(delta_x as i8, delta_y as i8, StitchType::Jump)]
    } else {
        let abs_x = delta_x.abs();
        let abs_y = delta_y.abs();
        let sign_x = delta_x.signum() as i8;
        let sign_y = delta_y.signum() as i8;
        let chunks = f64::max(
            (f64::from(abs_x) / f64::from(MAX_JUMP)).ceil(),
            (f64::from(abs_y) / f64::from(MAX_JUMP)).ceil(),
        ) as i32;
        let step_x = (f64::from(abs_x) / f64::from(chunks)).ceil() as i32;
        let step_y = (f64::from(abs_y) / f64::from(chunks)).ceil() as i32;
        assert!(step_x <= MAX_JUMP);
        assert!(step_y <= MAX_JUMP);
        let mut cx = 0;
        let mut cy = 0;
        let mut re = Vec::with_capacity(chunks as usize);
        for i in 0..=chunks {
            let (nx, ny) = (i32::min(abs_x, i * step_x), i32::min(abs_y, i * step_y));
            re.push(StitchInformation::Move(
                sign_x * (nx - cx) as i8,
                sign_y * (ny - cy) as i8,
                StitchType::Jump,
            ));
            cx = nx;
            cy = ny;
        }
        re
    }
}

fn generate_cut() -> Vec<StitchInformation> {
    // TODO: Make this a write option so we can customise it.
    vec![
        StitchInformation::Move(2, 0, StitchType::Jump),
        StitchInformation::Move(-1, 0, StitchType::Jump),
        StitchInformation::Move(-1, 0, StitchType::Jump),
        StitchInformation::Move(0, 0, StitchType::Jump),
    ]
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_build_header() {
        let result = build_header(
            &Pattern {
                name: "Ä".to_owned(),
                attributes: vec![],
                color_groups: vec![],
            },
            &[],
        )
        .unwrap();

        assert_eq!("LA:Ä               \rST:      0\rCO:  0\r+X:0    \r-X:0    \r+Y:0    \r-Y:0    \rAX:+0    \rAY:+0    \rMX:+0    \rMY:+0    \rPD:******\r\u{0}\u{0}\u{0}", String::from_utf8(result).unwrap());
    }
}
