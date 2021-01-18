use byteorder::BigEndian;

use embroidery_lib::errors::add_context;
use embroidery_lib::prelude::*;
use embroidery_lib::{maybe_read_with_context, read_exact, read_int};

use std::io::Read;

pub fn read_wide_string_field(reader: &mut dyn Read, name: &str) -> Result<String, ReadError> {
    let len: usize = read_int!(reader, u16, BigEndian)?.into();
    if len % 2 != 0 {
        return Err(ReadError::invalid_format(format!(
            "Incorrect length for {}. Expected an even value, got {:?}",
            name, len
        )));
    }
    let mut utf16be_codepoints = vec![0_u16; len / 2];
    for (i, v) in utf16be_codepoints.iter_mut().enumerate() {
        *v = add_context(read_int!(reader, u16, BigEndian), || {
            format!(
                "Unable to completely read string for {}. Got to character {} of {}",
                name,
                i,
                len / 2
            )
        })?;
    }
    Ok(String::from_utf16_lossy(&utf16be_codepoints))
}
pub fn read_ascii_string_field(reader: &mut dyn Read, name: &str) -> Result<String, ReadError> {
    let len: usize = read_int!(reader, u16, BigEndian)?.into();
    let utf8_codepoints = maybe_read_with_context!(
        read_exact!(reader, vec![_; len]),
        "Attempting to read {} as an ASCII string of length {:X}",
        name,
        len
    )?;
    Ok(String::from_utf8_lossy(&utf8_codepoints).to_string())
}

pub fn read_short_ascii_string_field(reader: &mut dyn Read, name: &str) -> Result<String, ReadError> {
    let len: usize = read_int!(reader, u8)?.into();
    let utf8_codepoints = maybe_read_with_context!(
        read_exact!(reader, vec![_; len]),
        "Attempting to read {} as an ASCII string of length {:X}",
        name,
        len
    )?;
    Ok(String::from_utf8_lossy(&utf8_codepoints).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_wide_string_field_empty() {
        let file = b"\0\0";
        assert_eq!(read_wide_string_field(&mut &file[..], "").unwrap(), "".to_string());
    }
    #[test]
    fn test_read_wide_string_field_odd() {
        let file = [0_u8, 1];
        read_wide_string_field(&mut &file[..], "").unwrap_err();
    }
    #[test]
    fn test_read_wide_string_field_too_short() {
        let file = [0_u8, 2];
        read_wide_string_field(&mut &file[..], "").unwrap_err();
    }
    #[test]
    fn test_read_wide_string_field_something() {
        // The first 2 bytes are [0, 10];
        let file = b"\0\x0A\0h\0e\0l\0l\0o!!";
        assert_eq!(read_wide_string_field(&mut &file[..], "").unwrap(), "hello".to_string());
    }
}
