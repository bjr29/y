use std::collections::HashMap;
use crate::Value;

pub struct AppData {
    pub values: Vec<HashMap<String, Value>>,
    pub functions: HashMap<String, (usize, usize, Vec<String>)>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            values: vec![HashMap::new()],
            functions: HashMap::new(),
        }
    }
}
