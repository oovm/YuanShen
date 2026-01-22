use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use futures::{StreamExt, SinkExt};
use ys_types::YsError;
use ys_storage::StorageBackend;
use ys_gateway::Gateway;
use ys_protocol::git::{PktLine, PktLineCodec};
use bytes::Bytes;

pub struct GitGateway<S> {
    store: S,
}

impl<S> GitGateway<S> 
where 
    S: StorageBackend + 'static
{
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
}

impl<S> Gateway for GitGateway<S> 
where 
    S: StorageBackend + 'static
{
    fn name(&self) -> &'static str {
        "git"
    }

    async fn handle(&self, socket: TcpStream) -> Result<(), YsError> {
        let mut framed = Framed::new(socket, PktLineCodec);

        if let Some(result) = framed.next().await {
            let pkt = result.map_err(|e| YsError::external_error(e))?;
            if let PktLine::Data(data) = pkt {
                let request = String::from_utf8_lossy(&data);
                println!("Git request: {}", request);
                
                if request.contains("git-upload-pack") {
                    self.handle_ref_discovery(&mut framed).await?;
                }
            }
        }

        Ok(())
    }
}
