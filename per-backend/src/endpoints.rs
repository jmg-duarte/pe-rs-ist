use crate::tweet::{DelTweet, Tweet};

use actix_web::{delete, post, web, HttpResponse, Responder};
use redis::AsyncCommands;

const REDIS_URL: &'static str = "redis://0.0.0.0:32768";

#[post("/submit")]
pub(crate) async fn submit(tweet: web::Json<Tweet>) -> impl Responder {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let res: String = con
        .hset_multiple(
            &tweet.id,
            &[
                ("message", &tweet.message),
                ("interval", &tweet.interval.to_string()),
            ],
        )
        .await
        .unwrap();
    HttpResponse::Ok()
}

#[delete("/delete")]
pub(crate) async fn delete(id: web::Query<DelTweet>) -> impl Responder {
    let client = redis::Client::open(REDIS_URL).unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let res: isize = con.del(&id.id).await.unwrap();
    if res == 0 {
        HttpResponse::NotFound()
    } else {
        HttpResponse::Ok()
    }
}
