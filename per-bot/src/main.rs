mod auth;
mod tweets;
mod opts;

use std::fs;
use std::sync::Arc;

use tokio;
use toml;
use clap::Clap;

use auth::AuthConfig;
use tweets::Config;
use opts::*;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

#[tokio::main]
async fn main() -> Result {
    let opts = Options::parse();

    let tweet_file = fs::read(opts.tweet_list).expect("Error while reading the tweet file");
    let tweets: Config =
        toml::from_slice(tweet_file.as_slice()).expect("Error while parsing the tweet file");
    println!("{:?}", tweets);

    let auth_conf_file =
        fs::read(opts.authentication).expect("Error while reading authentication configuration");
    let auth_conf: AuthConfig = toml::from_slice(auth_conf_file.as_slice())
        .expect("Error while parsing the authentication configuration");
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
