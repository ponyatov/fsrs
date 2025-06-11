#![allow(dead_code)]
#![allow(unused_imports)]

mod config;
mod messages_actix;

#[cfg(test)]
mod test;

use crate::messages_actix::MessageApp;
use actix_web::{App, Error, HttpServer, Result, web};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    unsafe {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let app = MessageApp::new(config::PORT);
    app.run()
}
