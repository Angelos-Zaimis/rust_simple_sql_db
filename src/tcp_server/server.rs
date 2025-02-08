use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt}; // Import the necessary traits
use crate::database::SharedDatabase;

pub async fn run(db: SharedDatabase) {
    let listener = TcpListener::bind("127.0.0.1:4000").await.unwrap();
    println!("TCP Server is running on 127.0.0.1:4000");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        let db = db.clone();

        tokio::spawn(async move {
            let mut buf = [0; 1024];
            match socket.read(&mut buf).await {
                Ok(n) if n == 0 => return, 
                Ok(n) => {
                    let request = String::from_utf8_lossy(&buf[..n]);
                    // execute requests

                    let response = "SQL command processing not implemented yet";
                    socket.write_all(response.as_bytes()).await.unwrap();
                }
                Err(e) => eprintln!("Failed to read from socket; err = {:?}", e),
            }
        });
    }
}
