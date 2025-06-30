use axum::{Json, http::StatusCode};
use solana_sdk::signature::{Keypair, Signer};
use base64;
use bs58;
use crate::models::message::{
    SignMessageRequest, SignMessageResponse, SuccessResponse, ErrorResponse
};

pub async fn sign_message(
    Json(payload): Json<SignMessageRequest>,
) -> Result<Json<SuccessResponse<SignMessageResponse>>, (StatusCode, Json<ErrorResponse>)> {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Missing required fields".to_string(),
            }),
        ));
    }

    let secret_bytes = match bs58::decode(&payload.secret).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Invalid base58 secret key".to_string(),
                }),
            ));
        }
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Failed to parse secret key".to_string(),
                }),
            ));
        }
    };

    let signature = keypair.sign_message(payload.message.as_bytes());

    let response = SignMessageResponse {
        signature: base64::encode(signature.as_ref()),
        public_key: keypair.pubkey().to_string(),
        message: payload.message,
    };

    Ok(Json(SuccessResponse {
        success: true,
        data: response,
    }))
}
