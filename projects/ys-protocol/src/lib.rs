use ys_types::YsError;

/// YS 原生二进制协议定义
pub trait YsProtocol: Send + Sync {
    // TODO: 定义二进制通讯协议的序列化与反序列化接口
}

pub mod git;
