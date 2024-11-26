#[allow(dead_code)]
use std::{
    collections::HashMap,
    os::unix::process,
    sync::{Arc, Mutex},
    time::SystemTime,
};

use anyhow::{anyhow, Ok, Result};
use bytes::Bytes;
use mini_redis::{
    Command::{self, Get, Set},
    Connection, Frame,
};
use tokio::net::{TcpListener, TcpStream};

type CacheDb = Arc<Mutex<HashMap<String, Bytes>>>;

/// A patten for the MutexGuard safe use through threads
/// @src https://tokio.rs/tokio/tutorial/shared-state
#[derive(Debug)]
struct Cache {
    created: SystemTime,
    db: CacheDb,
}

impl Cache {
    fn new() -> Self {
        Self {
            created: SystemTime::now(),
            db: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn get<F, R>(&self, key: &str, callback: F) -> R
    where
        F: FnOnce(Option<&Bytes>) -> R,
    {
        let cache = self.db.lock().expect("Failed to lock the MutexGuard!");
        let value = cache.get(key);
        callback(value)
    }

    fn set(&self, key: String, value: Bytes) {
        let mut cache = self.db.lock().expect("Failed to lock the MutexGuard!");
        cache.insert(key, value);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("localhost:3001")
        .await
        .map_err(|e| anyhow!("Failed ot listen to localhost: {}", e))?;

    println!("Listening");

    let cache = Arc::new(Cache::new());
    println!("Cache DB is created : {:?}", cache);

    loop {
        let (socket, addr) = listener
            .accept()
            .await
            .map_err(|e| anyhow!("Failed to accept sockets: {}", e))?;

        let cache = Arc::clone(&cache);

        tokio::spawn(async move {
            println!("Processing socket addr {:?}", addr);
            process(socket, cache).await;
        });
    }
}

async fn process(socket: TcpStream, cache: Arc<Cache>) -> Result<()> {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                println!("Processing a SET command {:?}", cmd);
                cache.set(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                println!("Processing a GET command {:?}", cmd);
                cache.get(cmd.key(), |value| match value {
                    Some(value) => Frame::Bulk(value.clone()),
                    None => Frame::Null,
                })
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await.unwrap();
    }

    Ok(())
}
