use actix_web::{App, HttpServer, web};
use std::sync::Mutex;

// use std::io::{Error, Result};

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let port = self.port;
        let server = HttpServer::new(|| {
            App::new()
            // Add your routes here, e.g.:
            // .route("/", web::get().to(hello_world))
        })
        .bind(("0.0.0.0", port))?
        .run();

        println!("Server running on http://localhost:{}", port);
        server.await
    }
}
