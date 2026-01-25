use std::error::Error;

use actix_web::{App, HttpServer};

use crate::api::web::health;

mod api;

const PORT: u32 = 9000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[webapi] running at port {}", PORT);
    HttpServer::new(|| App::new().service(health))
        .bind(format!("0.0.0.0:{}", PORT))?
        .run()
        .await?;

    Ok(())
}
