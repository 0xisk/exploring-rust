use std::{
    collections::HashMap,
    os::unix::process,
    sync::{Arc, Mutex},
};

use anyhow::{anyhow, Ok, Result};
use bytes::Bytes;
use mini_redis::{
    cmd::Get,
    Command::{self, Get, Set},
    Connection, Frame,
};
use tokio::net::{TcpListener, TcpStream};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:3001")
        .await
        .map_err(|e| anyhow!("Failed ot listen to localhost: {}", e))?;

    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::<String, Bytes>::new()));

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .map_err(|e| anyhow!("Failed to accept sockets: {}", e))?;

        let db = Arc::clone(&db);

        tokio::spawn(async move {
            process(socket, db).await;
        });
    }
}

async fn process(socket: TcpStream, db: Db) -> Result<()> {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Set(cmd) => {
                let db = db.lock().unwrap();

                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }

    Ok(())
}
