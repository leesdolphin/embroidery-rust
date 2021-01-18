mod read;

use embroidery_lib::format::{CollectionFormat, CollectionReader, CollectionWriter};

pub use self::read::Vf3CollectionReader;

const NAME: &str = "vf3";
const EXTENSIONS: [&str; 1] = ["vf3"];

#[derive(Default)]
pub struct Vf3CollectionFormat {}

impl CollectionFormat for Vf3CollectionFormat {
    fn name<'a>(&self) -> &'a str {
        NAME
    }
    fn extensions<'a, 'b>(&self) -> &'a [&'b str] {
        &EXTENSIONS
    }
    fn reader(&self) -> Option<Box<dyn CollectionReader>> {
        // None
        Some(Box::new(Vf3CollectionReader::default()))
    }
    fn writer(&self) -> Option<Box<dyn CollectionWriter>> {
        None
    }
}
