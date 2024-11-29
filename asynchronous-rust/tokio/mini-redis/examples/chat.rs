#![warn(rust_2018_idioms)]
use std::{collections::HashMap, error::Error, io, net::SocketAddr, sync::Arc};

use futures::SinkExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let state = Arc::new(Mutex::new(Shared::new()));

    // Create a TCP listener
    let listener = TcpListener::bind("localhost:3000").await?;

    loop {
        /// Async wait for the inbound TcpSteam
        let (stream, addr) = listener.accept().await?;

        // clone the state
        let state = Arc::clone(&state);

        // Spawn our handler to be run async
        tokio::spawn(async move {
            if let Err(e) = process(state, stream, addr).await {
                panic!("An error occurred = {:?}", e);
            }
        });
    }
}

type Tx = mpsc::UnboundedSender<String>;
type Rx = mpsc::UnboundedReceiver<String>;

/// Shared data between all the
struct Shared {
    peers: HashMap<SocketAddr, Tx>,
}

/// The state for each connected peer
struct Peer {
    lines: Framed<TcpStream, LinesCodec>,
    rx: Rx,
}

impl Shared {
    fn new() -> Self {
        Shared {
            peers: HashMap::new(),
        }
    }

    /// Send a `LineCodec` encoded message to every peer,
    /// except for the sender.
    async fn broadcast(&mut self, sender: SocketAddr, message: &str) {
        for peer in self.peers.iter_mut() {
            if *peer.0 != sender {
                peer.1.send(message.into());
            }
        }
    }
}

impl Peer {
    /// Create a new instance of `Peer`.
    async fn new(
        state: Arc<Mutex<Shared>>,
        lines: Framed<TcpStream, LinesCodec>,
    ) -> io::Result<Peer> {
        // Get the client socket address
        let addr = lines.get_ref().peer_addr()?;

        // Create a channel for this peer
        let (tx, rx) = mpsc::unbounded_channel();

        // Add an entry for this `Peer` in the shared state map.
        state.lock().await.peers.insert(addr, tx);

        Ok(Peer { lines, rx })
    }
}

async fn process(
    state: Arc<Mutex<Shared>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());

    // Send a prompt to the client to enter their username.
    lines.send("Please enter your username: ").await?;

    // Read the first line from the `LineCodec`
    let username = match lines.next().await {
        Some(Ok(line)) => line,
        _ => panic!("Failed to get username"),
    };

    // Register our peer with the state
    let mut peer = Peer::new(state.clone(), lines).await?;

    // A client has connected, let's everyone knows
    {
        let mut state = state.lock().await;
        let msg = format!("{username} has joined the chat");
        state.broadcast(addr, &msg).await;
    }

    // Process incoming messages until our stream is exhausted by a disconnect
    loop {
        tokio::select! {
            // A message was received from a peer. Sen dit to the current user.
            Some(msg) = peer.rx.recv()=> {
                peer.lines.send(&msg).await?;
            }
            result = peer.lines.next() => match result {
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let msg = format!("{username}: {msg}");

                    state.broadcast(addr, &msg).await;
                }
                Some(Err(e)) => {
                    panic!("An error occurred while processing messages for {}; error = {:?}", username, e);
                }
                None => break,
            }
        }
    }

    {
        let mut state = state.lock().await;
        state.peers.remove(&addr);

        let msg = format!("{username} has left the chat");
        state.broadcast(addr, &msg).await;
    }

    Ok(())
}
