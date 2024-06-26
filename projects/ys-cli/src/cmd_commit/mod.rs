use clap::Args;
use std::env::current_dir;
use ys_core::{
    initialize::{DotYuanShenClient, YuanShenClient},
    IgnoreRules, ObjectID, ObjectStore, SnapShot, SnapShotData, SnapShotDirectory, YsError,
};

#[derive(Debug, Args)]
pub struct YuanShenCommit {
    #[clap(short, long)]
    message: String,
    #[clap(long)]
    author: Option<String>,
    #[clap(long)]
    data: Option<String>,
}

impl YuanShenCommit {
    pub async fn commit(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir).unwrap();
        let mut store = dot_rev.store().unwrap();
        let branch: String = dot_rev.get_branch_name().unwrap();
        let old_tip: ObjectID = dot_rev.get_branch_id(&branch)?;
        let ignores: IgnoreRules = dot_rev.ignores().unwrap();
        let directory = SnapShotDirectory::new(dir.as_path(), &ignores, &mut store).unwrap();
        let directory_id = store.put_typed(&directory).await.unwrap();
        let snap = SnapShot {
            directory: directory_id,
            previous: vec![old_tip].into_iter().collect(),
            data: SnapShotData { kind: 0, message: self.message, authors: Default::default() },
        };
        let snap_id = store.put_typed(&snap).await?;
        dot_rev.set_branch_snapshot_id(&branch, snap_id)
    }
}
