mod memory;
mod sleddb;

pub use memory::*;
pub use sleddb::*;

use crate::{KvError, KvPair, Value};

pub trait Storage {
    /// 从一个 HashTable 里获取一个 key 的 value
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// 从一个 HashTable 里设置一个 key 的 value，返回旧的 value
    fn set(
        &self,
        table: &str,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, KvError>;

    /// 查看 HashTable 中是否有 key
    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError>;

    /// 从 HashTable 中删除一个 key
    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError>;

    /// 遍历 HashTable，返回所有 kv pair（这个接口不好）
    fn get_all(&self, table: &str) -> Result<Option<Vec<KvPair>>, KvError>;

    /// 遍历 HashTable，返回 kv pair 的 Iterator
    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError>;
}

/// 提供 Storage iterator，这样 trait 的实现者只需要
/// 把它们的 iterator 提供给 StorageIter，然后它们保证
/// next() 传出的类型实现了 Into<Kvpair> 即可
pub struct StorageIter<T> {
    data: T,
}

impl<T> StorageIter<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Iterator for StorageIter<T>
where
    T: Iterator,
    T::Item: Into<KvPair>,
{
    type Item = KvPair;

    fn next(&mut self) -> Option<Self::Item> {
        self.data.next().map(|v| v.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::storage;
    use crate::storage::memory::MemTable;
    use tempfile::tempdir;

    #[test]
    fn get_or_create_table_should_work() {
        let store = MemTable::new();
        assert!(!store.tables.contains_key("t1"));
        store.get_or_create_table("t1");
        assert!(store.tables.contains_key("t1"));
    }

    #[test]
    fn sleddb_get_all_should_work() {
        let dir = tempdir().unwrap();
        let store = SledDb::new(dir);
        test_get_all(store);
    }

    #[test]
    fn sleddb_iter_should_work() {
        let dir = tempdir().unwrap();
        let store = SledDb::new(dir);
        test_get_iter(store);
    }

    #[test]
    fn sleddb_basic_interface_should_work() {
        let dir = tempdir().unwrap();
        let store = SledDb::new(dir);
        test_basi_interface(&store);
    }

    fn test_basi_interface<T: Storage>(store: &T) {
        let v = store.set("t1", "hello", "world");
        assert!(v.unwrap().is_none());
        // 再次 set 同样的 key 会更新，并返回之前的值
        let v1 = store.set("t1", "hello", "world1");
        assert_eq!(v1.unwrap(), Some("world".into()));
    }

    fn test_get_all(store: impl Storage) {
        store.set("t2", "k1", "v1").unwrap();
        store.set("t2", "k2", "v2").unwrap();
        let mut data = store.get_all("t2").unwrap().unwrap();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                KvPair::new("k1", "v1".into()),
                KvPair::new("k2", "v2".into())
            ]
        )
    }

    fn test_get_iter(store: impl Storage) {
        store.set("t2", "k1", "v1").unwrap();
        store.set("t2", "k2", "v2").unwrap();
        let mut data: Vec<_> = store.get_iter("t2").unwrap().collect();
        data.sort_by(|a, b| a.partial_cmp(b).unwrap());
        assert_eq!(
            data,
            vec![
                KvPair::new("k1", "v1".into()),
                KvPair::new("k2", "v2".into())
            ]
        )
    }
}
