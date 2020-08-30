mod auth;
mod error;
mod handlers;
mod opts;
mod tweets;

use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use anyhow::{Context, Result};
use auth::AuthConfig;
use clap::Clap;
use egg_mode::auth::Token;
use error::BotError;
use handlers::redis::RedisHandler;
use handlers::tweet::TweetHandler;
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

    let tweets: Vec<Tweet> = f_tweets
        .into_iter()
        .collect::<Result<Vec<Tweet>, redis::RedisError>>()
        .map_err(BotError::from)?;
    // tweets;
    let auth_conf = AuthConfig::load(opts.authentication)
        .context("Error while reading authentication config")?;
    init(auth_conf.token(), tweets).await
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
    let mut tweet_handlers = Vec::with_capacity(tweets.len());
    let mut redis_handlers = Vec::with_capacity(tweets.len());

    let client = redis::Client::open("redis://0.0.0.0:6379")?;

    for tweet in tweets {
        let (counter_tx, counter_rx) = channel::<u64>(5);
        let (message_tx, message_rx) = channel::<String>(5);

        let mut tweet_handler = TweetHandler::new(&token, tweet.clone(), counter_tx, message_rx);
        tweet_handlers.push(tokio::spawn(async move {
            tweet_handler.send().await.unwrap();
        }));

        let redis_handler = RedisHandler::new(
            tweet.id,
            client.get_async_connection().await?,
            counter_rx,
            message_tx,
        );
        redis_handlers.push(redis_handler.run());
    }

    for h in tweet_handlers {
        h.await?;
    }

    Ok(())
}

// async fn poll_redis(tx: Sender<Tweet>) -> Result<()> {
//     let client = redis::Client::open("redis://0.0.0.0:6379")?;
//     let mut con = client.get_async_connection().await?;
//     loop {
//         redis::cmd("JSON.GET").arg()
//     }
// }
