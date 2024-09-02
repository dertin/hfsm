use std::{any::Any, collections::HashMap};

pub trait StateContext {
    fn set(&mut self, key: &'static str, value: StateData);
    fn get(&self, key: &str) -> Option<&StateData>;
    fn remove(&mut self, key: &str);
}

#[derive(Debug)]
pub enum StateData {
    Integer(i32),
    Text(String),
    Boolean(bool),
    Boxed(Box<dyn Any>),
}

impl StateData {
    pub fn as_i32(&self) -> Option<i32> {
        if let StateData::Integer(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        if let StateData::Text(ref s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let StateData::Boolean(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_any(&self) -> Option<&dyn Any> {
        if let StateData::Boxed(ref boxed) = self {
            Some(&**boxed)
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct InMemoryStateContext {
    data: HashMap<&'static str, StateData>,
}

impl Default for InMemoryStateContext {
    fn default() -> Self {
        Self::new()
    }
}

impl InMemoryStateContext {
    pub fn new() -> Self {
        InMemoryStateContext { data: HashMap::new() }
    }
}

impl StateContext for InMemoryStateContext {
    fn set(&mut self, key: &'static str, value: StateData) {
        self.data.insert(key, value);
    }

    fn get(&self, key: &str) -> Option<&StateData> {
        self.data.get(key)
    }

    fn remove(&mut self, key: &str) {
        self.data.remove(key);
    }
}
