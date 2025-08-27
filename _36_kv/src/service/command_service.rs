use crate::service::CommandService;
use crate::{
    CommandResponse, Hdel, Hexist, Hget, Hgetall, Hmdel, Hmexist, Hmget, Hmset, Hset, KvError,
    Storage, Value,
};

impl CommandService for Hget {
    fn execute<T: Storage>(self, storage: &T) -> CommandResponse {
        match storage.get(&self.table, &self.key) {
            Ok(Some(res)) => res.into(),
            Ok(None) => KvError::NotFound(self.table, self.key).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmget {
    fn execute<T: Storage>(self, storage: &T) -> CommandResponse {
        self.keys
            .iter()
            .map(|key| match storage.get(&self.table, key) {
                Ok(Some(res)) => res.into(),
                _ => Value::default(),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hgetall {
    fn execute<T: Storage>(self, storage: &T) -> CommandResponse {
        match storage.get_all(&self.table) {
            Ok(Some(res)) => res.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hset {
    fn execute<T: Storage>(self, storage: &T) -> CommandResponse {
        match self.pair {
            Some(res) => match storage.set(&self.table, res.key, res.value.unwrap_or_default()) {
                Ok(Some(res)) => res.into(),
                Ok(None) => Value::default().into(),
                Err(e) => e.into(),
            },
            None => KvError::InvalidCommand(format!("{:?}", self)).into(),
        }
    }
}

impl CommandService for Hmset {
    fn execute<T: Storage>(self, storage: &T) -> CommandResponse {
        self.pairs
            .into_iter()
            .map(move |pair| {
                match storage.set(&self.table, &pair.key, pair.value.unwrap_or_default()) {
                    Ok(Some(res)) => res.into(),
                    _ => Value::default(),
                }
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hdel {
    fn execute<T: Storage>(self, storage: &T) -> CommandResponse {
        match storage.del(&self.table, &self.key) {
            Ok(Some(res)) => res.into(),
            Ok(None) => Value::default().into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmdel {
    fn execute<T: Storage>(self, storage: &T) -> CommandResponse {
        self.keys
            .iter()
            .map(|key| match storage.del(&self.table, key) {
                Ok(Some(res)) => res.into(),
                _ => Value::default(),
            })
            .collect::<Vec<_>>()
            .into()
    }
}

impl CommandService for Hexist {
    fn execute<T: Storage>(self, store: &T) -> CommandResponse {
        match store.contains(&self.table, &self.key) {
            Ok(v) => Value::from(v).into(),
            Err(e) => e.into(),
        }
    }
}

impl CommandService for Hmexist {
    fn execute<T: Storage>(self, store: &T) -> CommandResponse {
        self.keys
            .iter()
            .map(|key| match store.contains(&self.table, key) {
                Ok(v) => v.into(),
                _ => Value::default(),
            })
            .collect::<Vec<Value>>()
            .into()
    }
}
