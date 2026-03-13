use clap::Args;
use std::{borrow::Cow, env::current_dir};
use ys_types::{IgnoreRules, YsError, initialize::InitializeConfig};

/// 初始化命令参数
///
/// 用于初始化一个新的 YuanShen 仓库
#[derive(Debug, Args)]
pub struct YuanShenInitialize {
    /// 覆盖初始分支的名称
    #[clap(long, short = 'b')]
    initial_branch: Option<String>,
}

impl YuanShenInitialize {
    /// 执行初始化操作
    ///
    /// 在当前目录创建一个新的 YuanShen 仓库，包括初始化配置和创建初始分支
    pub async fn initialize(self) -> Result<(), YsError> {
        let config = InitializeConfig {
            current: current_dir()?,
            initial_branch: match self.initial_branch {
                Some(s) => Cow::Owned(s),
                None => Cow::Borrowed("master"),
            },
            ignores: IgnoreRules::default(),
        };
        config.generate().await?;
        Ok(())
    }
}
