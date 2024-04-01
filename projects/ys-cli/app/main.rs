use clap::{ Parser, Subcommand};
use std::{env::current_dir, fmt::Debug, io::stdout};
use ys_core::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectStore, SnapShot, SnapShotDirectory, YsError,
};
use yuan_shen::*;

#[derive(Parser, Debug)]
struct YuanShen {
    #[clap(subcommand)]
    cmd: YsCommand,
}

#[derive(Debug, Subcommand)]
enum YsCommand {
    #[command(alias = "init")]
    Initialize(YuanShenInitialize),
    #[command(alias = "diff")]
    Difference(YuanShenDifference),
    Changes,
    Commit(YuanShenCommit),
    Squash(YuanShenSquash),
    Merge(YuanShenMerge),
    Rebase(YuanShenRebase),
    Reset(YuanShenReset),
    Orphan(YuanShenOrphan),
    Checkout(YuanShenCheckout),
    Branch(YuanShenBranch),
    Stash(YuanShenCommit),
    #[command(alias = "gc")]
    GarbageCollect,
    #[command(external_subcommand)]
    External(Vec<String>),
}

#[tokio::main]
pub async fn main() -> Result<(), YsError> {
    let args = YuanShen::parse();
    use YsCommand::*;
    match args.cmd {
        Initialize(init) => init.initialize().await?,
        Difference(diff) => diff.difference().await?,
        Branch(b) => b.branch().await?,
        Checkout(c) => c.checkout().await?,
        Changes => {
            let dir = current_dir()?;
            let dot_rev = DotYuanShenClient::open(&dir).unwrap();
            let mut store = dot_rev.store().unwrap();
            let branch: String = dot_rev.get_branch_name().unwrap();
            let old_tip: ObjectID = dot_rev.get_branch_id(&branch).unwrap();
            let ignores: IgnoreRules = dot_rev.ignores().unwrap();
            let directory = SnapShotDirectory::new(dir.as_path(), &ignores, &mut store).unwrap();
            let snapshot: SnapShot = store.get_typed(old_tip).await.unwrap();
            let old_directory: SnapShotDirectory = store.get_typed(snapshot.directory).await.unwrap();
            serde_json::to_writer_pretty(stdout(), &old_directory.difference(&directory)).unwrap();
        }
        Commit(sub) => sub.commit().await.unwrap(),
        Squash(_) => {}
        Merge(_) => {}
        Rebase(_) => {}
        Reset(_) => {}
        Orphan(_) => {}
        Stash(_) => {}
        External(_) => {}
        GarbageCollect => {}
    }
    Ok(())
}
