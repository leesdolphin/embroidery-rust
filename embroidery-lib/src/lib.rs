#[macro_use]
pub extern crate failure;
#[macro_use]
#[allow(unused_imports)]
pub extern crate log;

mod byte_utils;
mod collection;
mod colors;
mod enum_utils;
mod pattern;
mod stitch;
mod stitch_util;
mod str_util;

pub mod errors;
pub mod format;
pub mod transforms;

pub use crate::collection::PatternCollection;
pub use crate::colors::Color;
pub use crate::errors::{Error, ReadError, WriteError};
pub use crate::pattern::{Pattern, PatternAttribute};
pub use crate::stitch::{ColorGroup, Stitch, StitchGroup, Thread};

pub mod utils {
    pub use crate::byte_utils::{format_u8_array, ReadByteIterator};
    pub use crate::stitch_util::{build_stitch_list, StitchInfo};
    pub use crate::str_util::{c_trim, char_truncate};
}

pub mod prelude {
    pub use log::{debug, error, info, trace, warn};

    pub use crate::collection::PatternCollection;
    pub use crate::colors::Color;
    pub use crate::errors::{Error, ReadError, WriteError};
    pub use crate::pattern::{Pattern, PatternAttribute};
    pub use crate::stitch::{ColorGroup, Stitch, StitchGroup, Thread};
}
