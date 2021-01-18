use std::io::{Bytes, Error, Read};

pub struct ReadByteIterator<T: Read + Sized> {
    reader: Bytes<T>,
    pub closed: bool,
    pub error: Option<Error>,
}

impl<T: Read + Sized> ReadByteIterator<T> {
    pub fn new(reader: T) -> Self
    where
        T: Read + Sized,
    {
        Self {
            reader: reader.bytes(),
            closed: false,
            error: None,
        }
    }
    pub fn close(self: &mut Self) {
        self.closed = true
    }
}

pub fn format_u8_array(arr: &[u8]) -> String {
    return format!(
        "[{}]",
        arr.iter().fold("".to_string(), |acc, i| {
            let sep = if acc != "" { ", " } else { "" };
            format!("{}{}0x{:02X}", acc, sep, i)
        })
    );
}

impl<T: Read + Sized> Iterator for ReadByteIterator<T> {
    type Item = u8;
    fn next(self: &mut Self) -> Option<<Self as Iterator>::Item> {
        if self.closed {
            None
        } else {
            match self.reader.next() {
                Some(Ok(value)) => Some(value),
                Some(Err(error)) => {
                    self.error = Some(error);
                    self.close();
                    None
                },
                None => {
                    self.close();
                    None
                },
            }
        }
    }
}

#[macro_export]
macro_rules! read_exact_magic {
    ($reader: expr, $expected: expr) => {{
        let expected: &[u8] = &$expected[..];
        // Make real the same type as expected; hopefully an array.
        let mut real = vec![0_u8; expected.len()];

        match $crate::maybe_read_with_context!(
            $crate::read_exact!($reader, &mut real),
            "Trying to read magic bytes {:?}",
            $crate::utils::format_u8_array(expected)
        ) {
            Err(err) => Err($crate::errors::ReadError::from(err)),
            Ok(()) => {
                if &real[..] != expected {
                    Err($crate::errors::ReadError::invalid_format(format!(
                        "Magic bytes do not match. Expected {:02X?}; got {:02X?} at {}:{}",
                        $crate::utils::format_u8_array(expected),
                        $crate::utils::format_u8_array(&real),
                        file!(),
                        line!()
                    )))
                } else {
                    // Return the read value in case the caller wants it.
                    Ok(real)
                }
            },
        }
    }};
}

#[macro_export]
macro_rules! maybe_read_with_context {
    ($maybe_error: expr, $($arg: tt)+) => {{
        use $crate::errors::ErrorWithContext;

        match $maybe_error {
            Ok(x) => Ok(x),
            Err(err) => Err(
                $crate::errors::ReadError::from(err).with_additional_context(
                    format!("{} at {}:{}", format!($($arg)+), file!(), line!()),
                )
            ),
        }
    }};
    ($maybe_error: expr) => {{
        use $crate::errors::ErrorWithContext;

        match $maybe_error {
            Ok(x) => Ok(x),
            Err(err) => Err($crate::errors::ReadError::from(err).with_additional_context(
                format!("... at {}:{}", file!(), line!()),
            )),
        }
    }};
}

#[macro_export]
macro_rules! __emb_lib_handle_eof {
    ($result: expr, $err_str: expr, $($arg : tt)*) => {
        match $result {
           Ok(x) => Ok(x),
           Err(e) => {
               if e.kind() == std::io::ErrorKind::UnexpectedEof {
                   Err($crate::errors::ReadError::unexpected_eof(
                       format!("{} at {}:{}", format!($err_str, $($arg)*), file!(), line!()),
                       e,
                   ))
               } else {
                   Err(e.into())
               }
           },
       }
    }
}

#[macro_export]
macro_rules! read_exact {
    ($reader: expr, vec![_; $length: expr]) => {{
        let mut target = vec![0_u8; $length];
        match $crate::read_exact!($reader, &mut target[..]) {
            Ok(()) => Ok(target),
            Err(e) => Err(e),
        }
    }};
    ($reader: expr, [_; $length: expr]) => {{
        let mut target = [0_u8; $length];
        match $crate::read_exact!($reader, &mut target[..]) {
            Ok(()) => Ok(target),
            Err(e) => Err(e),
        }
    }};
    ($reader: expr, $target: expr) => {{
        let real = $target;
        $crate::__emb_lib_handle_eof!(
            $reader.read_exact(real),
            "Failed to read exactly `{}` bytes from file(`{}`) into `{}`",
            real.len(),
            stringify!($reader),
            stringify!($target)
        )
    }};
}

#[macro_export]
macro_rules! read_int {
    ($reader: expr, u8) => {
        $crate::__emb_lib_read_int_impl!($reader, u8, read_u8)
    };
    ($reader: expr, i8) => {
        $crate::__emb_lib_read_int_impl!($reader, i8, read_i8)
    };
    ($reader: expr, u8, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u8, read_u8)
    };
    ($reader: expr, i8, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i8, read_i8)
    };

    ($reader: expr, u16, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u16, read_u16::<$endianness>)
    };
    ($reader: expr, u32, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u32, read_u32::<$endianness>)
    };
    ($reader: expr, u64, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u64, read_u64::<$endianness>)
    };
    ($reader: expr, u128, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, u128, read_u128::<$endianness>)
    };
    ($reader: expr, i16, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i16, read_i16::<$endianness>)
    };
    ($reader: expr, i32, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i32, read_i32::<$endianness>)
    };
    ($reader: expr, i64, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i64, read_i64::<$endianness>)
    };
    ($reader: expr, i128, $endianness: path) => {
        $crate::__emb_lib_read_int_impl!($reader, i128, read_i128::<$endianness>)
    };
}

