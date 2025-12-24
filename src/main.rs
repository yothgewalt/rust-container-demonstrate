use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MessageRequest {
    message: String,
}

#[derive(Debug, Serialize)]
struct MessageResponse {
    echo: String,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: String,
}

async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "healthy".to_string(),
    };
    println!("Health check request received");
    (StatusCode::OK, Json(response))
}

async fn echo_message(Json(payload): Json<MessageRequest>) -> impl IntoResponse {
    let response = MessageResponse {
        echo: payload.message,
    };
    println!("Echo message request received: {}", payload.message);
    (StatusCode::OK, Json(response))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/message", post(echo_message));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}
