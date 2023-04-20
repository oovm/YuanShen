use blake3::Hash;

#[derive(Copy, Clone, Debug)]
pub struct TreeID {
    hash256: Hash,
}
