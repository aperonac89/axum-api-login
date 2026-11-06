mod auth;
mod error;
mod models;
mod routes;
mod state;

use crate::routes::auth::register;
use crate::state::AppState;
use axum::{
    Json, Router,
    routing::{get, post},
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello_handler))
        .route("/health", get(health_handler))
        .route("/register", post(register))
        .with_state(AppState::default());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "Hello world!"
}

async fn health_handler() -> Json<Value> {
    Json(json!({"status": "UP", "timespamp": "now"}))
}
