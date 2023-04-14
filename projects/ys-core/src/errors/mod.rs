use std::{
    convert::Infallible,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

use crate::ObjectID;
use std::path::PathBuf;

/// 便捷 Result 类型, 可以少写一个 [YsError]
pub type Result<T> = std::result::Result<T, YsError>;

/// 源神架构的统一错误指针
pub struct YsError {
    kind: Box<YsErrorKind>,
}

impl YsError {
    /// 根据IO错误和相关路径创建一个错误实例。
    pub fn path_error<P: Into<PathBuf>>(error: std::io::Error, path: P) -> Self {
        Self { kind: Box::new(YsErrorKind::IO { error, path: Some(path.into()) }) }
    }
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

/// YsErrorKind 定义了可能发生的错误类型。
#[derive(Debug)]
pub enum YsErrorKind {
    /// IO 错误类型，包含原始的 IO 错误信息和错误发生的位置。
    IO {
        /// 原始的 IO 错误。
        error: std::io::Error,
        /// 错误发生的位置，可能是文件路径或其他资源的标识。
        path: Option<PathBuf>,
    },
    // 序列化或反序列化错误类型，包含原始的序列化错误信息。
    Serde {
        /// 原始序列化错误。
        error: serde_json::Error,
    },
    /// 表示找不到特定对象的错误类型。
    MissingObject {
        /// 缺失对象的 ID。
        id: ObjectID,
    },
}

impl Display for YsErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO { .. } => {
                todo!()
            }
            Self::Serde { .. } => {
                todo!()
            }
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
