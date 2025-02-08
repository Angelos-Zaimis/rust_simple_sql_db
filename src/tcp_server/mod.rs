pub mod server;

use crate::database::SharedDatabase;

pub async fn start_tcp_server(db: SharedDatabase) {
    server::run(db).await;
}
