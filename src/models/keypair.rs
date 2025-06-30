use serde::Serialize;

#[derive(Serialize)]
pub struct KeypairData {
    pub pubkey: String,
    pub secret: String,
}

#[derive(Serialize)]
pub struct SuccessResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
}
