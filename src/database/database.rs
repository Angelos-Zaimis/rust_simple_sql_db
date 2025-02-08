use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::{File, OpenOptions}, io::{Read, Write}, sync::{Arc, Mutex}};

use super::{Row, Table};


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    tables: HashMap<String, Table>,
}

impl Database {
    /*Creates a new database instance */
    pub fn new() -> Self {
        Database {
            tables: HashMap::new(),
        }
    }

    /* Finds table */
    pub fn find_table(&mut self, table_name: &str) -> Option<&Table> {
        return self.tables.get(table_name);
    }

    /* Creates a new Table */
    pub fn create_table(&mut self, table_name: &str) -> bool {
        if self.tables.contains_key(table_name) {
            return false;
        }
        self.tables.insert(table_name.to_string(), Table::new(table_name));
        true
    }

    /* Inserts a row into a table */
    pub fn insert(&mut self, table_name: &str, row: Row) -> bool {
        self.tables
            .get_mut(table_name)
            .map(|table| table.insert(row))
            .unwrap_or(false)
    }

    /* Selects all rows from a table */
    pub fn select_all(&self, table_name: &str) -> Option<Vec<&Row>> {
        self.tables.get(table_name).map(|table| table.select_all())
    }

    /* Updates an existing row */
    pub fn update(&mut self, table_name: &str, id:u32, new_name: String, new_age: u32) -> bool {
        self.tables
        .get_mut(table_name)
        .map(|table| table.update(id, &new_name, new_age))
        .unwrap_or(false)
    }

    /* Delete an existing row */
    pub fn delete(&mut self, id:u32, table_name: &str) -> bool {
        self.tables
        .get_mut(table_name)
        .map(|table| table.delete(id))
        .unwrap_or(false)
    }

    /* Saves the databse to a binry file using Bincode */
    pub fn save_to_file(&self, filename: &str) -> Result<(), bincode::Error> {
        let encoded: Vec<u8> = bincode::serialize(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(&encoded)?;
        println!("Database saved successfully!");
        Ok(())
    }

    /* Loads the database from a binary file using Bincode */
    pub fn load_from_file(filename: &str) -> Self {
        if let Ok(mut file) = OpenOptions::new().read(true).open(filename) {
            let mut data = Vec::new();
            if file.read_to_end(&mut data).is_ok() {
                if let Ok(database) = bincode::deserialize(&data) {
                    return database;
                }
            }
        }
        Database::new()
    }
    
}

pub type SharedDatabase = Arc<Mutex<Database>>;