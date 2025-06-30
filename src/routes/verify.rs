use solana_sdk::signature::Signature;
use solana_sdk::pubkey::Pubkey;
use crate::models::verify::{VerifyMessageRequest, VerifyMessageResponse,
    SuccessResponse, ErrorResponse};
use axum::{Json, http::StatusCode};


pub async fn verify_message(
    Json(payload): Json<VerifyMessageRequest>,
) -> Result<Json<SuccessResponse<VerifyMessageResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let sig_bytes = match base64::decode(&payload.signature) {
        Ok(b) => b,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Invalid base64 signature".to_string(),
                }),
            ));
        }
    };

    let signature = match Signature::try_from(sig_bytes.as_slice()) {
        Ok(sig) => sig,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Invalid signature format".to_string(),
                }),
            ));
        }
    };

    let pubkey = match bs58::decode(&payload.pubkey).into_vec() {
        Ok(bytes) => match Pubkey::try_from(bytes.as_slice()) {
            Ok(pk) => pk,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        success: false,
                        error: "Invalid pubkey format".to_string(),
                    }),
                ));
            }
        },
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Invalid base58 pubkey".to_string(),
                }),
            ));
        }
    };

    let valid = signature.verify(pubkey.as_ref(), payload.message.as_bytes());

    let response = VerifyMessageResponse {
        valid,
        message: payload.message,
        pubkey: payload.pubkey,
    };

    Ok(Json(SuccessResponse {
        success: true,
        data: response,
    }))
}
