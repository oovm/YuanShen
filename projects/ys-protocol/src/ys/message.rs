use bytes::{Buf, BufMut, Bytes};
use std::io;
use ys_types::ObjectID;

/// YS 协议消息类型
///
/// 定义了 YS 原生二进制协议支持的所有消息类型。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum YsMessageType {
    /// 握手请求消息
    ///
    /// 客户端发送给服务器的初始握手消息，包含客户端信息和能力声明。
    HandshakeRequest = 0x01,
    /// 握手响应消息
    ///
    /// 服务器响应客户端握手的消息，包含服务器信息和接受的能力。
    HandshakeResponse = 0x02,
    /// 获取对象请求
    ///
    /// 请求获取指定 ID 的对象。
    GetObjectRequest = 0x10,
    /// 获取对象响应
    ///
    /// 包含请求的对象数据。
    GetObjectResponse = 0x11,
    /// 上传对象请求
    ///
    /// 上传一个新的对象到服务器。
    PutObjectRequest = 0x12,
    /// 上传对象响应
    ///
    /// 确认对象上传成功。
    PutObjectResponse = 0x13,
    /// 批量获取对象请求
    ///
    /// 请求获取多个对象。
    BatchGetObjectRequest = 0x14,
    /// 批量获取对象响应
    ///
    /// 包含多个请求的对象数据。
    BatchGetObjectResponse = 0x15,
    /// 获取提交历史请求
    ///
    /// 请求获取提交历史记录。
    GetCommitHistoryRequest = 0x20,
    /// 获取提交历史响应
    ///
    /// 包含提交历史记录。
    GetCommitHistoryResponse = 0x21,
    /// 创建分支请求
    ///
    /// 请求创建一个新分支。
    CreateBranchRequest = 0x30,
    /// 创建分支响应
    ///
    /// 确认分支创建成功。
    CreateBranchResponse = 0x31,
    /// 删除分支请求
    ///
    /// 请求删除一个分支。
    DeleteBranchRequest = 0x32,
    /// 删除分支响应
    ///
    /// 确认分支删除成功。
    DeleteBranchResponse = 0x33,
    /// 列出分支请求
    ///
    /// 请求列出所有分支。
    ListBranchesRequest = 0x34,
    /// 列出分支响应
    ///
    /// 包含所有分支的列表。
    ListBranchesResponse = 0x35,
    /// 心跳请求
    ///
    /// 用于保持连接活跃。
    HeartbeatRequest = 0xF0,
    /// 心跳响应
    ///
    /// 对心跳请求的响应。
    HeartbeatResponse = 0xF1,
    /// 错误消息
    ///
    /// 表示操作失败，包含错误信息。
    Error = 0xFF,
}

impl YsMessageType {
    /// 从 u8 值转换为 YsMessageType
    ///
    /// # Arguments
    /// * `value` - 要转换的 u8 值
    ///
    /// # Returns
    /// * `Some(YsMessageType)` - 成功转换
    /// * `None` - 无效的消息类型
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x01 => Some(YsMessageType::HandshakeRequest),
            0x02 => Some(YsMessageType::HandshakeResponse),
            0x10 => Some(YsMessageType::GetObjectRequest),
            0x11 => Some(YsMessageType::GetObjectResponse),
            0x12 => Some(YsMessageType::PutObjectRequest),
            0x13 => Some(YsMessageType::PutObjectResponse),
            0x14 => Some(YsMessageType::BatchGetObjectRequest),
            0x15 => Some(YsMessageType::BatchGetObjectResponse),
            0x20 => Some(YsMessageType::GetCommitHistoryRequest),
            0x21 => Some(YsMessageType::GetCommitHistoryResponse),
            0x30 => Some(YsMessageType::CreateBranchRequest),
            0x31 => Some(YsMessageType::CreateBranchResponse),
            0x32 => Some(YsMessageType::DeleteBranchRequest),
            0x33 => Some(YsMessageType::DeleteBranchResponse),
            0x34 => Some(YsMessageType::ListBranchesRequest),
            0x35 => Some(YsMessageType::ListBranchesResponse),
            0xF0 => Some(YsMessageType::HeartbeatRequest),
            0xF1 => Some(YsMessageType::HeartbeatResponse),
            0xFF => Some(YsMessageType::Error),
            _ => None,
        }
    }

    /// 转换为 u8 值
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}

