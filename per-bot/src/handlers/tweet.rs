use crate::tweets::Tweet;

use std::sync::Arc;
use tokio::sync::mpsc::Sender;

use egg_mode::auth::Token;
use egg_mode::tweet::DraftTweet;

use tokio::time::{interval, Duration};

pub struct TweetHandler {
    token: Arc<Token>,
    tweet: Tweet,
    tx: Sender<u64>
}

impl TweetHandler {
    pub fn new(token: &Arc<Token>, tweet: Tweet, tx: Sender<u64>) -> Self {
        Self {
            token: Arc::clone(token),
            tweet,
            tx
        }
    }

    pub async fn send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = interval(Duration::from_secs(self.tweet.interval));
        loop {
            let draft = DraftTweet::new(
                self.tweet
                    .message
                    .replace("{count}", &self.tweet.counter.to_string()),
            );
            interval.tick().await;
            // draft.send(&self.token).await?;
            println!("{:?}", draft);
            self.tweet.counter += 1;
            self.tx.send(self.tweet.counter).await.unwrap();
        }
    }
}
