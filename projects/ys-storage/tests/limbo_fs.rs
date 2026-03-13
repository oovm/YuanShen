#![cfg(feature = "limbo")]

use tempfile::tempdir;
use ys_storage::LimboFsStorage;
use ys_types::{BranchProxy, ObjectProxy, YuanShenObject};

#[tokio::test]
async fn test_limbo_fs_storage() {
    let dir = tempdir().unwrap();
    let storage = LimboFsStorage::new(dir.path().to_path_buf()).await.unwrap();

    // Test branch operations
    storage.set_branch_name("dev").await.unwrap();
    assert_eq!(storage.get_branch_name().await.unwrap(), "dev");

    let id = "test content".object_id();
    storage.set_branch_id("dev", id).await.unwrap();
    assert_eq!(storage.get_branch_id("dev").await.unwrap(), id);
    assert!(storage.branch_exists("dev").await.unwrap());

    let branches = storage.list_branches().await.unwrap();
    assert_eq!(branches.len(), 1);
    assert_eq!(branches[0].0, "dev");
    assert_eq!(branches[0].1, id);

    // Test object operations
    let text_file = storage.put_string("hello world").await.unwrap();
    assert!(storage.has(text_file.file_id).await.unwrap());
    assert_eq!(storage.get_string(text_file).await.unwrap(), "hello world");
}
