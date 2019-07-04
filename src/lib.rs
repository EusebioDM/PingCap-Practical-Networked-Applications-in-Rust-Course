use std::collections::HashMap;

#[derive(Default)]
pub struct KvStore {
    map: HashMap<String, String>,
}

impl KvStore {
    pub fn new() -> KvStore {
        KvStore {
            map: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String) {
        self.map.insert(key, value);
    }

    pub fn get(&mut self, key: String) -> Option<String> {
        let val = self.map.get(&key)?;

        Some(val.to_string())
    }

    pub fn remove(&mut self, key: String) {
        self.map.remove(&key);
    }
}
