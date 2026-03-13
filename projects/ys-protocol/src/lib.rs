/// YS 协议库
///
/// 提供 YS 原生二进制协议和 Git 协议的支持。
pub mod git;

/// YS 原生二进制协议模块
pub mod ys;

pub use git::{PktLine, PktLineCodec};
pub use ys::{YsMessage, YsMessageType, YsProtocolCodec};

/// YS 原生二进制协议定义
pub trait YsProtocol: Send + Sync {}
