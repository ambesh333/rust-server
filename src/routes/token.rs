use axum::{Json, http::StatusCode};
use solana_sdk::{
    instruction::Instruction,
    pubkey::Pubkey,
};
use spl_token::instruction::initialize_mint;
use base64;
use std::str::FromStr;

use crate::models::token::{
    CreateTokenRequest, SuccessResponse, ErrorResponse, CreateTokenResponse, AccountMetaResponse
};

pub async fn create_token(
    Json(payload): Json<CreateTokenRequest>,
) -> Result<Json<SuccessResponse<CreateTokenResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let mint_pubkey = match Pubkey::from_str(&payload.mint) {
        Ok(p) => p,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Invalid mint pubkey".to_string(),
                }),
            ))
        }
    };

    let mint_authority = match Pubkey::from_str(&payload.mint_authority) {
        Ok(p) => p,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,
                    error: "Invalid mint authority pubkey".to_string(),
                }),
            ))
        }
    };

    let program_id = spl_token::ID;

    let ix: Instruction = initialize_mint(
        &program_id,
        &mint_pubkey,
        &mint_authority,
        None,
        payload.decimals,
    ).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                success: false,
                error: format!("Failed to build instruction: {}", e),
            }),
        )
    })?;

    let accounts: Vec<AccountMetaResponse> = ix
        .accounts
        .iter()
        .map(|a| AccountMetaResponse {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
            is_writable: a.is_writable,
        })
        .collect();

    let encoded_ix_data = base64::encode(ix.data);

    let response = CreateTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts,
        instruction_data: encoded_ix_data,
    };

    Ok(Json(SuccessResponse {
        success: true,
        data: response,
    }))
}
