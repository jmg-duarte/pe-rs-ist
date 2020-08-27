mod auth;
mod error;
mod opts;
mod tweets;

use std::collections::HashSet;
use std::sync::Arc;

use anyhow::{Context, Result};
use auth::AuthConfig;
use clap::Clap;
use egg_mode::auth::Token;
use error::BotError;
use opts::*;
use redis::AsyncCommands;
use tweets::{Tweet, TweetList};

#[tokio::main]
async fn main() -> Result<()> {
    let opts = Options::parse();
    update_redis(&opts).await?;

    let client = Arc::new(redis::Client::open(opts.redis_url.to_string())?);
    let mut con = client.get_async_connection().await?;
    let ids: HashSet<String> = con.smembers("tweets::ids").await?;

    let mut queries = Vec::new();
    for id in ids {
        let client_arc = Arc::clone(&client);
        queries.push(tokio::spawn(async move {
            let mut con = client_arc.get_async_connection().await?;

            redis::cmd("JSON.GET")
                .arg(id)
                .query_async::<redis::aio::Connection, Tweet>(&mut con)
                .await
        }));
        // println!("{:?}", r);
    }
    let mut f_tweets = Vec::new();
    for q in queries {
        f_tweets.push(q.await?);
    }
    let tweets: Result<Vec<Tweet>, BotError> = f_tweets
    .into_iter()
    .collect();
    let auth_conf = AuthConfig::load(opts.authentication)
        .context("Error while reading authentication config")?;
    init(auth_conf.token(), tweets?).await
}

async fn update_redis(opts: &Options) -> Result<()> {
    let tweets = TweetList::load(&opts.tweet_list).context("Error while reading tweet list")?;
    let client = redis::Client::open(opts.redis_url.to_string())?;
    let mut con = client.get_async_connection().await?;
    for tweet in &tweets.tweet {
        redis::cmd("JSON.SET")
            .arg(&tweet.id)
            .arg(".")
            .arg(serde_json::to_string(tweet)?)
            .query_async(&mut con)
            .await?;
        let _: isize = con.sadd("tweets::ids", &tweet.id).await?;
    }
    Ok(())
}

async fn init(token: Token, tweets: Vec<Tweet>) -> Result<()> {
    let token = Arc::new(token);
    let mut handlers = Vec::new();
    for mut t in tweets {
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
