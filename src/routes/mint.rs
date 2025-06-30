use spl_token::instruction::mint_to;
use std::str::FromStr;

use axum::{Json, http::StatusCode};
use solana_sdk::pubkey::Pubkey;
use crate::models::mint_token::{
    MintTokenRequest, SuccessResponse, ErrorResponse, CreateTokenResponse, AccountMetaResponse,
};

pub async fn mint_token(
    Json(payload): Json<MintTokenRequest>,
) -> Result<Json<SuccessResponse<CreateTokenResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mint = Pubkey::from_str(&payload.mint).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Invalid mint address".to_string(),
            }),
        )
    })?;

    let destination = Pubkey::from_str(&payload.destination).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Invalid destination address".to_string(),
            }),
        )
    })?;

    let authority = Pubkey::from_str(&payload.authority).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: "Invalid authority address".to_string(),
            }),
        )
    })?;

    let program_id = spl_token::ID;

    let ix = mint_to(
        &program_id,
        &mint,
        &destination,
        &authority,
        &[],
        payload.amount,
    ).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Failed to create instruction: {}", e),
            }),
        )
    })?;

    let accounts = ix.accounts.iter().map(|a| AccountMetaResponse {
        pubkey: a.pubkey.to_string(),
        is_signer: a.is_signer,
        is_writable: a.is_writable,
    }).collect();

    let response = CreateTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: base64::encode(ix.data),
    };

    Ok(Json(SuccessResponse {
        success: true,
        data: response,
    }))
}
