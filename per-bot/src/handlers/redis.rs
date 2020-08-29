use crate::tweets::Tweet;
use redis::aio::Connection;
use tokio::time::{interval, Duration};

pub struct RedisHandler {
    id: String,
    conn: Connection,
}

impl RedisHandler {
    pub fn new(id: String, conn: Connection) -> Self {
        Self { id, conn }
    }

    pub async fn poll(&mut self) -> Result<(), redis::RedisError> {
        let mut pause = interval(Duration::from_secs(5));
        loop {
            let result = self.check_update().await?;
            println!("{:?}", result);
            pause.tick().await;
        }
    }

    async fn check_update(&mut self) -> Result<Tweet, redis::RedisError> {
        redis::cmd("JSON.GET")
            .arg(&self.id)
            .query_async(&mut self.conn)
            .await
    }
}
