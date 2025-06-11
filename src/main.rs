// https://masteringbackend.com/posts/actix-web-the-ultimate-guide#complete-actix-overview

mod config;
use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the index page!")
}

#[post("/create")]
async fn create() -> impl Responder {
    HttpResponse::Created().body("Resource created successfully!")
}

#[put("/update")]
async fn update() -> impl Responder {
    HttpResponse::Ok().body("Resource updated successfully!")
}

#[delete("/delete")]
async fn delete() -> impl Responder {
    HttpResponse::NoContent().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at http://{}:{}", config::HOST, config::PORT);
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(create)
            .service(update)
            .service(delete)
    })
    .bind(config::BIND)?
    .run()
    .await
}
