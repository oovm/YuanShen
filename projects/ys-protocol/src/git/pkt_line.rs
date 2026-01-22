use bytes::{Buf, BufMut, Bytes, BytesMut};
use tokio_util::codec::{Decoder, Encoder};
use std::io;

/// Git pkt-line 协议包类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PktLine {
    /// 普通数据包
    Data(Bytes),
    /// flush-pkt: 0000
    Flush,
    /// delim-pkt: 0001
    Delim,
    /// response-end-pkt: 0002
    ResponseEnd,
}

pub struct PktLineCodec;

impl Decoder for PktLineCodec {
    type Item = PktLine;
    type Error = io::Error;

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

    fn encode(&mut self, item: PktLine, dst: &mut BytesMut) -> Result<(), Self::Error> {
        match item {
            PktLine::Flush => dst.put_slice(b"0000"),
            PktLine::Delim => dst.put_slice(b"0001"),
            PktLine::ResponseEnd => dst.put_slice(b"0002"),
            PktLine::Data(data) => {
                let len = data.len() + 4;
                if len > 65524 {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "Pkt-line too large"));
                }
                dst.put_slice(format!("{:04x}", len).as_bytes());
                dst.put(data);
            }
        }
        Ok(())
    }
}
