use actix_web::{web, App, HttpServer};

use config::app_state::AppState;
use modules::country::route::country_route;
use sqlx::SqlitePool;

mod config;
mod modules;
mod traits;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = SqlitePool::connect("sqlite:data.db")
        .await
        .expect("Failed to connect database");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState::new(pool.clone())))
            .configure(country_route)
    });
    println!("Server is running...");
    server.bind("localhost:3000")?.run().await
}
