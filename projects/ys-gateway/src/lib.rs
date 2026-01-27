use tokio::net::TcpStream;
use ys_types::YsError;

/// Gateway trait 定义了服务端如何兼容不同的版本控制协议
#[allow(async_fn_in_trait)]
pub trait Gateway: Send + Sync {
    /// 获取网关的名称 (例如 "git", "svn", "p4")
    fn name(&self) -> &'static str;

    /// 处理进入的连接
    async fn handle(&self, stream: TcpStream) -> Result<(), YsError>;
}
