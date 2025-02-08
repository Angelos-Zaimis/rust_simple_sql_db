
mod database;
mod server;

use std::sync::{Arc, Mutex};
use database::{Database, SharedDatabase};
use server::start_server;
use tokio::runtime::Runtime;

fn main() {
    let db: SharedDatabase = Arc::new(Mutex::new(Database::load_from_file("database.db"))); // ✅ Corrected
    let rt = Runtime::new().unwrap();
    rt.block_on(start_server(db));
}