mod error;
mod formats;

#[macro_use]
pub extern crate failure;
#[macro_use]
pub extern crate log;

use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Seek;
use std::path::Path;

use simplelog::*;

use embroidery_lib::{
    format,
    prelude::{ReadError, WriteError},
    Pattern,
};

use crate::error::Error;
use crate::formats::get_all;
use crate::formats::PatternFormat;

fn write_pattern(
    path: &Path,
    extras: Option<String>,
    pattern: Pattern,
    formats: &[PatternFormat],
) -> Result<(), Error> {
    let base_file_name = path.file_name().ok_or("Path must have an filename")?.to_string_lossy();

    let file_name = if let Some(extra) = extras {
        format!("{}-{}", base_file_name, extra)
    } else {
        base_file_name.into_owned()
    };

    for format in formats.iter() {
        let extensions = format.extensions();

        assert!(!extensions.is_empty());
        //     if let Some(writer) = format.pattern_writer() {
        //         let ext = extensions[0];
        //         let output = path.with_file_name(format!("{}.{}", file_name, ext));
        //         let mut out = BufWriter::new(
        //             File::create(&output).map_err(|e| format!("Error writing to {}: {:?}", output.to_string_lossy(), e))?,
        //         );
        //         match writer.write_pattern(&pattern, &mut out) {
        //             Ok(()) => {}
        //             Err(WriteError::UnsupportedStitch {
        //                 stitch, idx: maybe_idx, ..
        //             }) => {
        //                 let idx = match maybe_idx {
        //                     Some(idx) => format!("{}", idx),
        //                     None => ("unknown").to_string(),
        //                 };
        //                 return Err(format!(
        //                     "Writer {} cannot write one of the stitches {:?}(at {}): {}",
        //                     ext, stitch, idx, file_name,
        //                 )
        //                 .into());
        //             }
        //             Err(WriteError::Std(err, _)) => return Err(err.into()),
        //         }
        //     }
    }
    Ok(())
}

fn load_pattern_and_write(
    path: &Path,
    format: &Box<dyn format::PatternFormat>,
    loader: &Box<dyn format::PatternReader>,
    formats: &[PatternFormat],
) -> Result<bool, Error> {
    let file_name = path.file_name().ok_or("Path must have an filename")?.to_string_lossy();

    let mut reader = BufReader::new(File::open(path)?);
    reader.seek(std::io::SeekFrom::Start(0))?;
    match loader.read_pattern(&mut reader) {
        Ok(p) => {
            write_pattern(path, None, p, formats)?;

            Ok(true)
        }
        Err(ReadError::InvalidFormat(msg, ctx)) => {
            warn!(
                "Format {} cannot parse file {}. Reason: {}\n Context: {:#?}",
                format.name(),
                file_name,
                msg,
                ctx
            );
            Ok(false)
        }
        Err(err) => Err(err.into()),
    }
}

fn load_collection_and_write(
    path: &Path,
    format: &Box<dyn format::CollectionFormat>,
    loader: &Box<dyn format::CollectionReader>,
    formats: &[PatternFormat],
) -> Result<bool, Error> {
    let file_name = path.file_name().ok_or("Path must have an filename")?.to_string_lossy();

    let mut reader = BufReader::new(File::open(path)?);
    reader.seek(std::io::SeekFrom::Start(0))?;
    match loader.read_pattern(&mut reader) {
        Ok(p) => {
            for (name, pattern) in p.patterns {
                let clean_name: String = name
                    .chars()
                    .map(|chr| match chr {
                        '/' => '_',
                        '\\' => '_',
                        '?' => '_',
                        chr if chr.is_ascii_control() => '_',
                        chr => chr,
                    })
                    .collect();
                let bytes: String = name.bytes().map(|byte| format!("{:02X}", byte)).collect();

                write_pattern(path, Some(format!("{}-{}", clean_name, bytes)), pattern, formats)?;
            }
            Ok(true)
        }
        Err(ReadError::InvalidFormat(msg, ctx)) => {
            warn!(
                "Format {} cannot parse file {}. Reason: {}\n Context: {:#?}",
                format.name(),
                file_name,
                msg,
                ctx
            );
            Ok(false)
        }
        Err(err) => Err(err.into()),
    }
}

fn load_and_write(path: &Path, fmt: &PatternFormat, formats: &[PatternFormat]) -> Result<bool, Error> {
    match fmt {
        PatternFormat::Pattern(fmt) => {
            if let Some(reader) = fmt.reader() {
                load_pattern_and_write(path, fmt, &reader, formats)
            } else {
                Ok(false)
            }
        }
        PatternFormat::Collection(fmt) => {
            if let Some(reader) = fmt.reader() {
                load_collection_and_write(path, fmt, &reader, formats)
            } else {
                Ok(false)
            }
        }
    }
}

fn main() -> Result<(), Error> {
    TermLogger::init(
        LevelFilter::Debug,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .set_target_level(LevelFilter::Off)
            .set_location_level(LevelFilter::Error)
            .build(),
        TerminalMode::Mixed,
    )?;

    let pattern_formats = get_all();

    for file in env::args().skip(1) {
        let path = Path::new(&file);
        warn!("Parsing {}", path.to_string_lossy());
        let extension = path
            .extension()
            .ok_or("Path must have an filename")?
            .to_string_lossy()
            .into_owned();
        for fmt in pattern_formats.iter() {
            let read = load_and_write(path, fmt, &pattern_formats)?;
            if read {
                let fmt_exts = fmt.extensions();
                if !fmt_exts.contains(&&extension.as_ref()) {
                    warn!(
                        "The format {} managed to read {}; but it's extension was not listed in the formats extension list.\n  Extension: {:?}; Supported Extension list: {:?}",
                        fmt.name(), path.to_string_lossy(), extension, fmt_exts
                    );
                }
            }
        }
    }
    Ok(())
}
