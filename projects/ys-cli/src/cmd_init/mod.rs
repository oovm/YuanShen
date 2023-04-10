use clap::Args;
use std::env::current_dir;
use ys_core::{DotYuanShen, YsError};

#[derive(Debug, Args)]
pub struct YuanShenInitialize {
    /// override the name of the initial branch
    #[clap(long, short = 'b')]
    initial_branch: Option<String>,
}

impl YuanShenInitialize {
    pub async fn initialize(self) -> Result<(), YsError> {
        DotYuanShen::new(current_dir().unwrap().join(".ys")).await?;
        Ok(())
    }
}
