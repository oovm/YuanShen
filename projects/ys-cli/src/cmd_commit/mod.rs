use std::env::current_dir;
use clap::Args;
use ys_core::{DotYuanShen, Ignores, InsertJson, ObjectID, SnapShot, SnapShotData, SnapShotDirectory, YsError};

#[derive(Debug, Args)]
pub struct YuanShenCommit {
    message: String,
}

impl YuanShenCommit {
    pub async fn run(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let rev_dir = dir.join(".ys");
        let dot_rev = DotYuanShen::open(rev_dir).unwrap();
        let mut store = dot_rev.store().unwrap();
        let branch: String = dot_rev.get_branch().unwrap();
        let old_tip: ObjectID = dot_rev.get_branch_snapshot_id(&branch).unwrap();
        let ignores: Ignores = dot_rev.ignores().unwrap();
        let directory = SnapShotDirectory::new(dir.as_path(), &ignores, &mut store).unwrap();
        let directory_id = store.insert_json(&directory).await.unwrap();
        let snap = SnapShot {
            directory: directory_id,
            previous: vec![old_tip].into_iter().collect(),
            data: SnapShotData { kind: 0, self.message },
        };
        let snap_id = store.insert_json(&snap).await.unwrap();
        dot_rev.set_branch_snapshot_id(&branch, snap_id)
    }
}
