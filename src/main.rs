mod api;

use actix_web::{web, App, HttpServer};
use api::config::db::connect_db;
use api::router::config;
use dotenvy::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().expect(".env file not found");
    let host = env::var("API_HOST").expect("host found failed");
    let port: u16 = env::var("API_PORT")
        .expect("port found failed")
        .parse()
        .unwrap();
    let db_conn = connect_db().await.expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_conn.clone()))
            .configure(config)
    })
    .bind((host, port))?
    .run()
    .await
}