/// YS 协议消息
///
/// YS 原生二进制协议的核心消息类型，包含所有可能的消息变体。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum YsMessage {
    /// 握手请求
    ///
    /// # Fields
    /// * `version` - 协议版本
    /// * `client_id` - 客户端标识符
    /// * `capabilities` - 客户端支持的能力列表
    HandshakeRequest { version: u32, client_id: String, capabilities: Vec<String> },
    /// 握手响应
    ///
    /// # Fields
    /// * `version` - 协议版本
    /// * `server_id` - 服务器标识符
    /// * `accepted_capabilities` - 服务器接受的能力列表
    HandshakeResponse { version: u32, server_id: String, accepted_capabilities: Vec<String> },
    /// 获取对象请求
    ///
    /// # Fields
    /// * `object_id` - 要获取的对象 ID
    GetObjectRequest { object_id: ObjectID },
    /// 获取对象响应
    ///
    /// # Fields
    /// * `object_id` - 对象 ID
    /// * `object_type` - 对象类型
    /// * `data` - 对象数据
    GetObjectResponse { object_id: ObjectID, object_type: String, data: Bytes },
    /// 上传对象请求
    ///
    /// # Fields
    /// * `object_id` - 对象 ID
    /// * `object_type` - 对象类型
    /// * `data` - 对象数据
    PutObjectRequest { object_id: ObjectID, object_type: String, data: Bytes },
    /// 上传对象响应
    ///
    /// # Fields
    /// * `object_id` - 对象 ID
    /// * `success` - 是否成功
    PutObjectResponse { object_id: ObjectID, success: bool },
    /// 批量获取对象请求
    ///
    /// # Fields
    /// * `object_ids` - 要获取的对象 ID 列表
    BatchGetObjectRequest { object_ids: Vec<ObjectID> },
    /// 批量获取对象响应
    ///
    /// # Fields
    /// * `objects` - 对象列表，每个元素包含 (object_id, object_type, data)
    BatchGetObjectResponse { objects: Vec<(ObjectID, String, Bytes)> },
    /// 获取提交历史请求
    ///
    /// # Fields
    /// * `branch_name` - 分支名称
    /// * `limit` - 最大返回数量
    GetCommitHistoryRequest { branch_name: String, limit: u32 },
    /// 获取提交历史响应
    ///
    /// # Fields
    /// * `commits` - 提交历史列表
    GetCommitHistoryResponse { commits: Vec<Bytes> },
    /// 创建分支请求
    ///
    /// # Fields
    /// * `branch_name` - 分支名称
    /// * `commit_id` - 起始提交 ID
    CreateBranchRequest { branch_name: String, commit_id: ObjectID },
    /// 创建分支响应
    ///
    /// # Fields
    /// * `branch_name` - 分支名称
    /// * `success` - 是否成功
    CreateBranchResponse { branch_name: String, success: bool },
    /// 删除分支请求
    ///
    /// # Fields
    /// * `branch_name` - 分支名称
    DeleteBranchRequest { branch_name: String },
    /// 删除分支响应
    ///
    /// # Fields
    /// * `branch_name` - 分支名称
    /// * `success` - 是否成功
    DeleteBranchResponse { branch_name: String, success: bool },
    /// 列出分支请求
    ListBranchesRequest,
    /// 列出分支响应
    ///
    /// # Fields
    /// * `branches` - 分支列表，每个元素包含 (branch_name, commit_id)
    ListBranchesResponse { branches: Vec<(String, ObjectID)> },
    /// 心跳请求
    HeartbeatRequest,
    /// 心跳响应
    HeartbeatResponse,
    /// 错误消息
    ///
    /// # Fields
    /// * `code` - 错误代码
    /// * `message` - 错误消息
    Error { code: u32, message: String },
}

