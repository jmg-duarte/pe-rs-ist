mod tweet;

use tweet::{DelTweet, Tweet};

use actix_web::{delete, post, web, App, HttpResponse, HttpServer, Responder};
use redis::AsyncCommands;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(submit).service(delete))
        .bind("127.0.0.1:8088")?
        .run()
        .await
}

#[post("/submit")]
async fn submit(tweet: web::Json<Tweet>) -> impl Responder {
    let client = redis::Client::open("redis://0.0.0.0:32768").unwrap();
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
async fn delete(id: web::Query<DelTweet>) -> impl Responder {
    let client = redis::Client::open("redis://0.0.0.0:32768").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    let res: isize = con.del(&id.id).await.unwrap();
    if res == 0 {
        HttpResponse::NotFound()
    } else {
        HttpResponse::Ok()
    }
}
