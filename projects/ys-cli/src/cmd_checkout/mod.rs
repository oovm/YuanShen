use clap::Args;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    YsError,
};

/// 检出命令参数
///
/// 用于创建并切换到指定的分支
#[derive(Debug, Args)]
pub struct YuanShenCheckout {
    /// 要创建或切换到的分支名称
    branch: String,
}

impl YuanShenCheckout {
    /// 执行检出操作
    ///
    /// 如果指定的分支不存在则创建，然后切换到该分支
    pub async fn checkout(self) -> Result<(), YsError> {
        let here = current_dir()?;
        let ys = DotYuanShenClient::open(&here)?;
        ys.create_branch(&self.branch)?;
        ys.set_branch(&self.branch)
    }
}
