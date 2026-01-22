use super::*;
use uuid::Uuid;

/// TenantID represents a unique identifier for a tenant (Org/Role context)
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TenantID(pub Uuid);

impl TenantID {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for TenantID {
    fn default() -> Self {
        Self(Uuid::nil())
    }
}

impl From<Uuid> for TenantID {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}
