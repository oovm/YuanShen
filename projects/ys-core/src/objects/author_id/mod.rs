use super::*;

#[derive(Eq, Debug, Clone)]
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
        self.hash256.partial_cmp(&other.hash256)
    }
}

impl Ord for AuthorID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hash256.partial_cmp(&other.hash256)
    }
}

impl Serialize for AuthorID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        todo!()
    }
}

impl<'de> Deserialize<'de> for AuthorID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        todo!()
    }
}

