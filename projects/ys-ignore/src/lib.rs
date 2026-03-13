use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;

/// The set of file names which we will ignore at any level.
#[derive(PartialEq, Eq, Debug, Clone)]
pub struct IgnoreRules {
    pub glob: Cow<'static, str>,
}

impl Default for IgnoreRules {
    fn default() -> Self {
        IgnoreRules { glob: Cow::Borrowed(include_str!(".ys.ignore")) }
    }
}

impl Serialize for IgnoreRules {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.glob)
    }
}

impl<'de> Deserialize<'de> for IgnoreRules {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(IgnoreRules { glob: Cow::Owned(s) })
    }
}
