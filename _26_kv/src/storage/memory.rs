use crate::{KvError, KvPair, Storage, StorageIter, Value};
use dashmap::DashMap;
use dashmap::mapref::one::Ref;

#[derive(Clone, Debug, Default)]
pub struct MemTable {
    pub(crate) tables: DashMap<String, DashMap<String, Value>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn get_or_create_table(&self, name: &str) -> Ref<String, DashMap<String, Value>> {
        match self.tables.get(name) {
            Some(table) => table,
            None => {
                let entry = self.tables.entry(name.into()).or_default();
                entry.downgrade()
            }
        }
    }
}

impl Storage for MemTable {
    fn get(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        let val = table.get(key).map(|v| v.value().clone());
        Ok(val)
    }

    fn set(
        &self,
        table: &str,
        key: impl Into<String>,
        value: impl Into<Value>,
    ) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.insert(key.into(), value.into()))
    }

    fn contains(&self, table: &str, key: &str) -> Result<bool, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.contains_key(key))
    }

    fn del(&self, table: &str, key: &str) -> Result<Option<Value>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(table.remove(key).map(|(_k, v)| v))
    }

    fn get_all(&self, table: &str) -> Result<Option<Vec<KvPair>>, KvError> {
        let table = self.get_or_create_table(table);
        Ok(Some(
            table
                .iter()
                .map(|v| KvPair::new(v.key(), v.value().clone()))
                .collect::<Vec<_>>(),
        ))
    }

    fn get_iter(&self, table: &str) -> Result<Box<dyn Iterator<Item = KvPair>>, KvError> {
        let table = self.get_or_create_table(table);
        // let iter = table
        //     .clone()
        //     .into_iter()
        //     .map(|v| v.into());
        let iter = StorageIter::new(table.clone().into_iter());
        // let iter = table.clone().into_iter().map(|v| v.into());

        Ok(Box::new(iter))
    }
}

impl From<(String, Value)> for KvPair {
    fn from(value: (String, Value)) -> Self {
        KvPair::new(value.0, value.1)
    }
}
