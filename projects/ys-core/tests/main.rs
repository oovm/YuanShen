use ys_core::{LocalObjectStore, MemoryObjectStore, ObjectStore};

#[test]
fn ready() {
    println!("it works!")
}

const YUAN_SHEN: &[u8] = "源神, 启动!".as_bytes();

#[tokio::test]
async fn test_memory_object_store() {
    let mut store = MemoryObjectStore::new();
    store.insert(YUAN_SHEN).await.unwrap();

    assert!(store.has(YUAN_SHEN.into()).await.unwrap());
    assert_eq!(store.read(YUAN_SHEN.into()).await.unwrap(), Vec::from(YUAN_SHEN));
}


#[tokio::test]
async fn test_local_object_store() {
    let temp = tempfile::tempdir().unwrap();
    let mut store = LocalObjectStore::new(temp.path().into()).unwrap();
    store.insert(YUAN_SHEN).await.unwrap();
    assert!(store.has(YUAN_SHEN.into()).await.unwrap());
    assert_eq!(store.read(YUAN_SHEN.into()).await.unwrap(), Vec::from(YUAN_SHEN));
}
