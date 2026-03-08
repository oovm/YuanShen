use tokio::net::TcpStream;
use ys_types::YsError;
use ys_storage::StorageBackend;
use crate::Gateway;

/// Perforce (P4) 网关实现，用于处理 P4 协议的连接和请求
pub struct P4Gateway<S> {
    store: S,
}

impl<S> P4Gateway<S> 
where 
    S: StorageBackend + 'static
{
    /// 创建一个新的 P4Gateway 实例
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// 处理 P4 协议的具体逻辑
    async fn process_p4_request(&self, _stream: &mut TcpStream) -> Result<(), YsError> {
        Err(YsError::not_implemented("P4Gateway::process_p4_request"))
    }
}

impl<S> Gateway for P4Gateway<S> 
where 
    S: StorageBackend + 'static
{
    fn name(&self) -> &'static str {
        "p4"
    }

    async fn handle(&self, mut stream: TcpStream) -> Result<(), YsError> {
        println!("Handling P4 connection");
        self.process_p4_request(&mut stream).await
    }
}
