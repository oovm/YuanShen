use super::message::{YsMessage, YsMessageType};
use bytes::{Buf, BufMut, BytesMut};
use std::io;
use tokio_util::codec::{Decoder, Encoder};

/// YS 协议魔数
///
/// 用于标识 YS 协议数据包，防止误解析其他协议数据。
const YS_MAGIC: u32 = 0x59535052;

/// YS 协议版本
const YS_PROTOCOL_VERSION: u8 = 1;

/// YS 协议编解码器
///
/// 实现 tokio_util 的 Decoder 和 Encoder trait，
/// 用于在字节流和 YsMessage 之间进行转换。
///
/// 协议帧格式：
/// ```text
/// +----------------+----------------+----------------+----------------+
/// | Magic (4 bytes)| Version (1)    | Type (1)       | Reserved (2)   |
/// +----------------+----------------+----------------+----------------+
/// | Payload Length (4 bytes)                                      |
/// +----------------------------------------------------------------+
/// | Payload (variable length)                                     |
/// +----------------------------------------------------------------+
/// ```
pub struct YsProtocolCodec;

impl Decoder for YsProtocolCodec {
    type Item = YsMessage;
    type Error = io::Error;

    /// 从字节流中解码出一个 YsMessage
    ///
    /// # Arguments
    /// * `src` - 输入的字节流缓冲区
    ///
    /// # Returns
    /// * `Ok(Some(item))` - 成功解码出一个消息
    /// * `Ok(None)` - 需要更多数据才能解码
    /// * `Err(err)` - 解码出错
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let header_size = 12;

        if src.len() < header_size {
            return Ok(None);
        }

        let mut cursor = &src[..];

        let magic = cursor.get_u32();
        if magic != YS_MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid YS protocol magic"));
        }

        let version = cursor.get_u8();
        if version != YS_PROTOCOL_VERSION {
            return Err(io::Error::new(io::ErrorKind::InvalidData, format!("Unsupported protocol version: {}", version)));
        }

        let msg_type_byte = cursor.get_u8();
        let msg_type = YsMessageType::from_u8(msg_type_byte).ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Invalid message type: 0x{:02x}", msg_type_byte))
        })?;

        let _reserved = cursor.get_u16();
        let payload_len = cursor.get_u32() as usize;

        let total_len = header_size + payload_len;
        if src.len() < total_len {
            return Ok(None);
        }

        src.advance(header_size);
        let mut payload = src.split_to(payload_len);

        let message = YsMessage::decode(msg_type, &mut payload)?;

        Ok(Some(message))
    }
}

impl Encoder<YsMessage> for YsProtocolCodec {
    type Error = io::Error;

    /// 将 YsMessage 编码到字节流中
    ///
    /// # Arguments
    /// * `item` - 要编码的 YsMessage
    /// * `dst` - 输出的字节流缓冲区
    ///
    /// # Returns
    /// * `Ok(())` - 编码成功
    /// * `Err(err)` - 编码出错
    fn encode(&mut self, item: YsMessage, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg_type = item.message_type();

        let mut payload = BytesMut::new();
        item.encode(&mut payload)?;

        let payload_len = payload.len();
        if payload_len > u32::MAX as usize {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "Payload too large"));
        }

        dst.put_u32(YS_MAGIC);
        dst.put_u8(YS_PROTOCOL_VERSION);
        dst.put_u8(msg_type.as_u8());
        dst.put_u16(0);
        dst.put_u32(payload_len as u32);
        dst.put(payload);

        Ok(())
    }
}

impl YsProtocolCodec {
    /// 创建一个新的 YsProtocolCodec 实例
    pub fn new() -> Self {
        YsProtocolCodec
    }
}

impl Default for YsProtocolCodec {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use ys_types::ObjectID;

    #[test]
    fn test_encode_decode_handshake_request() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let message = YsMessage::HandshakeRequest {
            version: 1,
            client_id: "test-client".to_string(),
            capabilities: vec!["batch".to_string(), "compress".to_string()],
        };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_handshake_response() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let message = YsMessage::HandshakeResponse {
            version: 1,
            server_id: "test-server".to_string(),
            accepted_capabilities: vec!["batch".to_string()],
        };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_get_object_request() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let object_id = ObjectID::new();
        let message = YsMessage::GetObjectRequest { object_id };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_get_object_response() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let object_id = ObjectID::new();
        let message =
            YsMessage::GetObjectResponse { object_id, object_type: "text".to_string(), data: Bytes::from("test data") };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_put_object_request() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let object_id = ObjectID::new();
        let message =
            YsMessage::PutObjectRequest { object_id, object_type: "binary".to_string(), data: Bytes::from(vec![1, 2, 3, 4]) };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_put_object_response() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let object_id = ObjectID::new();
        let message = YsMessage::PutObjectResponse { object_id, success: true };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_batch_get_object_request() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let object_ids = vec![ObjectID::new(), ObjectID::new(), ObjectID::new()];
        let message = YsMessage::BatchGetObjectRequest { object_ids };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_batch_get_object_response() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let objects = vec![
            (ObjectID::new(), "text".to_string(), Bytes::from("data1")),
            (ObjectID::new(), "binary".to_string(), Bytes::from("data2")),
        ];
        let message = YsMessage::BatchGetObjectResponse { objects };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_heartbeat() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let req_message = YsMessage::HeartbeatRequest;
        codec.encode(req_message.clone(), &mut dst).unwrap();
        let req_result = codec.decode(&mut dst).unwrap();
        assert_eq!(req_result, Some(req_message));

        let resp_message = YsMessage::HeartbeatResponse;
        codec.encode(resp_message.clone(), &mut dst).unwrap();
        let resp_result = codec.decode(&mut dst).unwrap();
        assert_eq!(resp_result, Some(resp_message));
    }

    #[test]
    fn test_encode_decode_error() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let message = YsMessage::Error { code: 404, message: "Object not found".to_string() };

        codec.encode(message.clone(), &mut dst).unwrap();

        let result = codec.decode(&mut dst).unwrap();
        assert_eq!(result, Some(message));
    }

    #[test]
    fn test_encode_decode_list_branches() {
        let mut codec = YsProtocolCodec::new();
        let mut dst = BytesMut::new();

        let req_message = YsMessage::ListBranchesRequest;
        codec.encode(req_message.clone(), &mut dst).unwrap();
        let req_result = codec.decode(&mut dst).unwrap();
        assert_eq!(req_result, Some(req_message));

        let branches = vec![("main".to_string(), ObjectID::new()), ("dev".to_string(), ObjectID::new())];
        let resp_message = YsMessage::ListBranchesResponse { branches };
        codec.encode(resp_message.clone(), &mut dst).unwrap();
        let resp_result = codec.decode(&mut dst).unwrap();
        assert_eq!(resp_result, Some(resp_message));
    }

    #[test]
    fn test_decode_incomplete_header() {
        let mut codec = YsProtocolCodec::new();
        let mut src = BytesMut::from(&[0x59, 0x53, 0x50][..]);

        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_invalid_magic() {
        let mut codec = YsProtocolCodec::new();
        let mut src = BytesMut::new();
        src.put_u32(0x12345678);
        src.put_u8(1);
        src.put_u8(0x01);
        src.put_u16(0);
        src.put_u32(0);

        let result = codec.decode(&mut src);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_incomplete_payload() {
        let mut codec = YsProtocolCodec::new();
        let mut src = BytesMut::new();
        src.put_u32(YS_MAGIC);
        src.put_u8(YS_PROTOCOL_VERSION);
        src.put_u8(0x01);
        src.put_u16(0);
        src.put_u32(100);

        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, None);
    }
}
