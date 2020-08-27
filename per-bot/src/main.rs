mod auth;
mod error;
mod opts;
mod tweets;

use std::sync::Arc;

use anyhow::{Context, Result};
use clap::Clap;
use redis::AsyncCommands;

use auth::AuthConfig;
use opts::*;
use tweets::{Tweet, TweetList};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::parse();
    update_redis(&opts).await?;
    let auth_conf = AuthConfig::load(opts.authentication)
        .context("Error while reading authentication config")?;
    let token = Arc::new(auth_conf.token());

    let tl: Vec<Tweet> = Vec::new();

    let mut handlers = Vec::new();
    for mut t in tl {
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

async fn update_redis(opts: &Options) -> Result<()> {
    let tweets = TweetList::load(&opts.tweet_list).context("Error while reading tweet list")?;
    let client = redis::Client::open(opts.redis_url.to_string())?;
    let mut con = client.get_async_connection().await?;
    for tweet in &tweets.tweet {
        let _: String = con
            .hset_multiple(
                &tweet.id,
                &[
                    ("message", &tweet.message),
                    ("interval", &tweet.interval.to_string()),
                    ("counter", &tweet.counter.to_string()),
                ],
            )
            .await?;
    }
    Ok(())
}
