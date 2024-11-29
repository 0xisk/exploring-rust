use serde::Serialize;
use serde_json::Serializer;
use std::{
    collections::HashMap,
    io,
    sync::Arc,
};
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

pub struct Block {
    pub number: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Clone, Serialize)]
pub struct Transaction {
    pub time: u64,
    pub id: u64,
    pub from: u64,
    pub to: u64,
    pub value: u64,
}

impl Transaction {
    fn new(number: u64) -> Self {
        Self {
            time: number,
            id: number,
            to: number % 17,
            from: (number + 1) % 13,
            value: (number + 2) % 89,
        }
    }
}

impl Block {
    fn new(number: u64) -> Self {
        Self {
            number,
            transactions: vec![Transaction::new(number)],
        }
    }
}

pub struct BlockProducer {
    pub number: u64,
}

impl BlockProducer {
    pub fn new() -> Self {
        Self { number: 0 }
    }

    pub async fn next(&mut self) -> Result<Block, ()> {
        self.number += 1;
        Ok(Block::new(self.number))
    }
}

type Address = u64;

struct Db {
    cache: HashMap<Address, Vec<Transaction>>,
}

impl Db {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

struct App {
    db: Arc<Mutex<Db>>,
}

impl App {
    fn new(db: Arc<Mutex<Db>>) -> Self {
        Self { db }
    }

    async fn handle(&self, address: String) -> Result<String, ()> {
        let address: u64 = address.parse().map_err(|_| ())?;
        let db = self.db.lock().await;

        if let Some(transactions) = db.cache.get(&address) {
            let json = serde_json::to_string(transactions).map_err(|_| ())?;
            return Ok(json);
        }

        Ok("[]".to_string())
    }
}

struct Indexer;

impl Indexer {
    async fn index(&self, block_producer: Arc<Mutex<BlockProducer>>, db: Arc<Mutex<Db>>) {
        let mut producer = block_producer.lock().await;
        let block = producer.next().await.expect("Failed to fetch block");

        println!("Indexing block: {}", block.number);

        let mut db = db.lock().await;

        for transaction in block.transactions.iter() {
            db.cache
                .entry(transaction.from)
                .or_default()
                .push(transaction.clone());
            db.cache
                .entry(transaction.to)
                .or_default()
                .push(transaction.clone());
        }
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let db = Arc::new(Mutex::new(Db::new()));
    let block_producer = Arc::new(Mutex::new(BlockProducer::new()));
    let indexer = Indexer;
    let app = App::new(db.clone());

    // Start the indexer in the background
    let block_producer_clone = block_producer.clone();
    tokio::spawn(async move {
        loop {
            indexer.index(block_producer_clone.clone(), db.clone()).await;
            sleep(Duration::from_millis(100)).await;
        }
    });

    // Query transactions for address "1"
    loop {
        let tx = app.handle("1".to_string()).await.unwrap();

        if tx
            == r#"[{"time":1,"id":1,"from":2,"to":1,"value":3},{"time":13,"id":13,"from":1,"to":13,"value":15}]"#
        {
            println!("Test passed!");
            break;
        }

        sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
