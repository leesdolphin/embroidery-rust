use embroidery_lib::format::{self, CollectionReader, CollectionWriter, PatternReader, PatternWriter};

use embroidery_fmt_csv::CsvPatternFormat;
use embroidery_fmt_dst::DstPatternFormat;
use embroidery_fmt_hus::{HusPatternFormat, VipPatternFormat};
use embroidery_fmt_svg::SvgPatternFormat;
use embroidery_fmt_vp3::{Vf3CollectionFormat, Vp3PatternFormat};

pub enum PatternFormat {
    Pattern(Box<dyn format::PatternFormat>),
    Collection(Box<dyn format::CollectionFormat>),
}

impl PatternFormat {
    pub fn extensions(&self) -> &[&str] {
        match self {
            PatternFormat::Pattern(fmt) => fmt.extensions(),
            PatternFormat::Collection(fmt) => fmt.extensions(),
        }
    }
    pub fn name(&self) -> &str {
        match self {
            PatternFormat::Pattern(fmt) => fmt.name(),
            PatternFormat::Collection(fmt) => fmt.name(),
        }
    }

    pub fn pattern(&self) -> Option<&Box<dyn format::PatternFormat>> {
        match self {
            PatternFormat::Pattern(p) => Some(p),
            PatternFormat::Collection(_) => None,
        }
    }
    pub fn collection(&self) -> Option<&Box<dyn format::CollectionFormat>> {
        match self {
            PatternFormat::Pattern(_) => None,
            PatternFormat::Collection(p) => Some(p),
        }
    }

    pub fn pattern_reader(&self) -> Option<Box<dyn PatternReader>> {
        match self {
            PatternFormat::Pattern(p) => p.reader(),
            PatternFormat::Collection(_) => None,
        }
    }
    pub fn pattern_writer(&self) -> Option<Box<dyn PatternWriter>> {
        match self {
            PatternFormat::Pattern(p) => p.writer(),
            PatternFormat::Collection(_) => None,
        }
    }
    pub fn collection_reader(&self) -> Option<Box<dyn CollectionReader>> {
        match self {
            PatternFormat::Pattern(_) => None,
            PatternFormat::Collection(p) => p.reader(),
        }
    }
    pub fn collection_writer(&self) -> Option<Box<dyn CollectionWriter>> {
        match self {
            PatternFormat::Pattern(_) => None,
            PatternFormat::Collection(p) => p.writer(),
        }
    }
}

pub fn get_all() -> Vec<PatternFormat> {
    vec![
        PatternFormat::Pattern(Box::new(CsvPatternFormat::default())),
        PatternFormat::Pattern(Box::new(DstPatternFormat::default())),
        PatternFormat::Pattern(Box::new(HusPatternFormat::default())),
        PatternFormat::Pattern(Box::new(SvgPatternFormat::default())),
        PatternFormat::Pattern(Box::new(VipPatternFormat::default())),
        PatternFormat::Pattern(Box::new(Vp3PatternFormat::default())),
        PatternFormat::Collection(Box::new(Vf3CollectionFormat::default())),
    ]
}
pub fn get_pattern_readers() -> Vec<Box<dyn PatternReader>> {
    get_all()
        .into_iter()
        .filter_map(|format| format.pattern_reader())
        .collect()
}
pub fn get_pattern_writers() -> Vec<Box<dyn PatternWriter>> {
    get_all()
        .into_iter()
        .filter_map(|format| format.pattern_writer())
        .collect()
}

pub fn get_collection_readers() -> Vec<Box<dyn CollectionReader>> {
    get_all()
        .into_iter()
        .filter_map(|format| format.collection_reader())
        .collect()
}
pub fn get_collection_writers() -> Vec<Box<dyn CollectionWriter>> {
    get_all()
        .into_iter()
        .filter_map(|format| format.collection_writer())
        .collect()
}
