use crate::{
    objects::{ObjectID, tenant_id::TenantID},
    traits::YuanShenObject,
    utils::hash_json,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct SnapShotData {
    pub kind: u64,
    pub message: String,
    pub tenants: BTreeSet<TenantID>,
}

/// 快照
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Commit {
    pub tree: ObjectID,
    pub parents: BTreeSet<ObjectID>,
    pub extra: SnapShotData,
}

impl YuanShenObject for Commit {
    fn object_id(&self) -> ObjectID {
        hash_json(self).unwrap().into()
    }
}
