use super::*;

#[test]
fn test_try_from() -> Result<(), std::io::Error> {
    let object_id = ObjectID::try_from(std::fs::File::options().read(true).open("./src/lib.rs").unwrap())?;
    let object_id_prime = ObjectID::try_from(Path::new("./src/lib.rs"))?;
    assert_eq!(object_id, object_id_prime);
    Ok(())
}
