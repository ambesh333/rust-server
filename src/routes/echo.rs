use axum::Json;
use crate::models::echo::{EchoRequest, EchoResponse};

pub async fn echo_handler(Json(payload): Json<EchoRequest>) -> Json<EchoResponse> {
    Json(EchoResponse {
        echoed: payload.input,
    })
}
