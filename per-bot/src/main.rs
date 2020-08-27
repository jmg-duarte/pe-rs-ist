mod auth;
mod tweets;
mod opts;
mod error;

use std::sync::Arc;

use tokio;
use clap::Clap;
use anyhow::{Result, Context};
use redis::AsyncCommands;

use auth::AuthConfig;
use tweets::TweetList;
use opts::*;


#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::parse();
    let tweets = TweetList::load(opts.tweet_list).context("Error while reading tweet list")?;
    let auth_conf = AuthConfig::load(opts.authentication).context("Error while reading authentication config")?;
    let token = Arc::new(auth_conf.token());

    let client = redis::Client::open(opts.redis_url)?;
    let mut con = client.get_async_connection().await?;
    for tweet in &tweets.tweet {
        let _ : String = con.hset_multiple(
            &tweet.id,
            &[
                ("message", &tweet.message),
                ("interval", &tweet.interval.to_string()),
                ("counter", &tweet.counter.to_string())
            ]
        ).await?;
    }

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
