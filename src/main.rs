use actix::fut::ok;
use actix_web::{web::Data, App, HttpServer, http};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

mod routes;


pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("database url needed");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("error building connection pool");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone()}))
            .service(get_artists)
            .service(create_artist)
            .service(get_album)
            .service(create_album)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
