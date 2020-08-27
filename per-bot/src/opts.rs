use clap::Clap;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

const REDIS_URL: &str = "redis://0.0.0.0:32768";

#[derive(Clap)]
#[clap(version=VERSION, author=AUTHORS)]
pub struct Options {
    /// The path to the authentication file
    #[clap(short = "a", long = "auth", default_value = "auth.toml")]
    pub authentication: String,
    /// The path to the tweet list
    #[clap(short = "t", long = "tweet-list", default_value = "tweets.toml")]
    pub tweet_list: String,
    /// The Redis url to use
    #[clap(short="r", long="redis-url", default_value = REDIS_URL)]
    pub redis_url: String,
}
