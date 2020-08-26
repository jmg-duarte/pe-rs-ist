use egg_mode::KeyPair;
use serde::Deserialize;

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
    fn api_token(&self) -> KeyPair {
        KeyPair::new(self.api_key.clone(), self.api_key_secret.clone())
    }
    
    fn access_token(&self) -> KeyPair {
        KeyPair::new(self.access_token.clone(), self.access_token_secret.clone())
    }
    
    pub fn token(&self) -> egg_mode::Token {
        egg_mode::Token::Access {
            consumer: self.api_token(),
            access: self.access_token(),
        }
    }
}