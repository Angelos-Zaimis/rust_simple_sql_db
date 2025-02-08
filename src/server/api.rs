use warp::Filter;
use crate::database::{SharedDatabase};
use super::handler;

pub async fn start_server(db: SharedDatabase) {

    let find_table_route = warp::post()
        .and(warp::path("find"))
        .and(warp::path::param::<String>())
        .and(with_db(db.clone()))
        .and_then(handler::find_table_handler);

    // Create Table Route
    let create_table_route = warp::post()
        .and(warp::path("create"))
        .and(warp::body::json()) // Expect JSON request body
        .and(with_db(db.clone()))
        .and_then(handler::create_table_handler);

    // Insert Row Route
    let insert_route = warp::post()
        .and(warp::path("insert"))
        .and(warp::path::param::<String>())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::insert_handler);

    // Select All Rows Route
    let select_route = warp::get()
        .and(warp::path("select"))
        .and(warp::path::param::<String>())
        .and(with_db(db.clone()))
        .and_then(handler::select_handler);

    // Update Row Route
    let update_route = warp::put()
        .and(warp::path("update"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<u32>())
        .and(warp::body::json())
        .and(with_db(db.clone()))
        .and_then(handler::update_handler);

    // Delete Row Route
    let delete_route = warp::delete()
        .and(warp::path("delete"))
        .and(warp::path::param::<String>())
        .and(warp::path::param::<u32>())
        .and(with_db(db.clone()))
        .and_then(handler::delete_handler);

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