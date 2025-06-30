use axum::{Json, http::StatusCode};
use solana_sdk::signature::{Keypair, Signer};
use bs58;

use crate::models::keypair::{KeypairData, SuccessResponse};

pub async fn generate_keypair() -> (StatusCode, Json<SuccessResponse<KeypairData>>) {
    let keypair = Keypair::new();

    let pubkey = keypair.pubkey().to_string();
    let secret_bytes = keypair.to_bytes();
    let secret = bs58::encode(secret_bytes).into_string();

    let response = SuccessResponse {
        success: true,
        data: KeypairData { pubkey, secret },
    };

    (StatusCode::OK, Json(response))
}
