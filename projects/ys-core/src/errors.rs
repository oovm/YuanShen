use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use std::path::PathBuf;
use crate::ObjectID;


pub type Result<T> = std::result::Result<T, YsError>;

pub struct YsError {
    kind: Box<YsErrorKind>,
}

impl Error for YsError {}

impl Debug for YsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Display for YsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.kind, f)
    }
}

#[derive(Debug)]
pub enum YsErrorKind {
    IO {
        error: std::io::Error,
        path: Option<PathBuf>,
    },
    Serde {
        error: serde_json::Error
    },
    MissingObject {
        id: ObjectID
    },
}

impl Display for YsErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            YsErrorKind::IO { .. } => { todo!() }
            YsErrorKind::Serde { .. } => { todo!() }
            YsErrorKind::MissingObject { .. } => { todo!() }
        }
    }
}

impl From<YsErrorKind> for YsError {
    fn from(error: YsErrorKind) -> Self {
        YsError { kind: Box::new(error) }
    }
}

impl From<std::io::Error> for YsError {
    fn from(error: std::io::Error) -> Self {
        YsErrorKind::IO { error, path: None }.into()
    }
}

impl From<serde_json::Error> for YsError {
    fn from(error: serde_json::Error) -> Self {
        YsErrorKind::Serde { error }.into()
    }
}
