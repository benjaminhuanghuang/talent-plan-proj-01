use std::collections::HashMap;

pub struct KvStore {
  map: HashMap<String, String>
}

impl KvStore {
    pub fn new() -> KvStore {
      let map: HashMap<String, String> = HashMap::new();
      KvStore {
        map
      }
    }

    pub fn set(&mut self, key: String, value: String) {
      self.map.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<String> {
      let value = self.map.get(&key);
      // convert Option<&str> to Option<String>
      value.map(String::from)
    }

    pub fn remove(&mut self, key: String) {
      self.map.remove(&key);
    }
}
