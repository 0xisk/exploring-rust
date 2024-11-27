use anyhow::{anyhow, Ok, Result};
use bytes::Bytes;
use mini_redis::{
    client,
    Command::{Get, Set},
};
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;


/// Channels
#[tokio::main]
async fn main() -> Result<()> {
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    let manager = tokio::spawn(async move {
        // Open a connection with the server
        let mut client = client::connect("localhost:3001")
            .await
            .map_err(|e| anyhow!("Failed to connect to server {}", e))
            .unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let res = client.get(&key).await;
                    let _ = resp.send(res);
                }
                Command::Set { key, value, resp } => {
                    let res = client.set(&key, value).await;
                    let _ = resp.send(res);
                }
            }
        }
    });

    // Spawn two tasks, one to get the key and the other to set the keys
    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "foo".to_string(),
            resp: resp_tx,
        };

        // Send the GET request
        tx.send(cmd)
            .await
            .map_err(|e| anyhow!("Failed to send tx1 {}", e))
            .unwrap();

        // Await the response
        let res = resp_rx.await.unwrap();
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: resp_tx,
        };

        // Send the SET request
        tx2.send(cmd)
            .await
            .map_err(|e| anyhow!("Failed to send tx2 {}", e))
            .unwrap();

        // Await the response
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    t1.await
        .map_err(|e| anyhow!("Error happened in t1 : {}", e));

    t2.await
        .map_err(|e| anyhow!("Error happened in t2 : {}", e));

    manager.await.unwrap();

    Ok(())
}
