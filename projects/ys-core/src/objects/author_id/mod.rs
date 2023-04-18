use super::*;

#[derive(Copy, Clone, Debug, Eq)]
pub struct AuthorID {
    hash256: Hash,
}

impl PartialEq for AuthorID {
    fn eq(&self, other: &Self) -> bool {
        self.hash256.eq(&other.hash256)
    }
}

impl PartialOrd for AuthorID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hash256.as_bytes().partial_cmp(&other.hash256.as_bytes())
    }
}

impl Ord for AuthorID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash256.as_bytes().cmp(&other.hash256.as_bytes())
    }
}

impl Serialize for AuthorID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.hash256.as_bytes().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for AuthorID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        match String::deserialize(deserializer) {
            Ok(_) => {
                todo!()
            }
            Err(_) => {
                todo!()
            }
        }
    }
}
