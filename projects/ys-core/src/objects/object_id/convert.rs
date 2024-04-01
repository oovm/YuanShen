use super::*;

impl From<blake3::Hash> for ObjectID {
    fn from(value: blake3::Hash) -> Self {
        Self { hash256: value }
    }
}

impl From<&Vec<u8>> for ObjectID {
    fn from(vec: &Vec<u8>) -> Self {
        ObjectID { hash256: blake3::hash(&vec) }
    }
}

impl From<&[u8]> for ObjectID {
    fn from(bytes: &[u8]) -> Self {
        ObjectID { hash256: blake3::hash(&bytes) }
    }
}

impl TryFrom<File> for ObjectID {
    type Error = std::io::Error;

    fn try_from(mut f: File) -> Result<Self, Self::Error> {
        let mut vec = Vec::new();
        f.read_to_end(&mut vec)?;
        Ok((&vec).into())
    }
}

impl<'a> TryFrom<&Path> for ObjectID {
    type Error = std::io::Error;

    fn try_from(p: &Path) -> Result<Self, Self::Error> {
        ObjectID::try_from(File::options().read(true).open(p)?)
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
