use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HelloResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct EchoRequest {
    pub input: String,
}

#[derive(Serialize)]
pub struct EchoResponse {
    pub echoed: String,
}
