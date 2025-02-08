use crate::database::{Row, SharedDatabase};
use serde_json::json;
use warp::{Rejection, Reply};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTableRequest {
    table_name: String,
}
pub async fn health_handler() -> impl Reply {
    warp::reply::json(&"OK")
}

//Find Table Handler
pub async fn find_table_handler(table_name: String, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    let table = db.find_table(&table_name);
    Ok(warp::reply::json(&json!({"status": "success", "data": table})))
}

// Create Table Handler
pub async fn create_table_handler(req: CreateTableRequest, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    
    if db.create_table(&req.table_name) {
        db.save_to_file("database.db").unwrap();
        Ok(warp::reply::json(&json!({"status": "created", "table": req.table_name})))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Table already exists"})))
    }
}

// Insert Row Handler
pub async fn insert_handler(table_name: String, row: Row, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    if db.insert(&table_name, row.clone()) {
        db.save_to_file("database.db").unwrap();
        Ok(warp::reply::json(&json!({"status": "inserted", "row": row})))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Row already exists"})))
    }
}

// Select All Rows Handler
pub async fn select_handler(table_name: String, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let db = db.lock().unwrap();
    let data = db.select_all(&table_name);
    Ok(warp::reply::json(&json!({"status": "success", "data": data})))
}

// Update Row Handler
pub async fn update_handler(table_name: String, id: u32, body: serde_json::Value, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();

    if let (Some(new_name), Some(new_age)) = (body["name"].as_str(), body["age"].as_u64()) {
        if db.update(&table_name, id, new_name.to_string(), new_age as u32) {
            db.save_to_file("database.db").unwrap();
            return Ok(warp::reply::json(&json!({"status": "updated", "id": id})));
        }
    }

    Ok(warp::reply::json(&json!({"status": "error", "message": "Update failed"})))
}

// Delete Row Handler
pub async fn delete_handler(table_name: String, id: u32, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    if db.delete(id, &table_name) {
        db.save_to_file("database.db").unwrap();
        Ok(warp::reply::json(&json!({"status": "deleted", "id": id})))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Row not found"})))
    }
}