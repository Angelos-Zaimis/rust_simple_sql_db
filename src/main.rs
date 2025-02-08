mod database;
mod web_server;
mod tcp_server;

use std::sync::{Arc, Mutex};
use database::{Database, SharedDatabase};
use web_server::start_rest_api;
use tcp_server::start_tcp_server;
use tokio::select;

#[tokio::main]
async fn main() {
    let db: SharedDatabase = Arc::new(Mutex::new(Database::load_from_file("database.db")));

    print!("Starting both TCP and REST API servers...");

    select! {
        _ = start_tcp_server(db.clone()) => (),
        _ = start_rest_api(db.clone()) => (),
    }
}