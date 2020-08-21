use std::fs;

use egg_mode::tweet::DraftTweet;
use egg_mode::KeyPair;
use serde::Deserialize;
use tokio;
use tokio::time;
use toml;

const AUTH_CONF: &'static str = "auth.toml";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
struct AuthConfig {
    api_key: String,
    api_key_secret: String,
    bearer_token: String,
    access_token: String,
    access_token_secret: String,
}

impl AuthConfig {
    fn api_token(&self) -> KeyPair {
        KeyPair::new(self.api_key.clone(), self.api_key_secret.clone())
    }
    fn access_token(&self) -> KeyPair {
        KeyPair::new(self.access_token.clone(), self.access_token_secret.clone())
    }
    fn token(&self) -> egg_mode::Token {
        egg_mode::Token::Access {
            consumer: self.api_token(),
            access: self.access_token(),
        }
    }
}

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
    Ok(())
}
