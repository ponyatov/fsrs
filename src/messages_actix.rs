use actix_web::{App, Error, HttpServer, Result, web};
use std::sync::Mutex;

pub struct MessageApp {
    port: u16,
}

impl MessageApp {
    pub fn new(port: u16) -> Self {
        MessageApp { port }
    }

    pub async fn run(&self) -> Result<()> {
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
