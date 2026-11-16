use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

/// Builds the HTTP router for the `serve` command.
pub fn app() -> Router {
    Router::new().route("/health", get(health))
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
