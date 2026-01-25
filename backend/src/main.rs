use std::error::Error;

use actix_web::{App, HttpServer, web};

mod api;

const PORT: u32 = 9000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("[webapi] running at port {}", PORT);
    HttpServer::new(|| App::new().route("/", web::get()))
        .bind(format!("[::1]:{}", PORT))?
        .run()
        .await?;

    Ok(())
}
