use actix_web::web::{Data, Json, Path, Payload};
use actix_web::{Error, HttpRequest, HttpResponse, Responder, get, rt};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::api::web;

#[derive(Deserialize, Serialize)]
struct GameResponse {
    game_id: Uuid,
}

// main websocket
pub async fn game_get(
    data: Data<AppState>,
    req: HttpRequest,
    stream: Payload,
) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        .max_continuation_size(2_usize.pow(20));

    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Ping(msg)) => {
                    session.pong(&msg).await.unwrap();
                }
                _ => {}
            }
        }
    });

    Ok(res)
}

#[get("/game/start/{uuid}")]
async fn game_start(data: Data<AppState>, path: Path<String>) -> Result<impl Responder, Error> {
    println!("starting game {}", path.into_inner());
    Ok(HttpResponse::Ok())
}

#[get("/game/join/{uuid}")]
async fn game_join(data: Data<AppState>, path: Path<String>) -> Result<impl Responder, Error> {
    println!("joining game {}", path.into_inner());
    Ok(HttpResponse::Ok())
}

#[get("/game/create")]
async fn game_create() -> Result<impl Responder, Error> {
    println!("creating game");
    Ok(web::Json(GameResponse {
        game_id: Uuid::new_v4(),
    }))
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok()
}
