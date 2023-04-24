use blake3::Hash;
use std::fmt::Formatter;

pub(crate) trait WriteHashID {
    fn write_hash_id(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
}

impl WriteHashID for Hash {
    fn write_hash_id(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in self.as_bytes() {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}
