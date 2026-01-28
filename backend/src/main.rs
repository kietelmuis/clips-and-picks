use dotenv::dotenv;
use migration::prelude::Local;
use reqwest::multipart::Form;
use sea_orm::ActiveValue::Set;
use std::{collections::VecDeque, env, error};
use uuid::Uuid;

use actix_web::{
    App, HttpRequest, HttpResponse, HttpServer, Responder,
    cookie::{Cookie, time::Duration},
    get,
    web::{Data, Query, get},
};
use rand::{Rng, distr::Alphanumeric};
use serde_derive::Deserialize;

use crate::{
    api::web::{game_create, game_get, game_join, game_start, health},
    db::{db::Db, models::sessions},
};

mod api;
mod db;

const PORT: u32 = 9000;

struct GameState {
    videos: VecDeque<String>,
}

struct AppState {
    db: Db,
    client: reqwest::Client,
}

const AUTH_URL: &str = "https://open.tiktokapis.com/v2/oauth/token/";
const CLIENT_KEY: &str = "your_tiktok_client_id";
const CLIENT_SECRET: &str = "your_tiktok_client_secret";
const SCOPE: &str = "user.info.basic";
const REDIRECT_URI: &str = "https://whatliked.onrender.com/auth/callback";

#[derive(Deserialize)]
struct AuthCallback {
    code: Option<String>,
    state: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    open_id: Option<String>,
    expires_in: Option<String>,
    refresh_expires_in: Option<String>,
}

#[get("/auth/callback")]
async fn auth_callback(
    req: HttpRequest,
    query: Query<AuthCallback>,
    state: Data<AppState>,
) -> Result<HttpResponse, Box<dyn error::Error>> {
    if let Some(err) = &query.error {
        return Ok(HttpResponse::BadRequest().body(format!(
            "auth callback error: {} - {:?}",
            err, query.error_description
        )));
    }

    let code = match &query.code {
        Some(c) => c,
        None => return Ok(HttpResponse::BadRequest().body("Missing authorization code")),
    };

    let returned_state = match &query.state {
        Some(s) => s,
        None => return Ok(HttpResponse::BadRequest().body("Missing state")),
    };

    let csrf_cookie = match req.cookie("csrf_state") {
        Some(c) => c.value().to_string(),
        None => return Ok(HttpResponse::BadRequest().body("Missing CSRF cookie")),
    };

    // verify csrf
    if csrf_cookie != *returned_state {
        return Ok(HttpResponse::Unauthorized().body("Invalid CSRF state"));
    }

    let token_response = state
        .client
        .post(AUTH_URL)
        .multipart(
            Form::new()
                .text("client_key", CLIENT_KEY)
                .text("client_secret", CLIENT_SECRET)
                .text("code", code.clone())
                .text("grant_type", "authorization_code")
                .text("redirect_uri", REDIRECT_URI),
        )
        .send()
        .await?
        .json::<TokenResponse>()
        .await?;

    match token_response.refresh_token {
        Some(refresh_token) => {
            println!("certified token: {}", refresh_token);

            // {
            //     state
            //         .users
            //         .lock()
            //         .insert(Uuid::new_v4().to_string(), TiktokApi::new(token.clone()));
            // }
            // println!("key {} {} users", token, state.users.lock().len());

            Ok(HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish())
        }
        None => Err("No refresh token in response".into()),
    }
}

#[get("/auth")]
async fn auth() -> impl Responder {
    let state: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let redirect_url = format!(
        "{}?client_key={}&response_type=code&scope={}&redirect_uri={}&state={}",
        AUTH_URL,
        CLIENT_KEY,
        SCOPE,
        urlencoding::encode(REDIRECT_URI),
        state
    );

    HttpResponse::Found()
        .cookie(
            Cookie::build("csrf_state", state)
                .http_only(true)
                .secure(true)
                .path("/")
                .max_age(Duration::seconds(300))
                .finish(),
        )
        .append_header(("Location", redirect_url))
        .finish()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    dotenv().ok();

    let database = db::db::Db::new(env::var("DATABASE_URL").expect("failed to get db url")).await?;
    // database
    //     .insert(sessions::ActiveModel {
    //         id: Set(Uuid::new_v4()),
    //         tiktok_user_id: Set(Uuid::new_v4()),
    //         refresh_token: Set("hello".to_string()),
    //         expires_at: Set(Local::now().naive_local()),
    //         revoked: Set(false),
    //     })
    //     .await
    //     .unwrap();

    let state = Data::new(AppState {
        db: database,
        client: reqwest::Client::new(),
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
