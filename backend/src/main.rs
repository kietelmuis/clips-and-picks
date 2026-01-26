use std::{
    collections::{HashMap, VecDeque},
    error,
};

use actix_web::{
    App, HttpResponse, HttpServer, Responder, get,
    web::{Data, Form, get},
};
use parking_lot::Mutex;
use serde_derive::Deserialize;
use uuid::Uuid;

use crate::api::{
    tiktok::TiktokApi,
    web::{game_create, game_get, game_join, game_start, health},
};

mod api;

const PORT: u32 = 9000;

struct UserState {
    videos: VecDeque<String>,
    tiktok_api: TiktokApi,
}

struct AppState {
    users: Mutex<HashMap<String, UserState>>,
}

#[derive(Deserialize)]
struct AuthCallback {
    code: String,
}

#[get("/auth/callback")]
async fn auth_callback(state: Data<AppState>, form: Form<AuthCallback>) -> impl Responder {
    let token = form.into_inner().code;

    {
        let mut users = state.users.lock();
        users.insert(
            Uuid::new_v4().to_string(),
            UserState {
                tiktok_api: TiktokApi::new(token),
                videos: VecDeque::new(),
            },
        );
    }

    HttpResponse::Ok()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let state = Data::new(AppState {
        users: Mutex::new(HashMap::new()),
    });

    println!("[webapi] running at port {}", PORT);
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(health)
            .service(auth_callback)
            .service(game_create)
            .service(game_join)
            .service(game_start)
            .route("/game", get().to(game_get))
    })
    .bind(format!("0.0.0.0:{}", PORT))?
    .run()
    .await?;

    Ok(())
}