#[macro_export]
macro_rules! __emb_lib_read_int_impl {
    ($reader: expr, $type: ident, $($fn: tt)+) => {{
        use byteorder::ReadBytesExt;

        $crate::__emb_lib_handle_eof!(
            $reader. $($fn)+ (),
            "Failed to read `{}` from file(var name `{}`)",
            stringify!($type),
            stringify!($reader)
       )
    }}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_exact_magic() {
        let mut reader = &([0x00_u8, 0x01, 0x02, 0x03])[..];
        assert_eq!(read_exact_magic!(&mut reader, [0, 1]).unwrap(), [0, 1]);

        let err_str = match read_exact_magic!(&mut reader, [0x99, 0x98]).unwrap_err() {
            crate::errors::ReadError::InvalidFormat(err_str, _) => err_str,
            other => panic!("Unexpected error type: {:?}", other),
        };
        assert!(
            err_str.find(file!()).is_some(),
            "Cannot find file {:?} in error string {:?}",
            file!(),
            err_str
        );
        assert!(
            err_str.find("[0x99, 0x98]").is_some(),
            "Cannot find expected value in error string {:?}",
            err_str
        );
        assert!(
            err_str.find("[0x02, 0x03]").is_some(),
            "Cannot find actual value in error string {:?}",
            err_str
        );
        let (err_str, ctx) = match read_exact_magic!(&mut reader, [0x50, 0x51]).unwrap_err() {
            crate::errors::ReadError::UnexpectedEof(err_str, _, ctx) => (err_str, ctx),
            other => panic!("Unexpected error type: {:?}", other),
        };
        assert!(
            err_str.find(file!()).is_some(),
            "Cannot find file {:?} in error string {:?}",
            file!(),
            err_str
        );
        assert!(
            ctx[0].find("[0x50, 0x51]").is_some(),
            "Cannot find expected value in error string {:?}",
            err_str
        );
    }
    #[test]
    fn test_read_exact_magic_long() {
        let mut reader = &([
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29,
            30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56,
            57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83,
            84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
        ])[..];

        // Check that we support arrays of more than 32 items.
        assert_eq!(
            read_exact_magic!(
                &mut reader,
                [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
                    28, 29, 30, 31, 32, 33
                ]
            )
            .unwrap(),
            &[
                1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
                29, 30, 31, 32, 33
            ][..]
        );
    }

    #[test]
    fn test_read_exact() {
        let mut reader = &([0_u8, 1, 2])[..];
        let mut target = [0_u8; 2];
        read_exact!(&mut reader, &mut target).unwrap();

        let err_str = match read_exact!(&mut reader, &mut target).unwrap_err() {
            crate::errors::ReadError::UnexpectedEof(err_str, _, _) => err_str,
            other => panic!("Unexpected error type: {:?}", other),
        };
        assert!(
            err_str.find(file!()).is_some(),
            "Cannot find file {:?} in error string {:?}",
            file!(),
            err_str
        );
        assert!(
            err_str.find("`2` bytes").is_some(),
            "Cannot find expected read amount in error string {:?}",
            err_str
        );
        assert!(
            err_str.find("&mut target").is_some(),
            "Cannot find target variable in error string {:?}",
            err_str
        );
    }
    #[test]
    fn test_read_exact_slice() {
        let mut reader = &([0_u8, 1, 2])[..];
        assert_eq!(read_exact!(&mut reader, [_; 2]).unwrap(), [0, 1]);
    }
    #[test]
    fn test_read_exact_vec() {
        let mut reader = &([0_u8, 1, 2])[..];
        let len = 2;
        assert_eq!(read_exact!(&mut reader, vec![_; len]).unwrap(), vec![0, 1]);
    }
    #[test]
    #[allow(clippy::cognitive_complexity)]
    fn test_read_int() {
        use byteorder::BigEndian;

        // Should need (31 * 2) for the fixed sized integers and at most (16*2) for the *size integers
        // Which is 94; but use 200 just to be safe
        let mut reader = &([0_u8; 200])[..];

        assert_eq!(read_int!(&mut reader, u8).unwrap(), 0_u8);
        assert_eq!(read_int!(&mut reader, i8).unwrap(), 0_i8);

        assert_eq!(read_int!(&mut reader, u16, BigEndian).unwrap(), 0_u16);
        assert_eq!(read_int!(&mut reader, u32, BigEndian).unwrap(), 0_u32);
        assert_eq!(read_int!(&mut reader, u64, BigEndian).unwrap(), 0_u64);
        assert_eq!(read_int!(&mut reader, u128, BigEndian).unwrap(), 0_u128);
        assert_eq!(read_int!(&mut reader, i16, BigEndian).unwrap(), 0_i16);
        assert_eq!(read_int!(&mut reader, i32, BigEndian).unwrap(), 0_i32);
        assert_eq!(read_int!(&mut reader, i64, BigEndian).unwrap(), 0_i64);
        assert_eq!(read_int!(&mut reader, i128, BigEndian).unwrap(), 0_i128);
    }
}
