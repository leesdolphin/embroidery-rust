use byteorder::BigEndian;

use embroidery_lib::prelude::*;
use embroidery_lib::{maybe_read_with_context, read_exact_magic, read_int};

use std::convert::TryInto;
use std::io::{Read, Take};

use super::util::read_ascii_string_field;

#[derive(Debug, PartialEq)]
pub struct Vp3ThreadHeader {
    pub thread_length: u32,
    pub x_offset_a: i32,
    pub y_offset_a: i32,
    pub color: (u8, u8, u8),
    pub color_table: Vec<[u8; 6]>,
    pub thread_code: String,
    pub thread_name: String,
    pub thread_manufacturer: String,
    pub x_offset_b: i32,
    pub y_offset_b: i32,
    pub stitch_bytes: usize,
}

impl Vp3ThreadHeader {
    pub fn to_thread(&self) -> Thread {
        let color = {
            let (red, green, blue) = self.color;
            Color::rgb(red, green, blue)
        };
        let mut thread = Thread::new_str(color, &self.thread_name, &self.thread_code);
        thread.manufacturer = Some(self.thread_manufacturer.to_string());
        thread.attributes.insert(
            "color_table_hex".to_string(),
            self.color_table
                .iter()
                .map(|v| v.iter().map(|h| format!("{:02X}", h)).collect::<Vec<_>>().join("\n"))
                .collect::<Vec<_>>()
                .join("\n"),
        );

        thread
    }
}

pub fn read_threads(mut reader: &mut dyn Read, threads: usize) -> Result<Vec<ColorGroup>, ReadError> {
    let mut cgs = Vec::with_capacity(threads);
    for i in 0..threads {
        let (mut thread_reader, thread_header) =
            maybe_read_with_context!(read_thread_header(&mut reader), "Reading thread {} of {}", i, threads,)?;
        let stitches = maybe_read_with_context!(
            read_stitches(&mut thread_reader, &thread_header),
            "Reading thread {} of {}",
            i,
            threads,
        )?;
        cgs.push(ColorGroup {
            thread: Some(thread_header.to_thread()),
            stitch_groups: stitches,
        });
    }
    Ok(cgs)
}

#[allow(clippy::cognitive_complexity)]
pub fn read_thread_header(ub_reader: &mut dyn Read) -> Result<(Take<&mut dyn Read>, Vp3ThreadHeader), ReadError> {
    read_exact_magic!(ub_reader, [0x00, 0x05, 0x00])?;
    let thread_length = read_int!(ub_reader, u32, BigEndian)?;
    let mut reader = ub_reader.take(thread_length.into());
    let x_offset_a = read_int!(reader, i32, BigEndian)?;
    let y_offset_a = read_int!(reader, i32, BigEndian)?;

    let table_multiplier: usize = read_int!(reader, u8)?.into();
    let color = (read_int!(reader, u8)?, read_int!(reader, u8)?, read_int!(reader, u8)?);
    // Maybe colour table?
    let mut color_table = Vec::with_capacity(table_multiplier * 6);
    for _ in 0..table_multiplier {
        let mut table = [0_u8; 6];
        reader.read_exact(&mut table)?;
        color_table.push(table);
    }

    let thread_code = read_ascii_string_field(&mut reader, "thread_code")?;
    let thread_name = read_ascii_string_field(&mut reader, "thread_name")?;
    let thread_manufacturer = read_ascii_string_field(&mut reader, "thread_manufacturer")?;

    let x_offset_b = read_int!(reader, i32, BigEndian)?;
    let y_offset_b = read_int!(reader, i32, BigEndian)?;

    // A read_ascii_string_field with apparently constant values
    read_exact_magic!(reader, [0x00, 0x01, 0x00])?;
    let stitch_bytes = read_int!(reader, u32, BigEndian)?.try_into().unwrap();
    Ok((
        reader,
        Vp3ThreadHeader {
            thread_length,
            x_offset_a,
            y_offset_a,
            color,
            color_table,
            thread_code,
            thread_name,
            thread_manufacturer,
            x_offset_b,
            y_offset_b,
            stitch_bytes,
        },
    ))
}

