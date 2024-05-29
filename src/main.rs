use axum::http::{header::CONTENT_TYPE, Method};
use dotenv::dotenv;
use routes::userRoute::create_router;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};

pub struct AppState {
    db: MySqlPool,
}

mod controllers;  // Ensure this line is here
mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ MySQL Connection Successful !");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the Database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any)
        .allow_headers([CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    println!("✅ Server is listening on Port 8080 !");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
