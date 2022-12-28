use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpResponse
};
use serde::{Deserialize, Serialize};
use sqlx::{self, FromRow};
use crate::AppState;


#[get("/artists")]
pub async fn get_artists(state: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json("get artist")
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
