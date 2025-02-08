pub mod handler;
pub mod routes;

use crate::database::SharedDatabase;

pub async fn start_rest_api(db: SharedDatabase) {
    routes::run(db).await;
}