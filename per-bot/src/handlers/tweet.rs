use crate::tweets::Tweet;

use std::sync::Arc;

use tokio::sync::mpsc::{Receiver, Sender};

use egg_mode::auth::Token;
use egg_mode::tweet::DraftTweet;

use tokio::time::{interval, Duration};

pub struct TweetHandler {
    token: Arc<Token>,
    tweet: Tweet,
    counter_tx: Sender<u64>,
    message_rx: Receiver<String>,
}

impl TweetHandler {
    pub fn new(
        token: &Arc<Token>,
        tweet: Tweet,
        counter_tx: Sender<u64>,
        message_rx: Receiver<String>,
    ) -> Self {
        Self {
            token: Arc::clone(token),
            tweet,
            counter_tx,
            message_rx
        }
    }

    pub async fn send(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut interval = interval(Duration::from_secs(self.tweet.interval));
        loop {
            self.tweet.message = self.message_rx.recv().await.unwrap();
            let draft = DraftTweet::new(
                self.tweet
                    .message
                    .replace("{count}", &self.tweet.counter.to_string()),
            );
            interval.tick().await;
            // draft.send(&self.token).await?;
            println!("{:?}", draft);
            self.tweet.counter += 1;
            self.counter_tx.send(self.tweet.counter).await.unwrap();
        }
    }
}