impl YsMessage {
    /// 获取消息类型
    pub fn message_type(&self) -> YsMessageType {
        match self {
            YsMessage::HandshakeRequest { .. } => YsMessageType::HandshakeRequest,
            YsMessage::HandshakeResponse { .. } => YsMessageType::HandshakeResponse,
            YsMessage::GetObjectRequest { .. } => YsMessageType::GetObjectRequest,
            YsMessage::GetObjectResponse { .. } => YsMessageType::GetObjectResponse,
            YsMessage::PutObjectRequest { .. } => YsMessageType::PutObjectRequest,
            YsMessage::PutObjectResponse { .. } => YsMessageType::PutObjectResponse,
            YsMessage::BatchGetObjectRequest { .. } => YsMessageType::BatchGetObjectRequest,
            YsMessage::BatchGetObjectResponse { .. } => YsMessageType::BatchGetObjectResponse,
            YsMessage::GetCommitHistoryRequest { .. } => YsMessageType::GetCommitHistoryRequest,
            YsMessage::GetCommitHistoryResponse { .. } => YsMessageType::GetCommitHistoryResponse,
            YsMessage::CreateBranchRequest { .. } => YsMessageType::CreateBranchRequest,
            YsMessage::CreateBranchResponse { .. } => YsMessageType::CreateBranchResponse,
            YsMessage::DeleteBranchRequest { .. } => YsMessageType::DeleteBranchRequest,
            YsMessage::DeleteBranchResponse { .. } => YsMessageType::DeleteBranchResponse,
            YsMessage::ListBranchesRequest => YsMessageType::ListBranchesRequest,
            YsMessage::ListBranchesResponse { .. } => YsMessageType::ListBranchesResponse,
            YsMessage::HeartbeatRequest => YsMessageType::HeartbeatRequest,
            YsMessage::HeartbeatResponse => YsMessageType::HeartbeatResponse,
            YsMessage::Error { .. } => YsMessageType::Error,
        }
    }

    /// 将消息序列化为字节
    ///
    /// # Arguments
    /// * `dst` - 目标缓冲区
    pub fn encode(&self, dst: &mut impl BufMut) -> io::Result<()> {
        match self {
            YsMessage::HandshakeRequest { version, client_id, capabilities } => {
                dst.put_u32(*version);
                encode_string(dst, client_id)?;
                encode_string_list(dst, capabilities)?;
            }
            YsMessage::HandshakeResponse { version, server_id, accepted_capabilities } => {
                dst.put_u32(*version);
                encode_string(dst, server_id)?;
                encode_string_list(dst, accepted_capabilities)?;
            }
            YsMessage::GetObjectRequest { object_id } => {
                encode_object_id(dst, object_id)?;
            }
            YsMessage::GetObjectResponse { object_id, object_type, data } => {
                encode_object_id(dst, object_id)?;
                encode_string(dst, object_type)?;
                encode_bytes(dst, data)?;
            }
            YsMessage::PutObjectRequest { object_id, object_type, data } => {
                encode_object_id(dst, object_id)?;
                encode_string(dst, object_type)?;
                encode_bytes(dst, data)?;
            }
            YsMessage::PutObjectResponse { object_id, success } => {
                encode_object_id(dst, object_id)?;
                dst.put_u8(if *success { 1 } else { 0 });
            }
            YsMessage::BatchGetObjectRequest { object_ids } => {
                encode_object_id_list(dst, object_ids)?;
            }
            YsMessage::BatchGetObjectResponse { objects } => {
                dst.put_u32(objects.len() as u32);
                for (id, typ, data) in objects {
                    encode_object_id(dst, id)?;
                    encode_string(dst, typ)?;
                    encode_bytes(dst, data)?;
                }
            }
            YsMessage::GetCommitHistoryRequest { branch_name, limit } => {
                encode_string(dst, branch_name)?;
                dst.put_u32(*limit);
            }
            YsMessage::GetCommitHistoryResponse { commits } => {
                dst.put_u32(commits.len() as u32);
                for commit in commits {
                    encode_bytes(dst, commit)?;
                }
            }
            YsMessage::CreateBranchRequest { branch_name, commit_id } => {
                encode_string(dst, branch_name)?;
                encode_object_id(dst, commit_id)?;
            }
            YsMessage::CreateBranchResponse { branch_name, success } => {
                encode_string(dst, branch_name)?;
                dst.put_u8(if *success { 1 } else { 0 });
            }
            YsMessage::DeleteBranchRequest { branch_name } => {
                encode_string(dst, branch_name)?;
            }
            YsMessage::DeleteBranchResponse { branch_name, success } => {
                encode_string(dst, branch_name)?;
                dst.put_u8(if *success { 1 } else { 0 });
            }
            YsMessage::ListBranchesRequest => {}
            YsMessage::ListBranchesResponse { branches } => {
                dst.put_u32(branches.len() as u32);
                for (name, id) in branches {
                    encode_string(dst, name)?;
                    encode_object_id(dst, id)?;
                }
            }
            YsMessage::HeartbeatRequest => {}
            YsMessage::HeartbeatResponse => {}
            YsMessage::Error { code, message } => {
                dst.put_u32(*code);
                encode_string(dst, message)?;
            }
        }
        Ok(())
    }

