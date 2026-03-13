use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io;
use tokio_util::codec::{Decoder, Encoder};

/// Git pkt-line 协议包类型
///
/// 代表 Git pkt-line 协议中的各种数据包。
/// 参考 Git 协议文档：https://git-scm.com/docs/protocol-common
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PktLine {
    /// 普通数据包
    ///
    /// 包含实际的数据内容，长度格式为 4 位十六进制数 + 数据。
    Data(Bytes),
    /// flush-pkt: 0000
    ///
    /// 表示数据包流的结束，用于分隔不同的部分。
    Flush,
    /// delim-pkt: 0001
    ///
    /// 用于分隔数据包组，在 Git 协议 2.0 中使用。
    Delim,
    /// response-end-pkt: 0002
    ///
    /// 表示响应的结束，在 Git 协议 2.0 中使用。
    ResponseEnd,
}

/// Git pkt-line 编解码器
///
/// 实现 tokio_util 的 Decoder 和 Encoder trait，
/// 用于在字节流和 PktLine 之间进行转换。
pub struct PktLineCodec;

impl Decoder for PktLineCodec {
    type Item = PktLine;
    type Error = io::Error;

    /// 从字节流中解码出一个 PktLine
    ///
    /// # Arguments
    /// * `src` - 输入的字节流缓冲区
    ///
    /// # Returns
    /// * `Ok(Some(item))` - 成功解码出一个包
    /// * `Ok(None)` - 需要更多数据才能解码
    /// * `Err(err)` - 解码出错
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 4 {
            return Ok(None);
        }

        let len_str = std::str::from_utf8(&src[..4])
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid pkt-line length encoding"))?;

        let len = usize::from_str_radix(len_str, 16)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid pkt-line length hex"))?;

        match len {
            0 => {
                src.advance(4);
                Ok(Some(PktLine::Flush))
            }
            1 => {
                src.advance(4);
                Ok(Some(PktLine::Delim))
            }
            2 => {
                src.advance(4);
                Ok(Some(PktLine::ResponseEnd))
            }
            3 => Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid pkt-line length: 3")),
            _ => {
                if src.len() < len {
                    return Ok(None);
                }
                src.advance(4);
                let data = src.split_to(len - 4).freeze();
                Ok(Some(PktLine::Data(data)))
            }
        }
    }
}

impl Encoder<PktLine> for PktLineCodec {
    type Error = io::Error;

    /// 将 PktLine 编码到字节流中
    ///
    /// # Arguments
    /// * `item` - 要编码的 PktLine
    /// * `dst` - 输出的字节流缓冲区
    ///
    /// # Returns
    /// * `Ok(())` - 编码成功
    /// * `Err(err)` - 编码出错
    fn encode(&mut self, item: PktLine, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            PktLine::Flush => dst.put_slice(b"0000"),
            PktLine::Delim => dst.put_slice(b"0001"),
            PktLine::ResponseEnd => dst.put_slice(b"0002"),
            PktLine::Data(data) => {
                let len = data.len() + 4;
                if len > 65535 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Pkt-line too large"));
                }
                dst.put_slice(format!("{:04x}", len).as_bytes());
                dst.put(data);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    #[test]
    fn test_encode_flush() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        codec.encode(PktLine::Flush, &mut dst).unwrap();
        assert_eq!(dst.as_ref(), b"0000");
    }

    #[test]
    fn test_decode_flush() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("0000");
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::Flush));
        assert_eq!(src.len(), 0);
    }

    #[test]
    fn test_encode_delim() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        codec.encode(PktLine::Delim, &mut dst).unwrap();
        assert_eq!(dst.as_ref(), b"0001");
    }

    #[test]
    fn test_decode_delim() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("0001");
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::Delim));
        assert_eq!(src.len(), 0);
    }

    #[test]
    fn test_encode_response_end() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        codec.encode(PktLine::ResponseEnd, &mut dst).unwrap();
        assert_eq!(dst.as_ref(), b"0002");
    }

    #[test]
    fn test_decode_response_end() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("0002");
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::ResponseEnd));
        assert_eq!(src.len(), 0);
    }

    #[test]
    fn test_encode_data() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        let data = Bytes::from("hello");
        codec.encode(PktLine::Data(data.clone()), &mut dst).unwrap();
        assert_eq!(&dst[..4], b"0009");
        assert_eq!(&dst[4..], b"hello");
    }

    #[test]
    fn test_decode_data() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("0009hello");
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::Data(Bytes::from("hello"))));
        assert_eq!(src.len(), 0);
    }

    #[test]
    fn test_decode_incomplete_length() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("00");
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_incomplete_data() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("0009he");
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_decode_invalid_length_hex() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("zzzz");
        let result = codec.decode(&mut src);
        assert!(result.is_err());
    }

    #[test]
    fn test_decode_invalid_length_3() {
        let mut codec = PktLineCodec;
        let mut src = BytesMut::from("0003");
        let result = codec.decode(&mut src);
        assert!(result.is_err());
    }

    #[test]
    fn test_encode_max_length() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        let max_data_len = 65535 - 4;
        let data = Bytes::from(vec![0u8; max_data_len]);
        codec.encode(PktLine::Data(data.clone()), &mut dst).unwrap();
        assert_eq!(&dst[..4], b"ffff");
    }

    #[test]
    fn test_encode_too_large() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        let too_large_data = Bytes::from(vec![0u8; 65532]);
        let result = codec.encode(PktLine::Data(too_large_data), &mut dst);
        assert!(result.is_err());
    }

    #[test]
    fn test_roundtrip_flush() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        codec.encode(PktLine::Flush, &mut dst).unwrap();
        let mut src = BytesMut::from(dst.as_ref());
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::Flush));
    }

    #[test]
    fn test_roundtrip_delim() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        codec.encode(PktLine::Delim, &mut dst).unwrap();
        let mut src = BytesMut::from(dst.as_ref());
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::Delim));
    }

    #[test]
    fn test_roundtrip_response_end() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        codec.encode(PktLine::ResponseEnd, &mut dst).unwrap();
        let mut src = BytesMut::from(dst.as_ref());
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::ResponseEnd));
    }

    #[test]
    fn test_roundtrip_data() {
        let mut codec = PktLineCodec;
        let mut dst = BytesMut::new();
        let data = Bytes::from("test data 123");
        codec.encode(PktLine::Data(data.clone()), &mut dst).unwrap();
        let mut src = BytesMut::from(dst.as_ref());
        let result = codec.decode(&mut src).unwrap();
        assert_eq!(result, Some(PktLine::Data(data)));
    }
}
