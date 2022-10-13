use std::collections::HashMap;
use crate::Value;

pub struct AppData {
    pub values: HashMap<String, Value>,
    pub functions: HashMap<String, (i32, i32)>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            functions: HashMap::new(),
        }
    }
}
