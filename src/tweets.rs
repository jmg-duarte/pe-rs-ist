use std::sync::Arc;

use egg_mode::{auth::Token, tweet::DraftTweet};

use serde::Deserialize;
use tokio::time::{interval, Duration, Interval};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub tweet: Vec<Tweet>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tweet {
    pub message: String,
    /// The interval is in seconds
    pub interval: u64,
    #[serde(default="Tweet::default_counter")]
    pub counter: u64,
}

impl Tweet {
    pub async fn send(
        &mut self,
        token: Arc<Token>,
    ) -> std::result::Result<(), Box<dyn std::error::Error>> {
        let mut interval = interval(Duration::from_secs(self.interval));
        loop {
            let draft = DraftTweet::new(
                self.message
                    .to_owned()
                    .replace("{count}", self.counter.to_string().as_str()),
            );
            interval.tick().await;
            draft.send(&*token).await?;
            self.counter += 1;
        }
        Ok(())
    }

    fn default_counter() -> u64 {
        0
    }
    
}

