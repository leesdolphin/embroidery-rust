use std::io::Read;

use embroidery_lib::format::CollectionReader;
use embroidery_lib::prelude::*;

use crate::common::header::{read_header, FileType, Header};
use crate::vf3::read::pattern::read_font_pattern;

mod pattern;

pub struct Vf3CollectionReader {}

impl Default for Vf3CollectionReader {
    fn default() -> Self {
        Vf3CollectionReader {}
    }
}

impl CollectionReader for Vf3CollectionReader {
    fn is_loadable(&self, file: &mut dyn Read) -> Result<bool, ReadError> {
        // Load the header
        match read_header(file, Some(FileType::Font)) {
            Ok(_) => Ok(true),
            Err(ReadError::InvalidFormat(_, _)) => Ok(false),
            Err(ReadError::UnexpectedEof(_, _, _)) => Ok(false),
            Err(other) => Err(other),
        }
    }

    fn read_pattern(&self, file: &mut dyn Read) -> Result<PatternCollection, ReadError> {
        let (_common_header, header, _) = read_header(file, Some(FileType::Font))?;
        let header = match header {
            Header::Font(font_header) => font_header,
            _ => unreachable!(),
        };
        let patterns = read_font_pattern(file, &header.character_offsets)?;
        // todo!();
        Ok(PatternCollection {
            attributes: vec![],
            patterns,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_parsable() {
        // Less than 3 bytes means the iterator has been exhausted.
        let b = b"ab";
        assert_eq!(Vf3CollectionReader::default().is_loadable(&mut &b[..]).unwrap(), false);
    }
}
