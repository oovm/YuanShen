use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use futures::{StreamExt, SinkExt};
use ys_types::{YsError, ObjectID};
use ys_storage::StorageBackend;
use crate::Gateway;
use ys_protocol::git::{PktLine, PktLineCodec};
use bytes::{Bytes, BytesMut, BufMut};
use std::collections::HashSet;

/// Git 网关实现，用于处理 Git 协议的连接和请求
pub struct GitGateway<S> {
    /// 存储后端，用于访问 Git 对象
    store: S,
}

impl<S> GitGateway<S> 
where 
    S: StorageBackend + 'static
{
    /// 创建一个新的 GitGateway 实例
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// 处理 Git 引用发现 (v1)
    async fn handle_ref_discovery(&self, framed: &mut Framed<TcpStream, PktLineCodec>) -> Result<(), YsError> {
        let branches = self.store.list_branches().await?;
        
        if branches.is_empty() {
            framed.send(PktLine::Flush).await.map_err(|e| YsError::external_error(e))?;
            return Ok(());
        }

        let mut first = true;
        for (name, id) in branches {
            let line = if first {
                first = false;
                format!("{} refs/heads/{}\0multi_ack side-band-64k agent=ys-git\n", id, name)
            } else {
                format!("{} refs/heads/{}\n", id, name)
            };
            framed.send(PktLine::Data(Bytes::from(line))).await.map_err(|e| YsError::external_error(e))?;
        }

        framed.send(PktLine::Flush).await.map_err(|e| YsError::external_error(e))?;
        Ok(())
    }

    /// 处理 Git upload-pack 协议的请求
    async fn handle_upload_pack(&self, framed: &mut Framed<TcpStream, PktLineCodec>) -> Result<(), YsError> {
        self.handle_ref_discovery(framed).await?;
        self.handle_want_have_negotiation(framed).await?;
        Ok(())
    }

    /// 处理客户端的 want/have 协商
    async fn handle_want_have_negotiation(&self, framed: &mut Framed<TcpStream, PktLineCodec>) -> Result<(), YsError> {
        let mut wants = HashSet::new();
        let mut haves = HashSet::new();

        while let Some(result) = framed.next().await {
            let pkt = result.map_err(|e| YsError::external_error(e))?;
            
            match pkt {
                PktLine::Flush => {
                    break;
                }
                PktLine::Data(data) => {
                    let line = String::from_utf8_lossy(&data).trim().to_string();
                    
                    if line.starts_with("want ") {
                        if let Some(id_str) = line.strip_prefix("want ") {
                            if let Ok(id) = ObjectID::from_bytes(id_str.as_bytes()) {
                                wants.insert(id);
                            }
                        }
                    } else if line.starts_with("have ") {
                        if let Some(id_str) = line.strip_prefix("have ") {
                            if let Ok(id) = ObjectID::from_bytes(id_str.as_bytes()) {
                                haves.insert(id);
                            }
                        }
                    } else if line == "done" {
                        break;
                    }
                }
                _ => {}
            }
        }

        self.send_objects(framed, &wants).await?;

        Ok(())
    }

    /// 发送请求的对象数据，使用 side-band-64k 协议
    async fn send_objects(&self, framed: &mut Framed<TcpStream, PktLineCodec>, wants: &HashSet<ObjectID>) -> Result<(), YsError> {
        framed.send(PktLine::Data(Bytes::from("NAK\n"))).await.map_err(|e| YsError::external_error(e))?;
        
        for &id in wants {
            if self.store.has(id).await? {
                self.send_object(framed, id).await?;
            }
        }

        Ok(())
    }

    /// 发送单个 Git 对象
    async fn send_object(&self, framed: &mut Framed<TcpStream, PktLineCodec>, id: ObjectID) -> Result<(), YsError> {
        let text_file = ys_types::objects::TextFile { file_id: id };
        let data = self.store.get_buffer(text_file).await?;
        
        let header = format!("blob {}\0", data.len());
        let mut buf = BytesMut::with_capacity(header.len() + data.len());
        buf.put_slice(header.as_bytes());
        buf.put_slice(&data);
        
        self.send_side_band_data(framed, &buf.freeze()).await?;
        
        Ok(())
    }

    /// 使用 side-band-64k 发送数据
    async fn send_side_band_data(&self, framed: &mut Framed<TcpStream, PktLineCodec>, data: &Bytes) -> Result<(), YsError> {
        const MAX_PACKET: usize = 65516;
        
        let mut offset = 0;
        while offset < data.len() {
            let chunk_size = std::cmp::min(MAX_PACKET, data.len() - offset);
            let mut packet = BytesMut::with_capacity(chunk_size + 1);
            packet.put_u8(1);
            packet.put_slice(&data[offset..offset + chunk_size]);
            
            framed.send(PktLine::Data(packet.freeze())).await.map_err(|e| YsError::external_error(e))?;
            
            offset += chunk_size;
        }
        
        Ok(())
    }
}

impl<S> Gateway for GitGateway<S> 
where 
    S: StorageBackend + 'static
{
    /// 获取网关的名称
    fn name(&self) -> &'static str {
        "git"
    }

    /// 处理进入的 Git 协议连接
    async fn handle(&self, socket: TcpStream) -> Result<(), YsError> {
        let mut framed = Framed::new(socket, PktLineCodec);

        if let Some(result) = framed.next().await {
            let pkt = result.map_err(|e| YsError::external_error(e))?;
            if let PktLine::Data(data) = pkt {
                let request = String::from_utf8_lossy(&data);
                
                if request.contains("git-upload-pack") {
                    self.handle_upload_pack(&mut framed).await?;
                }
            }
        }

        Ok(())
    }
}