    /// 从字节反序列化消息
    ///
    /// # Arguments
    /// * `msg_type` - 消息类型
    /// * `src` - 源缓冲区
    ///
    /// # Returns
    /// * 反序列化的消息
    pub fn decode(msg_type: YsMessageType, src: &mut impl Buf) -> io::Result<Self> {
        match msg_type {
            YsMessageType::HandshakeRequest => {
                let version = src.get_u32();
                let client_id = decode_string(src)?;
                let capabilities = decode_string_list(src)?;
                Ok(YsMessage::HandshakeRequest { version, client_id, capabilities })
            }
            YsMessageType::HandshakeResponse => {
                let version = src.get_u32();
                let server_id = decode_string(src)?;
                let accepted_capabilities = decode_string_list(src)?;
                Ok(YsMessage::HandshakeResponse { version, server_id, accepted_capabilities })
            }
            YsMessageType::GetObjectRequest => {
                let object_id = decode_object_id(src)?;
                Ok(YsMessage::GetObjectRequest { object_id })
            }
            YsMessageType::GetObjectResponse => {
                let object_id = decode_object_id(src)?;
                let object_type = decode_string(src)?;
                let data = decode_bytes(src)?;
                Ok(YsMessage::GetObjectResponse { object_id, object_type, data })
            }
            YsMessageType::PutObjectRequest => {
                let object_id = decode_object_id(src)?;
                let object_type = decode_string(src)?;
                let data = decode_bytes(src)?;
                Ok(YsMessage::PutObjectRequest { object_id, object_type, data })
            }
            YsMessageType::PutObjectResponse => {
                let object_id = decode_object_id(src)?;
                let success = src.get_u8() != 0;
                Ok(YsMessage::PutObjectResponse { object_id, success })
            }
            YsMessageType::BatchGetObjectRequest => {
                let object_ids = decode_object_id_list(src)?;
                Ok(YsMessage::BatchGetObjectRequest { object_ids })
            }
            YsMessageType::BatchGetObjectResponse => {
                let count = src.get_u32() as usize;
                let mut objects = Vec::with_capacity(count);
                for _ in 0..count {
                    let id = decode_object_id(src)?;
                    let typ = decode_string(src)?;
                    let data = decode_bytes(src)?;
                    objects.push((id, typ, data));
                }
                Ok(YsMessage::BatchGetObjectResponse { objects })
            }
            YsMessageType::GetCommitHistoryRequest => {
                let branch_name = decode_string(src)?;
                let limit = src.get_u32();
                Ok(YsMessage::GetCommitHistoryRequest { branch_name, limit })
            }
            YsMessageType::GetCommitHistoryResponse => {
                let count = src.get_u32() as usize;
                let mut commits = Vec::with_capacity(count);
                for _ in 0..count {
                    commits.push(decode_bytes(src)?);
                }
                Ok(YsMessage::GetCommitHistoryResponse { commits })
            }
            YsMessageType::CreateBranchRequest => {
                let branch_name = decode_string(src)?;
                let commit_id = decode_object_id(src)?;
                Ok(YsMessage::CreateBranchRequest { branch_name, commit_id })
            }
            YsMessageType::CreateBranchResponse => {
                let branch_name = decode_string(src)?;
                let success = src.get_u8() != 0;
                Ok(YsMessage::CreateBranchResponse { branch_name, success })
            }
            YsMessageType::DeleteBranchRequest => {
                let branch_name = decode_string(src)?;
                Ok(YsMessage::DeleteBranchRequest { branch_name })
            }
            YsMessageType::DeleteBranchResponse => {
                let branch_name = decode_string(src)?;
                let success = src.get_u8() != 0;
                Ok(YsMessage::DeleteBranchResponse { branch_name, success })
            }
            YsMessageType::ListBranchesRequest => Ok(YsMessage::ListBranchesRequest),
            YsMessageType::ListBranchesResponse => {
                let count = src.get_u32() as usize;
                let mut branches = Vec::with_capacity(count);
                for _ in 0..count {
                    let name = decode_string(src)?;
                    let id = decode_object_id(src)?;
                    branches.push((name, id));
                }
                Ok(YsMessage::ListBranchesResponse { branches })
            }
            YsMessageType::HeartbeatRequest => Ok(YsMessage::HeartbeatRequest),
            YsMessageType::HeartbeatResponse => Ok(YsMessage::HeartbeatResponse),
            YsMessageType::Error => {
                let code = src.get_u32();
                let message = decode_string(src)?;
                Ok(YsMessage::Error { code, message })
            }
        }
    }
}

