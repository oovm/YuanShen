use std::env::current_dir;
use clap::Args;
use ys_core::{DotYuanShen, YsError};

#[derive(Debug, Args)]
pub struct YuanShenInitialize {}

impl YuanShenInitialize {
    pub async fn initialize(self) -> Result<DotYuanShen, YsError> {
        Ok(DotYuanShen::new(current_dir().unwrap().join(".ys")).await?)
    }
}