use embroidery_lib::format::PatternReader;
use embroidery_lib::maybe_read_with_context;
use embroidery_lib::prelude::*;

use std::io::Read;

use crate::common::{header, thread};

#[derive(Default)]
pub struct Vp3PatternReader {}

impl PatternReader for Vp3PatternReader {
    fn is_loadable(&self, reader: &mut dyn Read) -> Result<bool, ReadError> {
        header::read_header(reader, Some(header::FileType::Pattern))?;
        Ok(false)
    }
    fn read_pattern(&self, ub_reader: &mut dyn Read) -> Result<Pattern, ReadError> {
        let (_common_header, header, mut reader) = header::read_header(ub_reader, Some(header::FileType::Pattern))?;
        let header = match header {
            header::Header::Pattern(head) => head,
            _ => unreachable!(),
        };
        let cgs = thread::read_threads(&mut reader, header.number_of_threads)?;
        Ok(Pattern {
            attributes: vec![],
            color_groups: cgs,
            name: "".to_string(),
        })
    }
}
