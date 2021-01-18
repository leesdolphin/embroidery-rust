use byteorder::BigEndian;

use embroidery_lib::prelude::*;
use embroidery_lib::{read_exact_magic, read_int};

use std::convert::TryInto;
use std::io::Read;

use crate::common::util::read_wide_string_field;

#[derive(Debug, PartialEq)]
pub struct Vp3Header {
    pub file_comment_string: String,
    pub hoop: Vp3Hoop,
    pub another_software_vendor_string: String,
    pub number_of_threads: usize,
}
#[derive(Debug, PartialEq)]
pub struct Vp3Hoop {
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
    pub top: i32,
    pub unknown_a: u32,
    pub unknown_b: u16,
    pub bytes_remaining: usize,
    pub x_offset: i32,
    pub y_offset: i32,

    /* Centered hoop dimensions */
    pub right2: i32,
    pub left2: i32,
    pub bottom2: i32,
    pub top2: i32,

    pub width: i32,
    pub height: i32,
}

pub fn read_pattern_header(mut reader: &mut dyn Read) -> Result<Vp3Header, ReadError> {
    let file_comment_string = read_wide_string_field(&mut reader, "file_comment_string")?;

    let hoop = read_hoop(&mut reader)?;

    read_exact_magic!(
        reader,
        [
            0x00, 0x00, 0x64, 0x64, 0x00, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x10, 0x00
        ]
    )?;

    // This is noted as [0x78, 0x78, 0x55, 0x55, 0x01, 0x00] in Embroidermodder; but testing
    // reveals it to be [0x78, 0x78, 0x50, 0x50, 0x01, 0x00]
    read_exact_magic!(reader, [0x78, 0x78, 0x50, 0x50, 0x01, 0x00])?;

    let another_software_vendor_string = read_wide_string_field(&mut reader, "another_software_vendor_string")?;

    let number_of_threads: usize = read_int!(reader, u16, BigEndian)?.into();

    Ok(Vp3Header {
        file_comment_string,
        hoop,
        another_software_vendor_string,
        number_of_threads,
    })
}

#[allow(clippy::cognitive_complexity)]
fn read_hoop(reader: &mut dyn Read) -> Result<Vp3Hoop, ReadError> {
    let left = read_int!(reader, i32, BigEndian)?;
    let top = read_int!(reader, i32, BigEndian)?;
    let right = read_int!(reader, i32, BigEndian)?;
    let bottom = read_int!(reader, i32, BigEndian)?;

    // Probably number of stitches
    let unknown_a = read_int!(reader, u32, BigEndian)?;
    // Probably number of colors(read: threads)
    let unknown_b = read_int!(reader, u16, BigEndian)?;

    read_exact_magic!(reader, [0x0C, 0x00, 0x01, 0x00, 0x03, 0x00])?;

    let bytes_remaining = read_int!(reader, u32, BigEndian)?;

    let y_offset = read_int!(reader, i32, BigEndian)?;
    let x_offset = read_int!(reader, i32, BigEndian)?;

    read_exact_magic!(reader, [0x00, 0x00, 0x00])?;

    /* Centered hoop dimensions */
    let right2 = read_int!(reader, i32, BigEndian)?;
    let left2 = read_int!(reader, i32, BigEndian)?;
    let bottom2 = read_int!(reader, i32, BigEndian)?;
    let top2 = read_int!(reader, i32, BigEndian)?;

    let width = read_int!(reader, i32, BigEndian)?;
    let height = read_int!(reader, i32, BigEndian)?;

    Ok(Vp3Hoop {
        right,
        bottom,
        left,
        top,
        unknown_a,
        unknown_b,
        bytes_remaining: bytes_remaining.try_into().unwrap(),
        x_offset,
        y_offset,
        right2,
        left2,
        bottom2,
        top2,
        width,
        height,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_hoop_t160_vp3() {
        // T160.vp3 StartOffset(h): 00000049, EndOffset(h): 0000008B, Length(h): 00000043
        let data: [u8; 67] = [
            0x00, 0x00, 0xF2, 0x30, 0x00, 0x01, 0x4F, 0xF0, 0xFF, 0xFF, 0x0D, 0xD0, 0xFF, 0xFE, 0xB0, 0x10, 0x00, 0x00,
            0x69, 0xB5, 0x00, 0x08, 0x0C, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0xD8, 0x1F, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x0D, 0xD0, 0x00, 0x00, 0xF2, 0x30, 0xFF, 0xFE, 0xB0,
            0x10, 0x00, 0x01, 0x4F, 0xF0, 0x00, 0x01, 0xE4, 0x60, 0x00, 0x02, 0x9F, 0xE0,
        ];
        let hoop = read_hoop(&mut &data[..]).unwrap();
        assert_eq!(
            hoop,
            Vp3Hoop {
                right: -62_000,
                left: 62_000,
                bottom: -86_000,
                top: 86_000,

                unknown_a: 27061,
                unknown_b: 8,
                bytes_remaining: 55327,

                x_offset: 0,
                y_offset: 0,

                right2: -62_000,
                left2: 62_000,
                bottom2: -86_000,
                top2: 86_000,
                width: 124_000,
                height: 172_000,
            }
        );
    }

    #[test]
    fn test_read_hoop_t42_1_vp3() {
        // T42-1.vp3 StartOffset(h): 00000049, EndOffset(h): 0000008B, Length(h): 00000043
        let data: [u8; 67] = [
            0x00, 0x01, 0x4F, 0xF0, 0x00, 0x01, 0x3C, 0x68, 0xFF, 0xFE, 0xB0, 0x10, 0xFF, 0xFE, 0xC3, 0x98, 0x00, 0x00,
            0x45, 0x71, 0x00, 0x01, 0x0C, 0x00, 0x01, 0x00, 0x03, 0x00, 0x00, 0x00, 0x8B, 0x9B, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFE, 0xB0, 0x10, 0x00, 0x01, 0x4F, 0xF0, 0xFF, 0xFE, 0xC3,
            0x98, 0x00, 0x01, 0x3C, 0x68, 0x00, 0x02, 0x9F, 0xE0, 0x00, 0x02, 0x78, 0xD0,
        ];
        let hoop = read_hoop(&mut &data[..]).unwrap();

        assert_eq!(
            hoop,
            Vp3Hoop {
                right: -86_000,
                left: 86_000,
                bottom: -81_000,
                top: 81_000,

                unknown_a: 17777,
                unknown_b: 1,
                bytes_remaining: 35739,

                x_offset: 0,
                y_offset: 0,

                right2: -86_000,
                left2: 86_000,
                bottom2: -81_000,
                top2: 81_000,
                width: 172_000,
                height: 162_000,
            }
        )
    }
}
