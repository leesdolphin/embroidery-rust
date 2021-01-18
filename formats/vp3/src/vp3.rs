mod read;
// mod write;

use embroidery_lib::format::{PatternFormat, PatternReader, PatternWriter};

pub use self::read::Vp3PatternReader;
// pub use self::write::Vp3PatternWriter;

const NAME: &str = "vp3";
const EXTENSIONS: [&str; 1] = ["vp3"];

#[derive(Default)]
pub struct Vp3PatternFormat {}

impl PatternFormat for Vp3PatternFormat {
    fn name<'a>(&self) -> &'a str {
        NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn PatternReader>> {
        Some(Box::from(Vp3PatternReader::default()))
    }
    fn writer(&self) -> Option<Box<dyn PatternWriter>> {
        None
    }
}
