/// Git 协议支持模块
///
/// 提供 Git pkt-line 协议的实现。
pub mod pkt_line;

pub use pkt_line::{PktLine, PktLineCodec};
