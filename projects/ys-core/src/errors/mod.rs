use std::convert::Infallible;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use std::path::PathBuf;
use crate::ObjectID;

/// 便捷 Result 类型, 可以少写一个 [YsError]
pub type Result<T> = std::result::Result<T, YsError>;

/// 整个架构的错误指针
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
    /// IO 错误
    IO {
        /// 原始的 IO 错误
        error: std::io::Error,
        /// 错误发生的位置
        path: Option<PathBuf>,
    },
    // 序列化错误
    Serde {
        /// 原始序列化错误
        error: serde_json::Error
    },
    /// 找不到对象
    MissingObject {
        /// 对象 ID
        id: ObjectID
    },
}

impl Display for YsErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO { .. } => { todo!() }
            Self::Serde { .. } => { todo!() }
            Self::MissingObject { id } => {
                write!(f, "找不到对象: {}", id)
            }
        }
    }
}


impl From<YsErrorKind> for YsError {
    fn from(error: YsErrorKind) -> Self {
        YsError { kind: Box::new(error) }
    }
}

impl From<Infallible> for YsError {
    fn from(_: Infallible) -> Self {
        unreachable!()
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
