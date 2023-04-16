#![feature(fs_try_exists)]

use std::{env::current_dir, fmt::Debug, io::stdout};
use clap::{Args, Parser, Subcommand};
use ys_core::{ Ignores,  ObjectID, SnapShot, SnapShotDirectory, YsError};
use ys_core::initialize::{DotYuanShen, InitializeConfig, InsertJson};
pub use crate::{
    cmd_checkout::YuanShenCheckout, cmd_commit::YuanShenCommit, cmd_diff::YuanShenDifference, cmd_init::YuanShenInitialize,
};

mod cmd_checkout;
mod cmd_commit;
mod cmd_diff;
mod cmd_init;

#[derive(Parser, Debug)]
struct YuanShen {
    #[clap(subcommand)]
    cmd: YsCommand,
}

#[derive(Subcommand, Debug)]
enum YsCommand {
    /// 创建一个新的源神项目
    #[command(alias = "init", alias = "启动!")]
    Initialize(YuanShenInitialize),
    Difference(YuanShenDifference),
    Changes,
    #[command(alias = "提交")]
    Commit(YuanShenCommit),
    #[command(alias = "塌缩")]
    Squash(YuanShenCommit),
    #[command(alias = "收束")]
    Merge(YuanShenCommit),
    #[command(alias = "干涉")]
    Rebase(YuanShenCommit),
    #[command(alias = "回溯")]
    Reverse(YuanShenCommit),
    #[command(alias = "退相干")]
    Orphan(YuanShenOrphan),
    #[command(alias = "跃迁")]
    Checkout(YuanShenCheckout),
    #[command(alias = "观测")]
    Stash(YuanShenCommit),
    Branch,
    #[command(external_subcommand)]
    External(Vec<String>),
}


#[derive(Debug, Args)]
pub struct YuanShenOrphan {
    
}

#[tokio::main]
pub async fn main() -> Result<(), YsError> {
    let args = YuanShen::parse();
    use YsCommand::*;
    match args.cmd {
        Initialize(init) => init.initialize().await?,
        Difference(diff) => diff.difference().await?,
        Branch => {
            let dot_rev = DotYuanShen::open(current_dir().unwrap().join(".ys")).unwrap();
            let branch = dot_rev.get_branch().unwrap();
            println!("{}", branch);
        }
        Checkout(c) => c.checkout().await?,
        Changes => {
            let dir = current_dir().unwrap();
            let rev_dir = dir.join(".ys");
            let dot_rev = DotYuanShen::open(rev_dir).unwrap();
            let mut store = dot_rev.store().unwrap();
            let branch: String = dot_rev.get_branch().unwrap();
            let old_tip: ObjectID = dot_rev.get_branch_snapshot_id(&branch).unwrap();
            let ignores: Ignores = dot_rev.ignores().unwrap();
            let directory = SnapShotDirectory::new(dir.as_path(), &ignores, &mut store).unwrap();
            let snapshot: SnapShot = store.read_json(old_tip).await.unwrap();
            let old_directory: SnapShotDirectory = store.read_json(snapshot.directory).await.unwrap();
            serde_json::to_writer_pretty(stdout(), &old_directory.difference(&directory)).unwrap();
        }
        Commit(sub) => sub.commit().await.unwrap(),
        Squash(_) => {}
        Merge(_) => {}
        Rebase(_) => {}
        Reverse(_) => {}
        Orphan(_) => {}
        Stash(_) => {}
        External(_) => {}
    }
    Ok(())
}
