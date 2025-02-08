use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Row {
    pub id: u32,
    pub name: String,
    pub age: u32,
}

impl Row {

    pub fn new(id: u32, name: &str, age: u32) -> Self {
        Row {
            id,
            name: name.to_string(),
            age,
        }
    }
}