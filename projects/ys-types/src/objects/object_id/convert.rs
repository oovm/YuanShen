use super::*;
use crate::traits::YuanShenObject;
use crate::errors::{YsError, YsErrorKind};
use uuid::Uuid;
use std::str::FromStr;
use std::io::Read;
use std::path::Path;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

impl<T: YuanShenObject> From<T> for ObjectID {
    fn from(value: T) -> Self {
        value.object_id()
    }
}

fn content_to_uuid(content: &[u8]) -> ObjectID {
    let namespace = Uuid::NAMESPACE_DNS;
    ObjectID(Uuid::new_v5(&namespace, content))
}

impl YuanShenObject for String {
    fn object_id(&self) -> ObjectID {
        content_to_uuid(self.as_bytes())
    }
}
impl<'a> YuanShenObject for &'a str {
    fn object_id(&self) -> ObjectID {
        content_to_uuid(self.as_bytes())
    }
}
impl YuanShenObject for Vec<u8> {
    fn object_id(&self) -> ObjectID {
        content_to_uuid(self)
    }
}
impl<'a> YuanShenObject for &'a [u8] {
    fn object_id(&self) -> ObjectID {
        content_to_uuid(self)
    }
}

impl TryFrom<std::fs::File> for ObjectID {
    type Error = std::io::Error;

    fn try_from(mut f: std::fs::File) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        f.read_to_end(&mut vec)?;
        Ok(content_to_uuid(&vec))
    }
}

impl<'a> TryFrom<&'a Path> for ObjectID {
    type Error = YsError;

    fn try_from(p: &'a Path) -> Result<Self, Self::Error> {
        let path = std::fs::File::options().read(true).open(p);
        let mut buffer = vec![];
        match path.and_then(|mut o| o.read_to_end(&mut buffer)) {
            Ok(_) => Ok(content_to_uuid(&buffer)),
            Err(e) => Err(YsError::path_error(e, p.to_path_buf())),
        }
    }
}

impl FromStr for ObjectID {
    type Err = YsError;

    fn from_str(s: &str) -> Result<Self, YsError> {
        match Uuid::from_str(s) {
            Ok(uuid) => Ok(ObjectID(uuid)),
            Err(e) => Err(YsErrorKind::InvalidObject { message: e.to_string() })?,
        }
    }
}