pub fn read_stitches(ub_reader: &mut dyn Read, thread: &Vp3ThreadHeader) -> Result<Vec<StitchGroup>, ReadError> {
    let mut stitch_bytes: usize = thread.stitch_bytes;
    let mut all_bytes = vec![0u8; stitch_bytes + 1];
    ub_reader.read_exact(&mut all_bytes)?;
    let mut reader = &all_bytes[..];

    read_exact_magic!(reader, [0xA, 0xF6, 0x0,])?;
    stitch_bytes -= 3;

    if thread.stitch_bytes % 2 == 0 {
        return Err(ReadError::invalid_format(format!(
            "Incorrect stitch length for thread {}. Expected an even value, got {:?}",
            thread.thread_code, thread.stitch_bytes
        )));
    }
    let mut stitch_groups = Vec::new();
    let mut stitches = Vec::new();
    let mut cx: i32 = thread.x_offset_a;
    let mut cy: i32 = thread.y_offset_a;

    while stitch_bytes >= 2 {
        let (read_bytes, stitch) = maybe_read_with_context!(
            read_stitch(&mut reader),
            "Read failed with {} reported bytes remaining",
            stitch_bytes
        )?;
        stitch_bytes = match stitch_bytes.checked_sub(read_bytes) {
            Some(b) => b,
            None => {
                return Err(ReadError::invalid_format(format!(
                    "Invalid final stitch consumed too many bytes. Remaining bytes {}, consumed {}, stitch: {:?}",
                    stitch_bytes, read_bytes, stitch
                )));
            },
        };

        match stitch {
            Vp3Stitch::Normal(x, y) => {
                if stitches.is_empty() {
                    stitches.push(Stitch::new(f64::from(cx) / 1000., f64::from(cy) / 1000.));
                }
                cx += x;
                cy += y;
                stitches.push(Stitch::new(f64::from(cx) / 1000., f64::from(cy) / 1000.));
            },
            Vp3Stitch::Jump(x, y) => {
                if !stitches.is_empty() {
                    let old_stitches = stitches;
                    stitches = Vec::new();
                    stitch_groups.push(StitchGroup {
                        stitches: old_stitches,
                        trim: false,
                        cut: false,
                    });
                }
                cx += x;
                cy += y;
            },
            Vp3Stitch::Skip => {},
        }
    }
    if !stitches.is_empty() {
        stitch_groups.push(StitchGroup {
            stitches,
            trim: false,
            cut: false,
        });
    }
    read_exact_magic!(reader, [0x00_u8])?;
    let mut tmp = [0_u8; 16];
    match reader.read(&mut tmp) {
        Ok(0) => {},
        Ok(other) => panic!(
            "Failed to completely consume reader, {} bytes remaining: {:X?}",
            other,
            &tmp[..other]
        ),
        Err(err) => panic!("Failed to read: {:?}", err),
    }

    Ok(stitch_groups)
}

#[derive(Debug)]
enum Vp3Stitch {
    // Vp3Stitch distances in 1/1000mm
    Normal(i32, i32),
    Jump(i32, i32),
    Skip,
}

