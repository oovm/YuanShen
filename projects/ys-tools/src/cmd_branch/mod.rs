use clap::Args;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    YsError,
};

/// 分支管理命令
#[derive(Debug, Args)]
pub struct YuanShenBranch {
    #[arg(long)]
    contains: Option<String>,
    #[arg(long)]
    without: Option<String>,
    #[arg(long, short)]
    ignore_case: bool,
}

impl YuanShenBranch {
    /// 执行分支管理命令，列出所有分支并标记当前分支
    pub async fn branch(self) -> Result<(), YsError> {
        let here = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&here)?;
        let current_branch = dot_rev.get_branch_name()?;
        let mut branches = dot_rev.list_branches()?;
        
        branches.sort();
        
        for branch in branches {
            if branch == current_branch {
                println!("* {}", branch);
            } else {
                println!("  {}", branch);
            }
        }
        
        Ok(())
    }
}
