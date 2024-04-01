use std::{
    collections::{BTreeMap, BTreeSet},
    env::current_dir,
};
use ys_core::{
    differences::{DifferenceEntry, SnapShotDifference},
    DirectoryEntry, IgnoreRules, LocalObjectStore, MemoryObjectStore, ObjectID, ObjectStore, SnapShotDirectory,
};

#[test]
fn ready() {
    println!("it works!")
}

const YUAN_SHEN: &[u8] = "源神, 启动!".as_bytes();

#[tokio::test]
async fn test_memory_object_store() {
    let mut store = MemoryObjectStore::new();
    store.put(YUAN_SHEN).await.unwrap();

    assert!(store.has(YUAN_SHEN.into()).await.unwrap());
    assert_eq!(store.get(YUAN_SHEN.into()).await.unwrap(), Vec::from(YUAN_SHEN));
}

#[tokio::test]
async fn test_local_object_store() {
    let temp = tempfile::tempdir().unwrap();
    let mut store = LocalObjectStore::new(temp.path().into()).unwrap();
    store.put(YUAN_SHEN).await.unwrap();
    assert!(store.has(YUAN_SHEN.into()).await.unwrap());
    assert_eq!(store.get(YUAN_SHEN.into()).await.unwrap(), Vec::from(YUAN_SHEN));
}

#[test]
fn test_diff_display() {
    let diff_empty: SnapShotDifference =
        SnapShotDifference { deleted: BTreeSet::new(), added: BTreeMap::new(), modified: BTreeMap::new() };
    assert_eq!(diff_empty.to_string(), "");

    let deleted_foo = BTreeSet::from([String::from("foo")]);
    let added_bar: BTreeMap<String, DirectoryEntry> =
        vec![(String::from("bar"), DirectoryEntry::File(ObjectID::from(&vec![])))].into_iter().collect();

    let diff_1: SnapShotDifference =
        SnapShotDifference { deleted: BTreeSet::new(), added: added_bar.clone(), modified: BTreeMap::new() };
    assert_eq!(diff_1.to_string(), "A bar\n");

    let diff_2: SnapShotDifference =
        SnapShotDifference { deleted: deleted_foo.clone(), added: BTreeMap::new(), modified: BTreeMap::new() };
    assert_eq!(diff_2.to_string(), "D foo\n");

    let diff_3: SnapShotDifference =
        SnapShotDifference { deleted: deleted_foo.clone(), added: added_bar.clone(), modified: BTreeMap::new() };
    assert_eq!(diff_3.to_string(), ["A bar", "D foo", ""].join("\n"));

    let diff_4: SnapShotDifference = SnapShotDifference {
        deleted: deleted_foo.clone(),
        added: added_bar.clone(),
        modified: vec![(String::from("baz"), DifferenceEntry::File(ObjectID::from(&vec![])))].into_iter().collect(),
    };
    assert_eq!(diff_4.to_string(), ["A bar", "M baz", "D foo", ""].join("\n"));

    let diff_5: SnapShotDifference = SnapShotDifference {
        deleted: deleted_foo.clone(),
        added: added_bar.clone(),
        modified: vec![
            (String::from("a"), DifferenceEntry::Directory(Box::new(diff_2))),
            (String::from("baz"), DifferenceEntry::Directory(Box::new(diff_4))),
        ]
        .into_iter()
        .collect(),
    };
    assert_eq!(diff_5.to_string(), ["D a/foo", "A bar", "A baz/bar", "M baz/baz", "D baz/foo", "D foo", ""].join("\n"));
}

#[tokio::test]
#[ignore]
async fn test_directory() {
    let dir = current_dir().unwrap();
    let mut store = MemoryObjectStore::new();
    let codebase = SnapShotDirectory::new(
        dir.as_path(),
        &IgnoreRules { glob: vec![String::from(".git"), String::from(".ys"), String::from("target")].into_iter().collect() },
        &mut store,
    )
    .unwrap();
    let readme_path = String::from("README.md");
    assert!(codebase.root.get(&readme_path).is_some());
}
