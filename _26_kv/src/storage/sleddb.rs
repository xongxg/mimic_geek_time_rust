use crate::{KvError, KvPair, Storage, StorageIter, Value};
use sled::{Db, Error, IVec};
use std::path::Path;

#[derive(Debug)]
pub struct SledDb(Db);

impl SledDb {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self(sled::open(path).unwrap())
    }

    // 在 sleddb 里，因为它可以 scan_prefix，我们用 prefix
    // 来模拟一个 table。当然，还可以用其它方案。
    fn get_full_key(table: &str, key: &str) -> String {
        format!("{}:{}", table, key)
    }

    // 遍历 table 的 key 时，我们直接把 prefix: 当成 table
    fn get_table_prefix(table: &str) -> String {
        format!("{}:", table)
    }
}

/// 把 Option<Result<T, E>> flip 成 Result<Option<T>, E>
/// 从这个函数里，你可以看到函数式编程的优雅
fn flip<T, E>(x: Option<Result<T, E>>) -> Result<Option<T>, E> {
    x.map_or(Ok(None), |v| v.map(Some))
}

impl Storage for SledDb {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let name = Self::get_full_key(table, key);
        let result = self
            .0
            .get(name.as_bytes())
            .unwrap()
            .map(|v| v.as_ref().try_into());
        flip(result)
    }

    fn set(
        &self,
        table: &str,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, KvError> {
        let key = key.into();
        let name = Self::get_full_key(table, &key);
        let data: Vec<u8> = value.into().try_into().unwrap();

        let res = self
            .0
            .insert(name.as_bytes(), data)
            .unwrap()
            .map(|v| v.as_ref().try_into());
        flip(res)
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let name = Self::get_full_key(table, key);
        Ok(self.0.contains_key(name.as_bytes()).unwrap())
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let name = Self::get_full_key(table, key);
        let res = self
            .0
            .remove(name.as_bytes())
            .unwrap()
            .map(|v| v.as_ref().try_into());
        flip(res)
    }

    fn get_all(&self, table: &str) -> Result<Option<Vec<KvPair>>, KvError> {
        let prefix = Self::get_table_prefix(table);
        let res = self
            .0
            .scan_prefix(prefix.as_bytes())
            .map(|v| v.into())
            .collect::<Vec<KvPair>>();

        Ok(Some(res))
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError> {
        let prefix = Self::get_table_prefix(table);
        let res = self.0.scan_prefix(prefix.as_bytes());
        let iter = StorageIter::new(res);
        Ok(Box::new(iter))
    }
}

impl From<Result<(IVec, IVec), sled::Error>> for KvPair {
    fn from(value: Result<(IVec, IVec), Error>) -> Self {
        match value {
            Ok((k, v)) => match v.as_ref().try_into() {
                Ok(v) => KvPair::new(ivec_to_key(k.as_ref()), v),
                Err(_) => KvPair::default(),
            },
            Err(_) => KvPair::default(),
        }
    }
}

fn ivec_to_key(ivec: &[u8]) -> &str {
    let s = str::from_utf8(ivec).unwrap();
    let mut iter = s.split(':');
    iter.next();
    iter.next().unwrap()
}
