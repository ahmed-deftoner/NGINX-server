use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use uuid::Uuid;
use crate::AppState;

#[derive(Serialize, FromRow)]
struct Artist {
    name: String,
}

#[derive(Serialize, FromRow)]
struct Album {
    name: String,
    songs: u32,
    artist: Uuid
}


#[get("/artists")]
pub async fn get_artists(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, artist>("SELECT id, name FROM artists")
        .fetch_all(&state.db)
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::NotFound().json("No artists found"),
    }
}


#[get("/album/{id}")]
pub async fn get_album(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("get album")
}

#[post("/album/{id}")]
pub async fn create_album(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("create album")
}

#[post("/artists")]
pub async fn create_artist(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("create artist")
}
