use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::Row;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Table {
    pub name: String,
    pub columns: Vec<String>,
    pub rows: HashMap<u32, Row>
}

impl Table {
    pub fn new(name: &str) -> Self {
        Table {
            name: name.to_string(),
            columns: vec!["id".to_string(), "name".to_string(), "age".to_string()],
            rows: HashMap::new(),
        }
    }

    pub fn insert(&mut self, row: Row) -> bool {
        if self.rows.contains_key(&row.id) {
            return false;
        }
        self.rows.insert(row.id, row);
        true
    }

    pub fn select_all(&self) -> Vec<&Row> {
        self.rows.values().collect()
    }

    pub fn update(&mut self, id: u32, name: &str, age: u32) -> bool {
        if let Some(row) = self.rows.get_mut(&id) {
            row.name = name.to_string();
            row.age = age;
            return true;
        }
        false
    }

    pub fn delete(&mut self, id: u32) -> bool {
        self.rows.remove(&id).is_some()
    }
}