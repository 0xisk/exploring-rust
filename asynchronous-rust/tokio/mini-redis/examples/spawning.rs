use std::collections::HashMap;

use anyhow::{anyhow, Ok, Result};
use mini_redis::{
    Command::{self, Get, Set},
    Connection, Frame,
};
use tokio::net::{TcpListener, TcpStream};

/// Spawning
/// Accepting sockets
///     1. The server needs to accept inbound TCP sockets. (tokio::net::TcpListener)
///     2. Bind it to port `6379`
///     3. Accept the sockets in a loop to process them.
///     4. Process each socket and then close it.
///     5. Then try to process many sockets concurrently. (tokio::spawn)
#[tokio::main]
async fn main() -> Result<()> {
    // Bind the listener to the address
    let listener = TcpListener::bind("localhost:3001")
        .await
        .map_err(|e| anyhow!("Failed to initialize a TcpListener on port 3000: {}", e))?;

    loop {
        // Accept inbound Tcp connection sockets
        let (socket, addr) = listener
            .accept()
            .await
            .map_err(|e| anyhow!("Failed to accept inbound connection: {}", e))?;

        println!("Accept {:?} socket from address {:?}", socket, addr);

        tokio::spawn(async move {
            // Process each accepted socket
            process(socket).await;
        });

        // let out = handler
        //     .await
        //     .map_err(|e| anyhow!("Error in the JoinHandler:{}", e))
        //     .unwrap();
    }
}

async fn default_process(socket: TcpStream) -> Result<()> {
    // The `Connection` let us read/write redis **frames** instead of
    // byte steams.
    let mut connection = Connection::new(socket);

    let frame = connection
        .read_frame()
        .await
        .map_err(|e| anyhow!("Failed to read socket frame: {}", e))?
        .ok_or_else(|| anyhow!("No frame found!"))?;

    println!("Got that frame: {:?}", frame);

    let response = Frame::Simple("Welcome".into());

    Ok(connection
        .write_frame(&response)
        .await
        .map_err(|e| anyhow!("Failed to send a response: {}", e))?)
}

async fn store_process(socket: TcpStream) {
    // A hashmap to store data
    let mut db = HashMap::new();

    let mut connection = Connection::new(socket);

    // Read the recieved command from the connection
    while let Some(frame) = connection.read_frame().await.expect("Failed to read frame") {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        connection.write_frame(&response).await;
    }
}

async fn process(socket: TcpStream) {
    use mini_redis::Command::{self, Get, Set};
    use std::collections::HashMap;

    // A hashmap is used to store data
    let mut db = HashMap::new();

    // Connection, provided by `mini-redis`, handles parsing frames from
    // the socket
    let mut connection = Connection::new(socket);

    // Use `read_frame` to receive a command from the connection.
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // The value is stored as `Vec<u8>`
                db.insert(cmd.key().to_string(), cmd.value().to_vec());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` expects data to be of type `Bytes`. This
                    // type will be covered later in the tutorial. For now,
                    // `&Vec<u8>` is converted to `Bytes` using `into()`.
                    Frame::Bulk(value.clone().into())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("unimplemented {:?}", cmd),
        };

        // Write the response to the client
        connection.write_frame(&response).await.unwrap();
    }
}
