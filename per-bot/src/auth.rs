use std::fs;

use egg_mode::KeyPair;
use serde::Deserialize;

use crate::error::{BotError, Result};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct AuthConfig {
    api_key: String,
    api_key_secret: String,
    bearer_token: String,
    access_token: String,
    access_token_secret: String,
}

impl AuthConfig {
    pub fn load(file_name: String) -> Result<Self> {
        let auth_conf_file = fs::read(file_name).map_err(BotError::from)?;
        toml::from_slice(auth_conf_file.as_slice()).map_err(BotError::from)
    }

    pub fn token(&self) -> egg_mode::Token {
        egg_mode::Token::Access {
            consumer: self.api_token(),
            access: self.access_token(),
        }
    }

    fn api_token(&self) -> KeyPair {
        KeyPair::new(self.api_key.clone(), self.api_key_secret.clone())
    }

    fn access_token(&self) -> KeyPair {
        KeyPair::new(self.access_token.clone(), self.access_token_secret.clone())
    }
}
