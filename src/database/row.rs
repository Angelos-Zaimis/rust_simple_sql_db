use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Row {
    pub id: u32,
    pub name: String,
    pub age: u32,
}