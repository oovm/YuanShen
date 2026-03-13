use tokio::net::{TcpListener, TcpStream};
use ys_gateway::Gateway;
use ys_storage::StorageBackend;
use ys_types::YsError;

/// Server 是 YuanShen 系统的服务端入口
///
/// Server 负责协调网关（Gateway）和存储（StorageBackend），提供版本控制服务的
/// 高层接口。它支持与不同的版本控制系统（如 Git、SVN、P4）兼容。
pub struct Server<S, G>
where
    S: StorageBackend,
    G: Gateway,
{
    storage: S,
    gateway: G,
}

impl<S, G> Server<S, G>
where
    S: StorageBackend,
    G: Gateway,
{
    /// 创建一个新的 Server 实例
    ///
    /// # 参数
    /// - `storage`: 存储后端实现，用于持久化数据
    /// - `gateway`: 网关实现，用于兼容特定版本控制系统协议
    ///
    /// # 返回值
    /// 返回一个新的 Server 实例
    pub fn new(storage: S, gateway: G) -> Self {
        Self { storage, gateway }
    }

    /// 获取当前使用的网关名称
    ///
    /// # 返回值
    /// 返回网关的名称，例如 "git"、"svn" 或 "p4"
    pub fn gateway_name(&self) -> &'static str {
        self.gateway.name()
    }

    /// 处理单个 TCP 连接
    ///
    /// # 参数
    /// - `stream`: 传入的 TCP 连接流
    ///
    /// # 错误
    /// 当处理连接失败时返回 YsError
    pub async fn handle_connection(&self, stream: TcpStream) -> Result<(), YsError> {
        self.gateway.handle(stream).await
    }

    /// 在指定地址上启动服务器并监听连接
    ///
    /// # 参数
    /// - `addr`: 服务器监听的地址，格式为 "host:port"
    ///
    /// # 错误
    /// 当启动服务器失败时返回 YsError
    pub async fn listen(&self, addr: &str) -> Result<(), YsError> {
        let listener = TcpListener::bind(addr).await?;

        loop {
            let (stream, _) = listener.accept().await?;
            self.handle_connection(stream).await?;
        }
    }

    /// 获取存储后端的引用
    ///
    /// # 返回值
    /// 返回存储后端的不可变引用
    pub fn storage(&self) -> &S {
        &self.storage
    }

    /// 获取存储后端的可变引用
    ///
    /// # 返回值
    /// 返回存储后端的可变引用
    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }

    /// 获取网关的引用
    ///
    /// # 返回值
    /// 返回网关的不可变引用
    pub fn gateway(&self) -> &G {
        &self.gateway
    }

    /// 获取网关的可变引用
    ///
    /// # 返回值
    /// 返回网关的可变引用
    pub fn gateway_mut(&mut self) -> &mut G {
        &mut self.gateway
    }
}
