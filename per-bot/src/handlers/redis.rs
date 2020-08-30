use crate::tweets::Tweet;

use redis::aio::Connection;

use tokio::time::{interval, Duration};
use tokio::sync::mpsc::{Sender, Receiver};

type Result<T> = std::result::Result<T, redis::RedisError>;

pub struct RedisHandler {
    id: String,
    conn: Connection,
    counter_rx: Receiver<u64>,
    message_tx: Sender<String>
}

impl RedisHandler {
    pub fn new(id: String, conn: Connection, counter_rx: Receiver<u64>, message_tx : Sender<String>) -> Self {
        Self { id, conn, counter_rx , message_tx}
    }

    pub fn run(mut self) {
        tokio::spawn(async move { self.poll().await.unwrap() });
    }

    async fn poll(&mut self) -> Result<()> {
        let mut pause = interval(Duration::from_secs(5));
        loop {
            let update = self.check_update().await;
            println!("Got result {:?}", update);
            self.message_tx.send(update.unwrap().message).await.unwrap();
            let (_, v) = tokio::join!(pause.tick(), self.counter_rx.recv());
            self.update_counter(v.unwrap()).await.unwrap();
        }
    }

    async fn check_update(&mut self) -> Result<Tweet> {
        redis::cmd("JSON.GET")
            .arg(&self.id)
            .query_async(&mut self.conn)
            .await
    }

    async fn update_counter(&mut self, counter: u64) -> Result<()> {
        redis::cmd("JSON.SET")
            .arg(&self.id)
            .arg("counter")
            .arg(serde_json::to_string(&counter).unwrap())
            .query_async(&mut self.conn)
            .await?;
        Ok(())
    }
}
