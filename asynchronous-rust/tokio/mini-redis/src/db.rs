use std::{
    collections::{BTreeSet, HashMap}, hash::Hash, os::linux::raw::stat, sync::{Arc, Mutex}
};

use bytes::Bytes;
use tokio::{
    sync::{broadcast, Notify},
    time::Instant,
};

/// That is a wrapper around a `Db` instance.
/// This exists to allow orderly cleanup of the `Db`
pub(crate) struct DbDropGuard {
    db: Db,
}

pub(crate) struct Db {
    shared: Arc<Shared>,
}

struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

/// The state contains:
///     1. `entries` Key/value data
///     2. `pub_sub` pub/sub key/value data we use HashMap for simplicity.
///     3. `expirations` Tracks key TTLS using `BTreeSet`
///     4. `shutdown` A boolean True value when the Db instance is shutting down.
struct State {
    entries: HashMap<String, Entry>,
    pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
    expirations: BTreeSet<(Instant, String)>,
    shutdown: bool,
}

struct Entry {
    data: Bytes,
    // Option when the entry expires and
    // should be removed from the Db
    expires_at: Option<Instant>,
}

impl DbDropGuard {
    /// Create a new `DbDropGuard`, wrapping a `Db` instance.
    pub(crate) fn new() -> DbDropGuard {
        DbDropGuard { db: Db::new() }
    }
}

/// Db should implement the following:
///     1. new()
///     2. get()
///     3. set()
///     4. publish()
///     5. subscribe()
///     6. shutdown_purge_task()
impl Db {
    pub(crate) fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new( State {
                entries: HashMap::new(),
                pub_sub: HashMap::new(),
                expirations: BTreeSet::new(),
                shutdown: false,
            }),
            background_task: Notify::new()
        });

        // Start the background task.
        //tokio::spawn(future)
    }
}

impl Shared {
    /// Purge all expired keys
    fn purge_expired_keys(&self) -> Option<Instant> {
        let mut state = self.state.lock().unwrap();

        if state.shutdown {
            return None;
        }

        let now = Instant::now();

        while let Some(&(when, ref key)) = state.expirations.iter().next()  =  {
            if when > now {
                return Some(when)
            }
        }
    }


}

async fn purge_expired_tasks(shared: Arc<Shared>) {
    while !shared.
}
