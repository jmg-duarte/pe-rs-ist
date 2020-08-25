mod auth;

use std::fs;

use egg_mode::tweet::DraftTweet;
use tokio;
use tokio::time;
use toml;

use auth::AuthConfig;

const AUTH_CONF: &'static str = "auth.toml";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let auth_conf_file =
        fs::read(AUTH_CONF).expect("Error while reading authentication configuration");
    let auth_conf: AuthConfig = toml::from_slice(auth_conf_file.as_slice())
        .expect("Error while parsing the authentication configuration");
    let token = auth_conf.token();

    let mut interval = time::interval(time::Duration::from_secs(86400));
    let mut days: u64 = 0;
    loop {
        interval.tick().await;
        let tweet = DraftTweet::new(format!("{} days complaining.", days));
        tweet.send(&token).await?;
        days += 1;
    }
}
