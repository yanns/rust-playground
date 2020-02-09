use std::collections::HashMap;

pub struct KvStore {
    values: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.values.insert(key, value);
        ()
    }

    pub fn get(&self, key: String) -> Option<String> {
        self.values.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) {
        self.values.remove(&key);
        ()
    }
}
