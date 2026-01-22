use tokio::net::TcpStream;
use ys_types::YsError;
use ys_storage::StorageBackend;
use ys_gateway::Gateway;

pub struct P4Gateway<S> {
    store: S,
}

impl<S> P4Gateway<S> 
where 
    S: StorageBackend + 'static
{
    pub fn new(store: S) -> Self {
        Self { store }
    }

    /// 处理 P4 协议的具体逻辑
    async fn process_p4_request(&self, _stream: &mut TcpStream) -> Result<(), YsError> {
        // TODO: 实现 P4 协议解析与响应
        // P4 协议通常使用自己的二进制协议
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
