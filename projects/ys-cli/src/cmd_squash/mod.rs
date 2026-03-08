use clap::Args;
use std::collections::BTreeSet;
use std::env::current_dir;
use ys_types::{
    initialize::{DotYuanShenClient, YuanShenClient},
    ObjectID, ObjectProxy, ObjectStore, Commit, SnapShotData, YsError,
};

#[derive(Debug, Args)]
pub struct YuanShenSquash {
    #[clap(short, long)]
    count: u64,
    #[clap(short, long)]
    message: String,
}

impl YuanShenSquash {
    pub async fn squash(self) -> Result<(), YsError> {
        let dir = current_dir()?;
        let dot_rev = DotYuanShenClient::open(&dir).unwrap();
        let mut store = ys_storage::LocalDotYuanShen::