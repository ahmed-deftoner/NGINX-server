
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
    id: i32,
    name: String,
}

#[derive(Deserialize)]
pub struct ArtistBody {
    pub name: String
}

#[derive(Serialize, FromRow)]
struct Album {
    name: String,
    songs: u32,
    artist: Uuid
}


#[get("/artists")]
pub async fn get_artists(state: Data<AppState>) -> impl Responder {
    match sqlx::query_as::<_, Artist>("SELECT id, name FROM Artist")
        .fetch_all(&state.db)
        .await
    {
        Ok(artists) => HttpResponse::Ok().json(artists),
        Err(_) => HttpResponse::NotFound().json("No artists found"),
    }
}


#[get("/album/{id}")]
pub async fn get_album(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("create album")

}

#[post("/album/{id}")]
pub async fn create_album(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("create album")
}

#[post("/artists")]
pub async fn create_artist(state: Data<AppState>, body: Json<ArtistBody>) -> impl Responder {

    match sqlx::query_as::<_, Artist>(
        "INSERT INTO Artist (name) VALUES ($1) RETURNING id, name"
    )
        .bind(body.name.to_string())
        .fetch_one(&state.db)
        .await
    {
        Ok(artist) => HttpResponse::Ok().json(artist),
        Err(_) => HttpResponse::InternalServerError().json("Failed to create artist"),
    }
}
