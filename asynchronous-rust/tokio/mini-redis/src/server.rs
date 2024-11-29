use std::{future::Future, net::Shutdown, sync::Arc, time::Duration};

use anyhow::Ok;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc, Semaphore},
    time,
};

/// Pre-connection handler. Reads from `connection` and
/// applies the command to the `db`.
struct Handler {

}

struct Listener {
    // DB Drop Guard

    // TCP listener
    listener: TcpListener,

    // For controlling the maximum no of connections
    limit_connections: Arc<Semaphore>,

    notify_shutdown: broadcast::Sender<()>,

    // Used as a part of graceful shutdown process
    // to wait for client connections to complete processing
    shutdown_complete_tx: mpsc::Sender<()>,
}

impl Listener {
    /// Run the server
    /// Listen for inbound connections
    /// For each inbound connection, spawn a task to process that connection.
    async fn run(&self) -> crate::Result<()> {
        loop {
            let permit = self
                .limit_connections
                .clone()
                .acquire_owned()
                .await
                .unwrap();
        }

        let socket = self.accept().await?;

        //let handler = 
    }

    async fn accept(&self) -> crate::Result<TcpStream> {
        let mut backoff = 1;

        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        return Err(err.into());
                    }
                }
            }

            // Pase execution until the back off persoid is finished
            time::sleep(Duration::from_secs(backoff)).await;

            // Drop the back off
            backoff += 2;
        }
    }
}

pub fn run(listener: TcpListener, shutdown: impl Future) {
    // We need to use a broadcast channel to send a shutdown message
    // to all active connections
    let (notify_shutdown, _) = broadcast::channel(1);
    let (shutdown_complete_tx, mut shutdown_complete_rx) = mpsc::channel(1);

    // Initialize the listener state
    let mut server = Listener { listener };

    // tokio::select! {
    //     res = server.run() => {

    //     }
    // }
}
