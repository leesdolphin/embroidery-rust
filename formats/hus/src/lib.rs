mod colors;
mod header;
mod read;
mod write;

use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

pub use read::HusVipPatternReader;
pub use write::HusVipPatternWriter;

const HUS_NAME: &str = "hus";
const HUS_EXTENSIONS: [&str; 1] = ["hus"];
const VIP_NAME: &str = "vip";
const VIP_EXTENSIONS: [&str; 1] = ["vip"];

#[derive(Default)]
pub struct HusPatternFormat {}

impl PatternFormat for HusPatternFormat {
    fn name<'a>(&self) -> &'a str {
        HUS_NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &HUS_EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        Some(Box::from(HusVipPatternReader::default()))
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        None
        // Some(Box::from(HusVipPatternWriter::hus()))
    }
}
#[derive(Default)]
pub struct VipPatternFormat {}

impl PatternFormat for VipPatternFormat {
    fn name<'a>(&self) -> &'a str {
        VIP_NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &VIP_EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        Some(Box::from(HusVipPatternReader::default()))
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        None
        // Some(Box::from(HusVipPatternWriter::vip()))
    }
}
