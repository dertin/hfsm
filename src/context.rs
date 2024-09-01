use std::collections::HashMap;

pub trait HFSMContext {
    fn set(&mut self, key: &str, value: String);
    fn get(&self, key: &str) -> Option<&String>;
    fn remove(&mut self, key: &str);
}

#[derive(Debug)]
pub struct HashMapContext {
    data: HashMap<String, String>,
}

impl Default for HashMapContext {
    fn default() -> Self {
        Self::new()
    }
}

impl HashMapContext {
    pub fn new() -> Self {
        HashMapContext {
            data: HashMap::new(),
        }
    }
}

impl HFSMContext for HashMapContext {
    fn set(&mut self, key: &str, value: String) {
        self.data.insert(key.to_string(), value);
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
}
