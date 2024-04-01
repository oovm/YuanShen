use super::*;
use crate::traits::YuanShenObject;

impl From<blake3::Hash> for ObjectID {
    fn from(value: blake3::Hash) -> Self {
        Self { hash256: value }
    }
}

impl<T: YuanShenObject> From<T> for ObjectID {
    fn from(value: T) -> Self {
        value.object_id()
    }
}

impl YuanShenObject for String {
    fn object_id(&self) -> ObjectID {
        ObjectID { hash256: blake3::hash(self.as_bytes()) }
    }
}
impl<'a> YuanShenObject for &'a str {
    fn object_id(&self) -> ObjectID {
        ObjectID { hash256: blake3::hash(self.as_bytes()) }
    }
}
impl YuanShenObject for Vec<u8> {
    fn object_id(&self) -> ObjectID {
        ObjectID { hash256: blake3::hash(self) }
    }
}
impl<'a> YuanShenObject for &'a [u8] {
    fn object_id(&self) -> ObjectID {
        ObjectID { hash256: blake3::hash(self) }
    }
}

impl TryFrom<std::fs::File> for ObjectID {
    type Error = std::io::Error;

    fn try_from(mut f: std::fs::File) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        f.read_to_end(&mut vec)?;
        Ok((&vec).object_id())
    }
}

impl<'a> TryFrom<&Path> for ObjectID {
    type Error = std::io::Error;

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        ObjectID::try_from(std::fs::File::options().read(true).open(p)?)
    }
}

impl Serialize for ObjectID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.hash256.to_string().serialize(serializer)
    }
}

impl FromStr for ObjectID {
    type Err = YsError;

    fn from_str(s: &str) -> Result<Self, YsError> {
        match blake3::Hash::from_hex(&s) {
            Ok(hash256) => Ok(hash256.into()),
            Err(e) => Err(YsErrorKind::InvalidObject { message: e.to_string() })?,
        }
    }
}

impl<'de> Deserialize<'de> for ObjectID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match blake3::Hash::from_hex(&s) {
            Ok(hash256) => Ok(hash256.into()),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    }
}
