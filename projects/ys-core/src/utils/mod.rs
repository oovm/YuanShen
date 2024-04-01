use crate::{YsError, YsErrorKind};
use blake3::Hash;
use serde::{Deserialize, Serialize};
use serde_json::{ser::PrettyFormatter, Serializer};
use std::{
    fmt::Formatter,
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

mod hasher;

pub trait WriteHashID {
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

pub fn read_json<A>(path: &Path) -> Result<A, YsError>
where
    A: for<'de> Deserialize<'de>,
{
    Ok(serde_json::from_reader(File::options().read(true).open(path)?)?)
}

pub fn from_json<A>(bytes: &[u8]) -> Result<A, YsError>
where
    A: for<'de> Deserialize<'de>,
{
    Ok(serde_json::from_slice(bytes)?)
}

pub fn write_json<A>(thing: &A, path: &Path) -> Result<(), YsError>
where
    A: Serialize,
{
    let file = File::options().write(true).create(true).open(path)?;
    let mut ser = Serializer::with_formatter(file, PrettyFormatter::with_indent(b"    "));
    Ok(thing.serialize(&mut ser)?)
}
pub fn vec_json<A>(thing: &A) -> Result<Vec<u8>, YsError>
where
    A: Serialize,
{
    let mut buffer = vec![];
    let mut ser = Serializer::with_formatter(&mut buffer, PrettyFormatter::with_indent(b"    "));
    thing.serialize(&mut ser)?;
    Ok(buffer)
}

pub fn hash_json<A>(thing: &A) -> Result<Hash, YsError>
where
    A: Serialize,
{
    let buffer = vec_json(thing)?;
    Ok(blake3::hash(&buffer))
}

pub fn truncate_write(path: PathBuf, bytes: &[u8]) -> Result<usize, YsError> {
    let open = File::options().write(true).truncate(true).open(&path);
    match open.and_then(|mut o| o.write(bytes)) {
        Ok(o) => Ok(o),
        Err(e) => {
            return Err(YsErrorKind::IO { error: e, path: Some(path.to_path_buf()) })?;
        }
    }
}

/// Create a test environment which returns the [Result<()>]
pub fn async_test<F>(future: F)
where
    F: std::future::Future<Output = Result<(), YsError>>,
{
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
    rt.block_on(async { future.await.unwrap() })
}
