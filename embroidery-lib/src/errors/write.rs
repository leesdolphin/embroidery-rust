use std::result;

use super::{ErrorWithContext, StdError};
use crate::stitch::Stitch;

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Unable to write stitch {:?} at {:?}", stitch, idx)]
    UnsupportedStitch {
        stitch: Stitch,
        idx: Option<usize>,
        ctx: Vec<String>,
    },

    #[fail(display = "{}", _0)]
    Std(#[cause] StdError, Vec<String>),
}

impl Error {
    pub fn unsupported_stitch(stitch: Stitch, idx: Option<usize>) -> Self {
        Self::UnsupportedStitch {
            stitch,
            idx,
            ctx: vec![],
        }
    }
    pub fn unsupported_stitch_msg<S>(stitch: Stitch, idx: Option<usize>, msg: S) -> Self
    where
        S: Into<String>,
    {
        Self::UnsupportedStitch {
            stitch,
            idx,
            ctx: vec![msg.into()],
        }
    }
}

impl ErrorWithContext for Error {
    fn context(&self) -> Vec<String> {
        match self {
            Self::UnsupportedStitch { ctx, .. } => ctx.clone(),
            Self::Std(_, c) => c.clone(),
        }
    }
    fn with_additional_context<S>(self, extra: S) -> Self
    where
        S: Into<String>,
    {
        match self {
            Self::UnsupportedStitch { stitch, idx, mut ctx } => {
                ctx.push(extra.into());
                Self::UnsupportedStitch { stitch, idx, ctx }
            },
            Self::Std(e, mut c) => {
                c.push(extra.into());
                Self::Std(e, c)
            },
        }
    }
    fn without_context(self) -> Self {
        match self {
            Self::UnsupportedStitch { stitch, idx, .. } => Self::UnsupportedStitch {
                stitch,
                idx,
                ctx: vec![],
            },
            Self::Std(e, _) => Self::Std(e, vec![]),
        }
    }
}

impl<T: Into<StdError>> From<T> for Error {
    fn from(err: T) -> Self {
        Error::Std(err.into(), vec![])
    }
}

pub type Result<T> = result::Result<T, Error>;
