mod auth;
mod tweets;
mod opts;

use std::sync::Arc;

use tokio;
use clap::Clap;

use auth::AuthConfig;
use tweets::TweetList;
use opts::*;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result {
    let opts = Options::parse();
    let tweets = TweetList::load(opts.tweet_list);
    let auth_conf = AuthConfig::load(opts.authentication);
    let token = Arc::new(auth_conf.token());

    let mut handlers = Vec::new();
    for mut t in tweets.tweet {
        let arc_tok = Arc::clone(&token);
        handlers.push(tokio::spawn(async move {
            let _ = t.send(arc_tok).await;
        }));
    }

    for h in handlers {
        h.await?;
    }

    Ok(())
}