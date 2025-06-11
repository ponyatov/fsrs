// https://masteringbackend.com/posts/actix-web-the-ultimate-guide#complete-actix-overview

mod config;
use actix_web::{App, HttpResponse, HttpServer, Responder, delete, get, post, put, web};

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

#[get("/user/{id}/{name}")]
async fn user_info(info: web::Path<(u32, String)>) -> impl Responder {
    let (id, name) = info.into_inner();
    HttpResponse::Ok().body(format!("User ID: {}, Name: {}", id, name))
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
            .service(user_info)
    })
    .bind(config::BIND)?
    .run()
    .await
}
