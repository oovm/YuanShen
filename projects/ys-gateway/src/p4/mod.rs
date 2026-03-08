use crate::Gateway;
use std::{future::Future, pin::Pin};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};
use ys_storage::StorageBackend;
use ys_types::YsError;

/// Perforce (P4) 协议项，用于表示 P4 协议中的各种数据类型
#[derive(Debug, PartialEq, Eq)]
pub enum P4Item {
    /// 数字类型
    Number(i64),
    /// 字符串类型
    String(Vec<u8>),
    /// 列表类型
    List(Vec<P4Item>),
    /// 字典/映射类型
    Map(Vec<(P4Item, P4Item)>),
}

/// Perforce (P4) 网关实现，用于处理 P4 协议的连接和请求
pub struct P4Gateway<S> {
    /// 存储后端，用于访问存储的对象
    _store: S,
}

impl<S> P4Gateway<S>
where
    S: StorageBackend + 'static,
{
    /// 创建一个新的 P4Gateway 实例
    pub fn new(store: S) -> Self {
        Self { _store: store }
    }

    /// 从异步读取器中读取一个 P4 协议项
    async fn read_item<R: AsyncReadExt + Unpin + Send>(reader: &mut R) -> Result<P4Item, YsError> {
        let mut b = [0u8; 1];
        reader.read_exact(&mut b).await.map_err(YsError::external_error)?;

        Self::read_item_with_first_byte(reader, b[0]).await
    }

    /// 从异步读取器中读取一个 P4 协议项，已知第一个字节
    fn read_item_with_first_byte<'a, R: AsyncReadExt + Unpin + Send + 'a>(
        reader: &'a mut R,
        first: u8,
    ) -> Pin<Box<dyn Future<Output = Result<P4Item, YsError>> + Send + 'a>> {
        Box::pin(async move {
            match first {
                b'0'..=b'9' => {
                    let mut num = (first - b'0') as i64;
                    let mut b = [0u8; 1];
                    loop {
                        reader.read_exact(&mut b).await.map_err(YsError::external_error)?;
                        if b[0] == b':' {
                            let len = num as usize;
                            let mut buf = vec![0u8; len];
                            reader.read_exact(&mut buf).await.map_err(YsError::external_error)?;
                            reader.read_exact(&mut b).await.map_err(YsError::external_error)?;
                            return Ok(P4Item::String(buf));
                        }
                        else if b[0] == b' ' {
                            return Ok(P4Item::Number(num));
                        }
                        else if b[0] >= b'0' && b[0] <= b'9' {
                            num = num * 10 + (b[0] - b'0') as i64;
                        }
                        else {
                            return Err(YsError::invalid_object(format!("Unexpected character in P4 item: {}", b[0] as char)));
                        }
                    }
                }
                b'(' => {
                    let mut list = Vec::new();
                    loop {
                        let mut b = [0u8; 1];
                        reader.read_exact(&mut b).await.map_err(YsError::external_error)?;
                        if b[0] == b')' {
                            let mut space = [0u8; 1];
                            reader.read_exact(&mut space).await.map_err(YsError::external_error)?;
                            return Ok(P4Item::List(list));
                        }
                        else if b[0] == b' ' {
                            continue;
                        }
                        else {
                            list.push(Self::read_item_with_first_byte(reader, b[0]).await?);
                        }
                    }
                }
                b'{' => {
                    let mut map = Vec::new();
                    loop {
                        let mut b = [0u8; 1];
                        reader.read_exact(&mut b).await.map_err(YsError::external_error)?;
                        if b[0] == b'}' {
                            let mut space = [0u8; 1];
                            reader.read_exact(&mut space).await.map_err(YsError::external_error)?;
                            return Ok(P4Item::Map(map));
                        }
                        else if b[0] == b' ' {
                            continue;
                        }
                        else {
                            let key = Self::read_item_with_first_byte(reader, b[0]).await?;
                            let value = Self::read_item(reader).await?;
                            map.push((key, value));
                        }
                    }
                }
                _ => Err(YsError::invalid_object(format!("Unexpected start of P4 item: {}", first as char))),
            }
        })
    }

    /// 将一个 P4 协议项写入到异步写入器中
    fn write_item<'a, W: AsyncWriteExt + Unpin + Send + 'a>(
        writer: &'a mut W,
        item: &'a P4Item,
    ) -> Pin<Box<dyn Future<Output = Result<(), YsError>> + Send + 'a>> {
        Box::pin(async move {
            match item {
                P4Item::Number(n) => {
                    writer.write_all(format!("{} ", n).as_bytes()).await.map_err(YsError::external_error)?;
                }
                P4Item::String(s) => {
                    writer.write_all(format!("{}:", s.len()).as_bytes()).await.map_err(YsError::external_error)?;
                    writer.write_all(s).await.map_err(YsError::external_error)?;
                    writer.write_all(b" ").await.map_err(YsError::external_error)?;
                }
                P4Item::List(l) => {
                    writer.write_all(b"( ").await.map_err(YsError::external_error)?;
                    for i in l {
                        Self::write_item(writer, i).await?;
                    }
                    writer.write_all(b") ").await.map_err(YsError::external_error)?;
                }
                P4Item::Map(m) => {
                    writer.write_all(b"{ ").await.map_err(YsError::external_error)?;
                    for (k, v) in m {
                        Self::write_item(writer, k).await?;
                        Self::write_item(writer, v).await?;
                    }
                    writer.write_all(b"} ").await.map_err(YsError::external_error)?;
                }
            }
            Ok(())
        })
    }

    /// 处理 P4 协议握手
    async fn handle_handshake(&self, stream: &mut TcpStream) -> Result<(), YsError> {
        let greeting = P4Item::List(vec![
            P4Item::String(b"success".to_vec()),
            P4Item::List(vec![
                P4Item::Number(2),
                P4Item::Number(2),
                P4Item::List(vec![]),
                P4Item::List(vec![P4Item::String(b"p4-protocol".to_vec())]),
            ]),
        ]);
        Self::write_item(stream, &greeting).await?;
        stream.flush().await.map_err(YsError::external_error)?;

        let _response = Self::read_item(stream).await?;

        let auth_greeting = P4Item::List(vec![
            P4Item::String(b"success".to_vec()),
            P4Item::List(vec![
                P4Item::List(vec![P4Item::String(b"ANONYMOUS".to_vec())]),
                P4Item::String(b"00000000-0000-0000-0000-000000000000".to_vec()),
            ]),
        ]);
        Self::write_item(stream, &auth_greeting).await?;
        stream.flush().await.map_err(YsError::external_error)?;

        Ok(())
    }

    /// 命令处理循环，处理 P4 协议中的各种命令
    async fn command_loop(&self, stream: &mut TcpStream) -> Result<(), YsError> {
        loop {
            match Self::read_item(stream).await {
                Ok(item) => {
                    if let Err(e) = self.dispatch_command(stream, item).await {
                        eprintln!("Error handling P4 command: {:?}", e);
                        return Err(e);
                    }
                }
                Err(e) => {
                    eprintln!("Error reading P4 command: {:?}", e);
                    return Err(e);
                }
            }
        }
    }

    /// 分发命令到对应的处理函数
    async fn dispatch_command(&self, stream: &mut TcpStream, item: P4Item) -> Result<(), YsError> {
        if let P4Item::List(items) = item {
            if let Some(P4Item::String(cmd_bytes)) = items.first() {
                let cmd = String::from_utf8_lossy(cmd_bytes);
                match cmd.as_ref() {
                    "info" => self.handle_info(stream, &items[1..]).await,
                    "files" => self.handle_files(stream, &items[1..]).await,
                    "sync" => self.handle_sync(stream, &items[1..]).await,
                    "submit" => self.handle_submit(stream, &items[1..]).await,
                    _ => self.handle_unknown_command(stream, &cmd).await,
                }
            }
            else {
                self.handle_unknown_command(stream, "invalid-command").await
            }
        }
        else {
            self.handle_unknown_command(stream, "invalid-command").await
        }
    }

    /// 处理 info 命令，获取服务器信息
    async fn handle_info(&self, stream: &mut TcpStream, _args: &[P4Item]) -> Result<(), YsError> {
        let response = P4Item::List(vec![
            P4Item::String(b"success".to_vec()),
            P4Item::Map(vec![
                (P4Item::String(b"serverVersion".to_vec()), P4Item::String(b"YS-P4/1.0".to_vec())),
                (P4Item::String(b"serverAddress".to_vec()), P4Item::String(b"ys-p4-server".to_vec())),
            ]),
        ]);
        Self::write_item(stream, &response).await?;
        stream.flush().await.map_err(YsError::external_error)?;
        Ok(())
    }

    /// 处理 files 命令，列出仓库中的文件
    async fn handle_files(&self, stream: &mut TcpStream, _args: &[P4Item]) -> Result<(), YsError> {
        let response = P4Item::List(vec![P4Item::String(b"success".to_vec()), P4Item::List(vec![])]);
        Self::write_item(stream, &response).await?;
        stream.flush().await.map_err(YsError::external_error)?;
        Ok(())
    }

    /// 处理 sync 命令，同步仓库内容到工作区
    async fn handle_sync(&self, stream: &mut TcpStream, _args: &[P4Item]) -> Result<(), YsError> {
        let response = P4Item::List(vec![P4Item::String(b"success".to_vec()), P4Item::List(vec![])]);
        Self::write_item(stream, &response).await?;
        stream.flush().await.map_err(YsError::external_error)?;
        Ok(())
    }

    /// 处理 submit 命令，提交更改
    async fn handle_submit(&self, stream: &mut TcpStream, _args: &[P4Item]) -> Result<(), YsError> {
        let response = P4Item::List(vec![
            P4Item::String(b"success".to_vec()),
            P4Item::Map(vec![(P4Item::String(b"change".to_vec()), P4Item::Number(1))]),
        ]);
        Self::write_item(stream, &response).await?;
        stream.flush().await.map_err(YsError::external_error)?;
        Ok(())
    }

    /// 处理未知命令
    async fn handle_unknown_command(&self, stream: &mut TcpStream, cmd: &str) -> Result<(), YsError> {
        let response = P4Item::List(vec![
            P4Item::String(b"failure".to_vec()),
            P4Item::List(vec![
                P4Item::String(b"illegal".to_vec()),
                P4Item::String(format!("Unknown P4 command: {}", cmd).into_bytes()),
            ]),
        ]);
        Self::write_item(stream, &response).await?;
        stream.flush().await.map_err(YsError::external_error)?;
        Ok(())
    }

    /// 处理 P4 协议的具体逻辑
    async fn process_p4_request(&self, stream: &mut TcpStream) -> Result<(), YsError> {
        self.handle_handshake(stream).await?;
        self.command_loop(stream).await
    }
}

impl<S> Gateway for P4Gateway<S>
where
    S: StorageBackend + 'static,
{
    /// 获取网关的名称
    fn name(&self) -> &'static str {
        "p4"
    }

    /// 处理进入的 P4 协议连接
    async fn handle(&self, mut stream: TcpStream) -> Result<(), YsError> {
        if let Ok(addr) = stream.peer_addr() {
            println!("Handling P4 connection from {:?}", addr);
        }

        if let Err(e) = self.process_p4_request(&mut stream).await {
            eprintln!("P4 request error: {:?}", e);
            return Err(e);
        }

        Ok(())
    }
}
