use axum::Json;
use crate::models::echo::HelloResponse;

pub async fn hello_handler() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello from Axum!".to_string(),
    })
}
