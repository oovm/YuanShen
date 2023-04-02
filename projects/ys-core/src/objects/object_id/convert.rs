
use super::*;

impl From<&Vec<u8>> for ObjectID {
    fn from(vec: &Vec<u8>) -> Self {
        ObjectID {
            hash256: blake3::hash(&vec),
        }
    }
}

impl From<&[u8]> for ObjectID {
    fn from(bytes: &[u8]) -> Self {
        ObjectID {
            hash256: blake3::hash(&bytes),
        }
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
        let f = File::options().read(true).open(p)?;
        ObjectID::try_from(f)
    }
}


impl Serialize for ObjectID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let b: &[u8] = self.hash256.as_bytes();
        b.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for ObjectID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;
        let v: Vec<u8> = s.into_bytes().iter().copied().collect();
        let mut bytes: [u8; 32] = [0; 32];
        for i in 0..32 {
            bytes[i] = v[i];
        }
        Ok(ObjectID {
            hash256: Hash::from(bytes),
        })
    }
}
