use warp::Filter;
use crate::database::{Database, Row, SharedDatabase, Table};
use serde_json::json;
use std::sync::{Arc, Mutex};
use warp::{Rejection, Reply};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateTableRequest {
    table_name: String,
}

pub async fn start_server(db: SharedDatabase) {

    let find_table_route = warp::post()
        .and(warp::path("find"))
        .and(warp::path::param::<String>())
        .and(with_db(db.clone()))
        .and_then(find_table_handler);

    // Create Table Route
    let create_table_route = warp::post()
        .and(warp::path("create"))
        .and(warp::body::json()) // Expect JSON request body
        .and(with_db(db.clone()))
        .and_then(create_table_handler);

    // Insert Row Route
    let insert_route = warp::post()
        .and(warp::path("insert"))
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(insert_handler);

    // Select All Rows Route
    let select_route = warp::get()
        .and(warp::path("select"))
        .and(warp::path::param::<String>())
        .and(with_db(db.clone()))
        .and_then(select_handler);

    // Update Row Route
    let update_route = warp::put()
        .and(warp::path("update"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<u32>())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(update_handler);

    // Delete Row Route
    let delete_route = warp::delete()
        .and(warp::path("delete"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<u32>())
        .and(with_db(db.clone()))
        .and_then(delete_handler);

    // Combine all routes
    let routes = create_table_route
        .or(find_table_route)
        .or(insert_route)
        .or(select_route)
        .or(update_route)
        .or(delete_route);

    // Start the server
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Middleware for passing database reference
fn with_db(db: SharedDatabase) -> impl Filter<Extract = (SharedDatabase,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

//Find Table Handler
async fn find_table_handler(table_name: String, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    let table = db.find_table(&table_name);
    Ok(warp::reply::json(&json!({"status": "success", "data": table})))
}

// Create Table Handler
async fn create_table_handler(req: CreateTableRequest, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    
    if db.create_table(&req.table_name) {
        db.save_to_file("database.db").unwrap();
        Ok(warp::reply::json(&json!({"status": "created", "table": req.table_name})))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Table already exists"})))
    }
}

// Insert Row Handler
async fn insert_handler(table_name: String, row: Row, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    if db.insert(&table_name, row.clone()) {
        db.save_to_file("database.db").unwrap();
        Ok(warp::reply::json(&json!({"status": "inserted", "row": row})))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Row already exists"})))
    }
}

// Select All Rows Handler
async fn select_handler(table_name: String, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let db = db.lock().unwrap();
    let data = db.select_all(&table_name);
    Ok(warp::reply::json(&json!({"status": "success", "data": data})))
}

// Update Row Handler
async fn update_handler(table_name: String, id: u32, body: serde_json::Value, db: SharedDatabase) -> Result<impl Reply, Rejection> {
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
async fn delete_handler(table_name: String, id: u32, db: SharedDatabase) -> Result<impl Reply, Rejection> {
    let mut db = db.lock().unwrap();
    if db.delete(id, &table_name) {
        db.save_to_file("database.db").unwrap();
        Ok(warp::reply::json(&json!({"status": "deleted", "id": id})))
    } else {
        Ok(warp::reply::json(&json!({"status": "error", "message": "Row not found"})))
    }
}
