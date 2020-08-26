use clap::Clap;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

#[derive(Clap)]
#[clap(version=VERSION, author=AUTHORS)]
pub struct Options {
    /// The path to the authentication file
    #[clap(short="a", long="auth", default_value = "auth.toml")]
    pub authentication: String,
    /// The path to the tweet list
    #[clap(short="t", long="tweet-list", default_value ="tweets.toml")]
    pub tweet_list: String,
}