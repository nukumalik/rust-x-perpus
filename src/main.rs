use actix_web::{web, App, HttpServer};
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
            .app_data(web::Data::new(config::app_state::AppState::new(
                pool.clone(),
            )))
            .configure(modules::country::route::country_route)
            .configure(modules::province::route::province_route)
            .configure(modules::city::route::city_route)
            .configure(modules::district::route::district_route)
    });
    println!("Server is running...");
    server.bind("localhost:3000")?.run().await
}
