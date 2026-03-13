pub mod codec;
/// YS 原生二进制协议模块
///
/// 提供 YS 版本控制系统的原生二进制协议实现。
/// 该协议设计用于高效传输版本控制相关数据。
pub mod message;

pub use codec::YsProtocolCodec;
pub use message::{YsMessage, YsMessageType};
