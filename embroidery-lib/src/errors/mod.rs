pub use self::read::{Error as ReadError, Result as ReadResult};
pub use self::write::{Error as WriteError, Result as WriteResult};
use std::fmt;
use std::fmt::Display;
use std::io;
use std::result;

pub mod read;
pub mod write;

pub trait ErrorWithContext {
    fn context(&self) -> Vec<String>;
    fn with_additional_context<S>(self, extra: S) -> Self
    where
        S: Into<String>;
    fn without_context(self) -> Self;

    fn context_string(&self) -> String {
        let ctx = self.context();
        if ctx.is_empty() {
            "".to_string()
        } else {
            format!("\nAdditional error context(deepest first):\n{}", ctx.join("\n"))
        }
    }
    fn set_context<I>(self, iter: I) -> Self
    where
        I: IntoIterator<Item = String>,
        Self: std::marker::Sized,
    {
        let mut s = self.without_context();
        for c in iter {
            s = s.with_additional_context(c)
        }
        s
    }
}

#[derive(Debug, Fail)]
pub enum StdError {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "{}", _0)]
    Fmt(#[cause] fmt::Error),
}

impl From<io::Error> for StdError {
    fn from(err: io::Error) -> Self {
        StdError::Io(err)
    }
}

impl From<fmt::Error> for StdError {
    fn from(err: fmt::Error) -> Self {
        StdError::Fmt(err)
    }
}

#[derive(Fail, Debug)]
pub enum Error {
    Read(#[cause] ReadError, Vec<String>),
    Write(#[cause] WriteError, Vec<String>),
    Standard(#[cause] StdError, Vec<String>),
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e, _) => write!(f, "Read error: {:?}", e)?,
            Self::Write(e, _) => write!(f, "Write error: {:?}", e)?,
            Self::Standard(StdError::Fmt(e), _) => write!(f, "Formatter error: {:?}", e)?,
            Self::Standard(StdError::Io(e), _) => write!(f, "IO error: {:?}", e)?,
        };
        write!(f, "{}", self.context_string())
    }
}
impl From<ReadError> for Error {
    fn from(err: ReadError) -> Self {
        match err {
            ReadError::Std(err, ctx) => Error::Standard(err, ctx).with_additional_context("Converted from ReadError"),
            err => {
                let c = err.context();
                Error::Read(err.without_context(), c)
            },
        }
    }
}
impl From<WriteError> for Error {
    fn from(err: WriteError) -> Self {
        match err {
            WriteError::Std(err, ctx) => Error::Standard(err, ctx).with_additional_context("Converted from WriteError"),
            err => {
                let c = err.context();
                Error::Write(err.without_context(), c)
            },
        }
    }
}
impl<T: Into<StdError>> From<T> for Error {
    fn from(err: T) -> Self {
        Error::Standard(err.into(), vec![])
    }
}
impl ErrorWithContext for Error {
    fn context(&self) -> Vec<String> {
        match self {
            Self::Read(_, c) => {
                // let ictx = e.context()
                let ictx = vec![];
                let mut ctx = Vec::with_capacity(ictx.len() + c.len());
                ctx.extend_from_slice(&ictx);
                ctx.extend_from_slice(c);
                ctx
            },
            Self::Write(_, c) => {
                // let ictx = e.context()
                let ictx = vec![];
                let mut ctx = Vec::with_capacity(ictx.len() + c.len());
                ctx.extend_from_slice(&ictx);
                ctx.extend_from_slice(c);
                ctx
            },
            Self::Standard(_, c) => c.clone(),
        }
    }
    fn with_additional_context<S>(self, extra: S) -> Self
    where
        S: Into<String>,
    {
        match self {
            Self::Read(e, mut c) => {
                c.push(extra.into());
                Self::Read(e, c)
            },
            Self::Write(e, mut c) => {
                c.push(extra.into());
                Self::Write(e, c)
            },
            Self::Standard(e, mut c) => {
                c.push(extra.into());
                Self::Standard(e, c)
            },
        }
    }
    fn without_context(self) -> Self {
        match self {
            Self::Read(e, _) => Self::Read(e, vec![]),
            Self::Write(e, _) => Self::Write(e, vec![]),
            Self::Standard(e, _) => Self::Standard(e, vec![]),
        }
    }
}

#[inline]
pub fn add_context<T, E, F, S>(result: result::Result<T, E>, ctx: F) -> result::Result<T, E>
where
    E: ErrorWithContext,
    S: Into<String>,
    F: FnOnce() -> S,
{
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(e.with_additional_context(ctx())),
    }
}
#[inline]
pub fn add_context_fn<T, E, S, EF, CF>(func: EF, ctx: CF) -> result::Result<T, E>
where
    E: ErrorWithContext,
    EF: FnOnce() -> result::Result<T, E>,
    S: Into<String>,
    CF: FnOnce() -> S,
{
    match func() {
        Ok(r) => Ok(r),
        Err(e) => Err(e.with_additional_context(ctx())),
    }
}

pub type Result<T> = result::Result<T, Error>;
