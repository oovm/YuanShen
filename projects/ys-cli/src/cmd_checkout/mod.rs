use clap::Args;
use std::env::current_dir;
use ys_core::{YsError};
use ys_core::initialize::{DotYuanShen, InitializeConfig};

#[derive(Debug, Args)]
pub struct YuanShenCheckout {
    #[clap(short, long)]
    branch: String,
}

impl YuanShenCheckout {
    pub async fn checkout(self) -> Result<(), YsError> {
        let dot_rev = DotYuanShen::open(current_dir().unwrap().join(".ys")).unwrap();
        if !dot_rev.branch_exists(&self.branch).unwrap() {
            dot_rev.create_branch(&self.branch).unwrap();
        }
        dot_rev.set_branch(&self.branch)
    }
}
