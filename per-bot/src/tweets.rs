use std::collections::HashSet;
use std::fs;
use std::sync::Arc;

use egg_mode::{auth::Token, tweet::DraftTweet};

use serde::Deserialize;
use tokio::time::{interval, Duration};

use crate::error::{BotError, Result};

#[derive(Deserialize, Debug)]
pub struct TweetList {
    pub tweet: Vec<Tweet>,
}

impl TweetList {
    pub fn load(file_name: &str) -> Result<Self> {
        let tweet_file = fs::read(file_name).map_err(BotError::from)?;
        toml::from_slice(tweet_file.as_slice())
            .map_err(BotError::from)
            .and_then(|tl: Self| tl.validate())
    }

    fn validate(self) -> Result<Self> {
        let mut hs = HashSet::new();
        for t in &self.tweet {
            if !hs.insert(&t.id) {
                return Err(BotError::DuplicateId);
            }
        }
        Ok(self)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Tweet {
    pub id: String,
    pub message: String,
    /// The interval is in seconds
    pub interval: u64,
    #[serde(default = "default_counter")]
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
            // draft.send(&*token).await?;
            println!("{:?}", draft);
            self.counter += 1;
        }
        // Ok(())
    }
}

fn default_counter() -> u64 {
    0
}