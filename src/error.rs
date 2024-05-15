use std::borrow::Cow;

use iref::IriBuf;
use json_ld::{flattening::ConflictingIndexes, none::CannotLoad, ExpandError};
use rdf_types::BlankIdBuf;

/// All possible error kinds.
#[derive(Debug, Copy, Clone)]
pub enum ErrorKind {
    /// JSON-LD error.
    JsonLd,
    /// Path format error.
    PathFormat,
    /// I/O error.
    Io,
    /// Template error.
    Template,
}

impl ErrorKind {
    pub(crate) const fn description(self) -> &'static str {
        match self {
            ErrorKind::JsonLd => "JSON-LD error",
            ErrorKind::PathFormat => "Path format error",
            ErrorKind::Io => "I/O error",
            ErrorKind::Template => "Template error",
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.description().fmt(f)
    }
}

/// Library error.
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    info: Cow<'static, str>,
}

impl Error {
    pub(crate) fn new(kind: ErrorKind, info: impl Into<Cow<'static, str>>) -> Self {
        Self {
            kind,
            info: info.into(),
        }
    }

    pub(crate) fn error(&self) -> String {
        format!("{}: {}", self.kind, self.info)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.error().fmt(f)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::new(ErrorKind::Io, e.to_string())
    }
}

impl From<minijinja::Error> for Error {
    fn from(e: minijinja::Error) -> Self {
        Self::new(ErrorKind::Io, e.to_string())
    }
}

impl From<ConflictingIndexes<IriBuf, BlankIdBuf>> for Error {
    fn from(e: ConflictingIndexes<IriBuf, BlankIdBuf>) -> Self {
        Self::new(ErrorKind::JsonLd, e.to_string())
    }
}

impl From<ExpandError<CannotLoad<IriBuf>>> for Error {
    fn from(e: ExpandError<CannotLoad<IriBuf>>) -> Self {
        Self::new(ErrorKind::JsonLd, e.to_string())
    }
}

impl From<json_ld::syntax::parse::Error> for Error {
    fn from(e: json_ld::syntax::parse::Error) -> Self {
        Self::new(ErrorKind::JsonLd, e.to_string())
    }
}

/// A specialized `Result` type.
pub type Result<T> = std::result::Result<T, Error>;
