use byteorder::{LittleEndian, WriteBytesExt};
use embroidery_lib::errors::{ReadError, ReadResult};
use embroidery_lib::read_int;
use embroidery_lib::utils::c_trim;
use std::io::{Read, Result, Write};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PatternType {
    // Magic bytes: [0x5B, 0xAF, 0xC8, 0x00]
    Hus,
    // Magic bytes: [0x5d ,0xfc 0x90 ,0x01]
    Vip,
}

impl PatternType {
    pub fn magic_bytes(self) -> [u8; 4] {
        match self {
            PatternType::Hus => [0x5B, 0xAF, 0xC8, 0x00],
            PatternType::Vip => [0x5D, 0xFC, 0x90, 0x01],
        }
    }
    pub fn match_magic_bytes(bytes: [u8; 4]) -> Option<Self> {
        if bytes == [0x5B, 0xAF, 0xC8, 0x00] || bytes == [0x5D, 0xFC, 0xC8, 0x00] {
            Some(PatternType::Hus)
        } else if bytes == [0x5D, 0xFC, 0x90, 0x01] {
            Some(PatternType::Vip)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[allow(clippy::module_name_repetitions)]
pub struct PatternHeader {
    pub pattern_type: PatternType,
    pub title: String,
    pub number_of_stitches: u32,
    pub number_of_colors: u32,
    pub postitive_x_hoop_size: i16,
    pub postitive_y_hoop_size: i16,
    pub negative_x_hoop_size: i16,
    pub negative_y_hoop_size: i16,
    pub attribute_offset: u32,
    pub x_offset: u32,
    pub y_offset: u32,
    // [0x00; 10]
}

impl PatternHeader {
    pub fn build(file: &mut dyn Read) -> ReadResult<Self> {
        let pattern_type = {
            let mut magic_code = [0; 4];
            file.read_exact(&mut magic_code)?;
            if let Some(t) = PatternType::match_magic_bytes(magic_code) {
                t
            } else {
                return Err(ReadError::invalid_format(format!(
                    "Invalid magic bytes [{:X}, {:X}, {:X}, {:X}]",
                    magic_code[0], magic_code[1], magic_code[2], magic_code[3]
                )));
            }
        };

        let number_of_stitches = read_int!(file, u32, LittleEndian)?;
        let number_of_colors = read_int!(file, u32, LittleEndian)?;

        let postitive_x_hoop_size = read_int!(file, i16, LittleEndian)?;
        let postitive_y_hoop_size = read_int!(file, i16, LittleEndian)?;
        let negative_x_hoop_size = read_int!(file, i16, LittleEndian)?;
        let negative_y_hoop_size = read_int!(file, i16, LittleEndian)?;

        let attribute_offset = read_int!(file, u32, LittleEndian)?;
        let x_offset = read_int!(file, u32, LittleEndian)?;
        let y_offset = read_int!(file, u32, LittleEndian)?;

        let title = {
            let mut title = [0; 10];
            file.read_exact(&mut title)?;
            c_trim(&String::from_utf8_lossy(&title))
        };
        if pattern_type == PatternType::Vip {
            // Maybe the color length; but can sometimes be wildly inaccurate; so we'll just
            // consume it and ignore the result.
            let _ = read_int!(file, u32, LittleEndian)?;
        }

        Ok(Self {
            pattern_type,
            title,
            number_of_stitches,
            number_of_colors,
            postitive_x_hoop_size,
            postitive_y_hoop_size,
            negative_x_hoop_size,
            negative_y_hoop_size,
            attribute_offset,
            x_offset,
            y_offset,
        })
    }

    pub fn header_len(&self) -> usize {
        // Magic bytes + #stitches + #colors + [+ve x] + [+ve y] + [-ve x] + [-ve y]
        //  + [attr offset] + [x offset] + [y offset] + [10 bytes]
        let header = 4 + (2 * 4) + (4 * 2) + (3 * 4) + 10;
        let header_color_extra = match self.pattern_type {
            PatternType::Hus => 0,
            PatternType::Vip => 4,
        };
        header + header_color_extra
    }
    pub fn color_len(&self) -> usize {
        match self.pattern_type {
            PatternType::Hus => (self.number_of_colors as usize) * 2,
            PatternType::Vip => (self.number_of_colors as usize) * 4,
        }
    }
    pub fn color_consume_len(&self) -> usize {
        (self.attribute_offset as usize) - self.header_len()
    }

    pub fn attribute_len(&self) -> usize {
        (self.x_offset as usize) - (self.attribute_offset as usize)
    }
    pub fn x_offset_len(&self) -> usize {
        (self.y_offset as usize) - (self.x_offset as usize)
    }

    #[allow(dead_code)]
    pub fn write(&self, file: &mut dyn Write) -> Result<()> {
        file.write_all(&self.pattern_type.magic_bytes())?;
        file.write_u32::<LittleEndian>(self.number_of_stitches)?;
        file.write_u32::<LittleEndian>(self.number_of_colors)?;
        file.write_i16::<LittleEndian>(self.postitive_x_hoop_size)?;
        file.write_i16::<LittleEndian>(self.postitive_y_hoop_size)?;
        file.write_i16::<LittleEndian>(self.negative_x_hoop_size)?;
        file.write_i16::<LittleEndian>(self.negative_y_hoop_size)?;
        file.write_u32::<LittleEndian>(self.attribute_offset)?;
        file.write_u32::<LittleEndian>(self.x_offset)?;
        file.write_u32::<LittleEndian>(self.y_offset)?;

        file.write_all(&[0x00; 10])?;

        if self.pattern_type == PatternType::Vip {
            // This was derrived from a number of files; Don't understand why though.
            file.write_u32::<LittleEndian>(0x2E + 8 * self.number_of_colors)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_header_roundtrip() {
        let data = [
            0x5d, 0xfc, 0x90, 0x01, 0x78, 0x03, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0xb3, 0x00, 0xb5, 0x00, 0x4d, 0xff,
            0x4c, 0xff, 0x4e, 0x00, 0x00, 0x00, 0x6b, 0x00, 0x00, 0x00, 0x8b, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x36, 0x00, 0x00, 0x00,
        ];
        let header = PatternHeader::build(&mut Cursor::new(&data[..])).unwrap();
        assert_eq!(header.pattern_type, PatternType::Vip);
        assert_eq!(header.number_of_stitches, 0x00_00_03_78);
        assert_eq!(header.number_of_colors, 0x00_00_00_01);
        assert_eq!(header.postitive_x_hoop_size, 0x00_b3);
        assert_eq!(header.postitive_y_hoop_size, 0x00_b5);
        assert_eq!(header.negative_x_hoop_size, 0x4d - 0x100);
        assert_eq!(header.negative_y_hoop_size, 0x4c - 0x100);
        assert_eq!(header.attribute_offset, 0x00_00_00_4e);
        assert_eq!(header.x_offset, 0x00_00_00_6b);
        assert_eq!(header.y_offset, 0x00_00_02_8b);
        // Skip 10 bytes
        // Skip 4 bytes
        let mut out = Vec::with_capacity(data.len());
        header.write(&mut Cursor::new(&mut out)).unwrap();
        assert_eq!(&data[..], &out[..])
    }
}
