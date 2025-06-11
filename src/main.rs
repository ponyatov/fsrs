// https://masteringbackend.com/posts/actix-web-the-ultimate-guide#complete-actix-overview

mod config;
use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    "Hello, Actix web!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at http://{}:{}", config::HOST, config::PORT);
    HttpServer::new(|| {
        App::new().service(hello)
    })
        .bind(config::BIND)?
        .run()
        .await
}