/// 编码字符串
fn encode_string(dst: &mut impl BufMut, s: &str) -> io::Result<()> {
    let bytes = s.as_bytes();
    if bytes.len() > u32::MAX as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "String too long"));
    }
    dst.put_u32(bytes.len() as u32);
    dst.put_slice(bytes);
    Ok(())
}

/// 解码字符串
fn decode_string(src: &mut impl Buf) -> io::Result<String> {
    let len = src.get_u32() as usize;
    if src.remaining() < len {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Incomplete string data"));
    }
    let bytes = src.copy_to_bytes(len);
    String::from_utf8(bytes.to_vec()).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// 编码字符串列表
fn encode_string_list(dst: &mut impl BufMut, list: &[String]) -> io::Result<()> {
    if list.len() > u32::MAX as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "List too long"));
    }
    dst.put_u32(list.len() as u32);
    for s in list {
        encode_string(dst, s)?;
    }
    Ok(())
}

/// 解码字符串列表
fn decode_string_list(src: &mut impl Buf) -> io::Result<Vec<String>> {
    let count = src.get_u32() as usize;
    let mut list = Vec::with_capacity(count);
    for _ in 0..count {
        list.push(decode_string(src)?);
    }
    Ok(list)
}

/// 编码字节
fn encode_bytes(dst: &mut impl BufMut, data: &[u8]) -> io::Result<()> {
    if data.len() > u32::MAX as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Data too long"));
    }
    dst.put_u32(data.len() as u32);
    dst.put_slice(data);
    Ok(())
}

/// 解码字节
fn decode_bytes(src: &mut impl Buf) -> io::Result<Bytes> {
    let len = src.get_u32() as usize;
    if src.remaining() < len {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Incomplete data"));
    }
    Ok(src.copy_to_bytes(len))
}

/// 编码 ObjectID
fn encode_object_id(dst: &mut impl BufMut, id: &ObjectID) -> io::Result<()> {
    dst.put_slice(id.as_bytes());
    Ok(())
}

/// 解码 ObjectID
fn decode_object_id(src: &mut impl Buf) -> io::Result<ObjectID> {
    if src.remaining() < 16 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Incomplete ObjectID"));
    }
    let mut bytes = [0u8; 16];
    src.copy_to_slice(&mut bytes);
    ObjectID::from_bytes(&bytes).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}

/// 编码 ObjectID 列表
fn encode_object_id_list(dst: &mut impl BufMut, ids: &[ObjectID]) -> io::Result<()> {
    if ids.len() > u32::MAX as usize {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "List too long"));
    }
    dst.put_u32(ids.len() as u32);
    for id in ids {
        encode_object_id(dst, id)?;
    }
    Ok(())
}

/// 解码 ObjectID 列表
fn decode_object_id_list(src: &mut impl Buf) -> io::Result<Vec<ObjectID>> {
    let count = src.get_u32() as usize;
    let mut ids = Vec::with_capacity(count);
    for _ in 0..count {
        ids.push(decode_object_id(src)?);
    }
    Ok(ids)
}
