mod endpoints;
mod tweet;

use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use env_logger::Env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(endpoints::submit)
            .service(endpoints::delete)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
