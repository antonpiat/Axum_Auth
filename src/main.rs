use std::sync::Arc;
use axum::http::{
    HeaderValue, Method,
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}
};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::CorsLayer;

pub mod model;
pub mod response;
pub mod config;
pub mod jwt_auth;
pub mod handler;
pub mod route;

use config::Config;
use crate::route::create_router;

pub struct AppState {
    db: Pool<Postgres>,
    env: Config
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
    {
        Ok(pool) => {
            println!("Successfully connected to database");
            pool
        },
        Err(err) => {
            println!("Failed to connect to database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState {
        db: pool.clone(),
        env: config.clone()
    }))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Starting server on port 8000");

    axum::serve(listener, app).await.unwrap();
}
