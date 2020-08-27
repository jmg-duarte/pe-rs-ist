use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub message: String,
    pub interval: u64,
}

#[derive(Debug, Deserialize)]
pub struct DelTweet {
    pub id: String
}