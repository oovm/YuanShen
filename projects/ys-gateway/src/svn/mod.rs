use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use ys_types::YsError;
use ys_storage::StorageBackend;
use crate::Gateway;
use std::future::Future;
use std::pin::Pin;

/// Subversion (SVN) 网关实现，用于处理 SVN 协议的连接和请求
pub struct SvnGateway<S> {
    store: S,
}

/// SVN 协议项，用于表示 SVN 协议中的各种数据类型
#[derive(Debug, PartialEq, Eq)]
pub enum SvnItem {
    /// 数字类型
    Number(i64),
    /// 字符串类型
    String(Vec<u8>),
    /// 列表类型
    List(Vec<SvnItem>),
}

impl<S> SvnGateway<S> 
where 
    S: StorageBackend + 'static
{
    /// 创建一个新的 SvnGateway 实例
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// 从异步读取器中读取一个 SVN 协议项
    async fn read_item<R: AsyncReadExt + Unpin + Send>(reader: &mut R) -> Result<SvnItem, YsError> {
        let mut b = [0u8; 1];
        reader
            .read_exact(&mut b)
            .await
            .map_err(|e| YsError::external_error(e))?;

        Self::read_item_with_first_byte(reader, b[0]).await
    }

    /// 从异步读取器中读取一个 SVN 协议项，已知第一个字节
    fn read_item_with_first_byte<'a, R: AsyncReadExt + Unpin + Send + 'a>(
        reader: &'a mut R,
        first: u8,
    ) -> Pin<Box<dyn Future<Output = Result<SvnItem, YsError>> + Send + 'a>> {
        Box::pin(async move {
            match first {
                b'0'..=b'9' => {
                    let mut num = (first - b'0') as i64;
                    let mut b = [0u8; 1];
                    loop {
                        reader
                            .read_exact(&mut b)
                            .await
                            .map_err(|e| YsError::external_error(e))?;
                        if b[0] == b':' {
                            let len = num as usize;
                            let mut buf = vec![0u8; len];
                            reader
                                .read_exact(&mut buf)
                                .await
                                .map_err(|e| YsError::external_error(e))?;
                            reader
                                .read_exact(&mut b)
                                .await
                                .map_err(|e| YsError::external_error(e))?;
                            return Ok(SvnItem::String(buf));
                        } else if b[0] == b' ' {
                            return Ok(SvnItem::Number(num));
                        } else if b[0] >= b'0' && b[0] <= b'9' {
                            num = num * 10 + (b[0] - b'0') as i64;
                        } else {
                            return Err(YsError::invalid_object(format!(
                                "Unexpected character in SVN item: {}",
                                b[0] as char
                            )));
                        }
                    }
                }
                b'(' => {
                    let mut list = Vec::new();
                    loop {
                        let mut b = [0u8; 1];
                        reader
                            .read_exact(&mut b)
                            .await
                            .map_err(|e| YsError::external_error(e))?;
                        if b[0] == b')' {
                            let mut space = [0u8; 1];
                            reader
                                .read_exact(&mut space)
                                .await
                                .map_err(|e| YsError::external_error(e))?;
                            return Ok(SvnItem::List(list));
                        } else if b[0] == b' ' {
                            continue;
                        } else {
                            list.push(Self::read_item_with_first_byte(reader, b[0]).await?);
                        }
                    }
                }
                _ => Err(YsError::invalid_object(format!(
                    "Unexpected start of SVN item: {}",
                    first as char
                ))),
            }
        })
    }

    /// 将一个 SVN 协议项写入到异步写入器中
    fn write_item<'a, W: AsyncWriteExt + Unpin + Send + 'a>(
        writer: &'a mut W,
        item: &'a SvnItem,
    ) -> Pin<Box<dyn Future<Output = Result<(), YsError>> + Send + 'a>> {
        Box::pin(async move {
            match item {
                SvnItem::Number(n) => {
                    writer
                        .write_all(format!("{} ", n).as_bytes())
                        .await
                        .map_err(|e| YsError::external_error(e))?;
                }
                SvnItem::String(s) => {
                    writer
                        .write_all(format!("{}:", s.len()).as_bytes())
                        .await
                        .map_err(|e| YsError::external_error(e))?;
                    writer
                        .write_all(s)
                        .await
                        .map_err(|e| YsError::external_error(e))?;
                    writer
                        .write_all(b" ")
                        .await
                        .map_err(|e| YsError::external_error(e))?;
                }
                SvnItem::List(l) => {
                    writer
                        .write_all(b"( ")
                        .await
                        .map_err(|e| YsError::external_error(e))?;
                    for i in l {
                        Self::write_item(writer, i).await?;
                    }
                    writer
                        .write_all(b") ")
                        .await
                        .map_err(|e| YsError::external_error(e))?;
                }
            }
            Ok(())
        })
    }

    /// 处理 SVN 协议握手
    async fn handle_handshake(&self, stream: &mut TcpStream) -> Result<(), YsError> {
        let greeting = SvnItem::List(vec![
            SvnItem::String(b"success".to_vec()),
            SvnItem::List(vec![
                SvnItem::Number(2),
                SvnItem::Number(2),
                SvnItem::List(vec![]),
                SvnItem::List(vec![SvnItem::String(b"edit-pipeline".to_vec())]),
            ]),
        ]);
        Self::write_item(stream, &greeting).await?;
        stream.flush().await.map_err(|e| YsError::external_error(e))?;
        
        let _response = Self::read_item(stream).await?;
        
        let auth_greeting = SvnItem::List(vec![
            SvnItem::String(b"success".to_vec()),
            SvnItem::List(vec![
                SvnItem::List(vec![SvnItem::String(b"ANONYMOUS".to_vec())]),
                SvnItem::String(b"00000000-0000-0000-0000-000000000000".to_vec()),
            ]),
        ]);
        Self::write_item(stream, &auth_greeting).await?;
        stream.flush().await.map_err(|e| YsError::external_error(e))?;

        Ok(())
    }
}

impl<S> Gateway for SvnGateway<S> 
where 
    S: StorageBackend + 'static
{
    fn name(&self) -> &'static str {
        "svn"
    }

    async fn handle(&self, mut stream: TcpStream) -> Result<(), YsError> {
        if let Ok(addr) = stream.peer_addr() {
            println!("Handling SVN connection from {:?}", addr);
        }
        
        if let Err(e) = self.handle_handshake(&mut stream).await {
            eprintln!("SVN handshake error: {:?}", e);
            return Err(e);
        }
        
        println!("SVN handshake successful");
        
        Ok(())
    }
}
