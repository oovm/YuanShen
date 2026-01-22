use tokio::net::TcpStream;
use tokio_util::codec::Framed;
use futures::{StreamExt, SinkExt};
use ys_types::{YsError, traits::{ObjectProxy, BranchProxy}};
use ys_gateway::Gateway;
use ys_protocol::git::{PktLine, PktLineCodec};
use bytes::Bytes;

pub struct GitGateway<S> {
    store: S,
}

impl<S> GitGateway<S> 
where 
    S: ObjectProxy + BranchProxy + Send + Sync + 'static
{
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// 处理 Git 引用发现 (v1)
    async fn handle_ref_discovery(&self, framed: &mut Framed<TcpStream, PktLineCodec>) -> Result<(), YsError> {
        // 这里假设 BranchProxy 有类似 list_branches 的能力，或者我们通过其它方式获取
        // 实际上之前的 DatabaseObjectStore 有 list_branches，但 BranchProxy 接口没定义
        // 让我们在 BranchProxy 中增加获取所有分支的能力
        
        // 暂时假设我们只能获取当前分支或者硬编码几个分支进行测试
        // 理想情况下 BranchProxy 应该能列出所有分支
        
        let branches = vec![("main".to_string(), "0000000000000000000000000000000000000000".to_string())];
        
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

#[async_trait::async_trait]
impl<S> Gateway for GitGateway<S> 
where 
    S: ObjectProxy + BranchProxy + Send + Sync + 'static
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