fn read_stitch(reader: &mut dyn Read) -> Result<(usize, Vp3Stitch), ReadError> {
    // Vp3Stitches are parsed in 1/10mm; we convert them to 1/1000mm
    let x = read_int!(reader, i8)?;
    let y = read_int!(reader, i8)?;
    if x == -0x80 {
        match y {
            0x00 => Ok((2, Vp3Stitch::Skip)),
            0x03 => Ok((2, Vp3Stitch::Skip)),
            0x01 => {
                let x = read_int!(reader, i16, BigEndian)?;
                let y = read_int!(reader, i16, BigEndian)?;
                match read_int!(reader, u16, BigEndian)? {
                    0x80_02 => Ok((8, Vp3Stitch::Jump(i32::from(x) * 100, i32::from(y) * 100))),
                    other => Err(ReadError::invalid_format(format!(
                        "Cannot parse jump stitch trailer value {:X}",
                        other
                    ))),
                }
            },
            other => Err(ReadError::invalid_format(format!(
                "Cannot parse special stitch with Y value {:X}",
                other
            ))),
        }
    } else {
        Ok((2, Vp3Stitch::Normal(i32::from(x) * 100, i32::from(y) * 100)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_thread_header_t42_1_vp3() {
        // T42-1.vp3 StartOffset(h): 000000E2, EndOffset(h): 00000141, Length(h): 00000060
        let mut data: &[u8] = &[
            0x00, 0x05, 0x00, 0x00, 0x00, 0x8B, 0x1B, /* Header starts here */ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xFF, 0x00, 0x00, 0x00, 0x05, 0x28, 0x00, 0x04, 0x31, 0x33, 0x37, 0x36,
            0x00, 0x0A, 0x53, 0x61, 0x6C, 0x65, 0x6D, 0x20, 0x42, 0x6C, 0x75, 0x65, 0x00, 0x10, 0x4D, 0x61, 0x64, 0x65,
            0x69, 0x72, 0x61, 0x20, 0x52, 0x61, 0x79, 0x6F, 0x6E, 0x20, 0x34, 0x30, 0x00, 0x00, 0x82, 0xDC, 0x00, 0x00,
            0x3C, 0x8C, 0x00, 0x01, 0x00, 0x00, 0x00, 0x8A, 0xD5, /* Header ends here */
            0x0A, 0xF6, 0x00, 0x80, 0x01, 0xFD, 0x26, 0xFD, 0x58, 0x80, 0x02, 0xF6, 0x0A, 0x05, 0xFB, 0x05, 0xFB, 0xFB,
            0x05, 0x05,
        ];

        let (take_reader, thread) = read_thread_header(&mut data).unwrap();
        assert_eq!(take_reader.limit(), 35611 - (6 + (18 * 3) + 9));
        assert_eq!(
            thread,
            Vp3ThreadHeader {
                thread_length: 35611, /* 0x00_00_8B_1B */
                x_offset_a: 0,
                y_offset_a: 0,
                color: (0, 0, 0),
                color_table: vec![[0xFF, 0x00, 0x00, 0x00, 0x05, 0x28]],
                thread_code: "1376".to_string(),
                thread_name: "Salem Blue".to_string(),
                thread_manufacturer: "Madeira Rayon 40".to_string(),
                x_offset_b: 33500,
                y_offset_b: 15500,
                stitch_bytes: 35541, /* 0x00_00_8A_D5 */
            }
        );
    }

    #[test]
    fn test_read_thread_header_t160_thread_1_vp3() {
        // T160.vp3 StartOffset(h): 000000E2, EndOffset(h): 00000141, Length(h): 00000060
        let data: [u8; 96] = [
            0x00, 0x05, 0x00, 0x00, 0x00, 0x1C, 0xBB, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0xDE,
            0xE6, 0xE8, 0x00, 0x00, 0x00, 0x05, 0x28, 0x00, 0x04, 0x31, 0x30, 0x30, 0x33, 0x00, 0x0E, 0x41, 0x6D, 0x65,
            0x74, 0x68, 0x79, 0x73, 0x74, 0x20, 0x4C, 0x69, 0x67, 0x68, 0x74, 0x00, 0x10, 0x4D, 0x61, 0x64, 0x65, 0x69,
            0x72, 0x61, 0x20, 0x52, 0x61, 0x79, 0x6F, 0x6E, 0x20, 0x34, 0x30, 0x00, 0x00, 0xB5, 0xA4, 0x00, 0x01, 0x2E,
            0xBC, 0x00, 0x01, 0x00, 0x00, 0x00, 0x1C, 0x71, 0x0A, 0xF6, 0x00, 0x80, 0x01, 0xFE, 0x7A, 0xFD, 0x4E, 0x80,
            0x02, 0xF6, 0x0A, 0x05, 0xFB, 0x05,
        ];

        let (_, thread) = read_thread_header(&mut &data[..]).unwrap();
        assert_eq!(
            thread,
            Vp3ThreadHeader {
                thread_length: 7355, /* 0x00_00_1C_BB */
                x_offset_a: 0,
                y_offset_a: 0,
                color: (0x00, 0xDE, 0xE6),
                color_table: vec![[0xE8, 0x00, 0x00, 0x00, 0x05, 0x28,]],
                thread_code: "1003".to_string(),
                thread_name: "Amethyst Light".to_string(),
                thread_manufacturer: "Madeira Rayon 40".to_string(),
                x_offset_b: 46500,
                y_offset_b: 77500,
                stitch_bytes: 7281, /* 0x00_00_1C_71 */
            }
        );
    }
}
