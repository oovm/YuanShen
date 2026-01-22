use ys_types::objects::ObjectID;
use std::path::Path;
use std::fs::File;

#[test]
fn test_try_from() {
    // Note: this test assumes it is run from the project root (projects/ys-types)
    // where src/lib.rs exists.
    let object_id = ObjectID::try_from(File::options().read(true).open("src/lib.rs").unwrap()).unwrap();
    let object_id_prime = ObjectID::try_from(Path::new("src/lib.rs")).unwrap();
    assert_eq!(object_id, object_id_prime);
}
