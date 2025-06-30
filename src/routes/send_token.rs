use axum::{Json, http::StatusCode};
use solana_sdk::{pubkey::Pubkey, instruction::Instruction};
use spl_token::instruction::transfer;
use spl_associated_token_account::get_associated_token_address;
use std::str::FromStr;
use base64;

use crate::models::send_token::{SendTokenRequest, SendTokenResponse, AccountMetaResponse,
    SuccessResponse, ErrorResponse};

pub async fn send_token(
    Json(payload): Json<SendTokenRequest>,
) -> Result<Json<SuccessResponse<SendTokenResponse>>, (StatusCode, Json<ErrorResponse>)> {
    let owner = Pubkey::from_str(&payload.owner).map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            success: false,
            error: "Invalid owner pubkey".into(),
        })
    ))?;

    let mint = Pubkey::from_str(&payload.mint).map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            success: false,
            error: "Invalid mint pubkey".into(),
        })
    ))?;

    let destination = Pubkey::from_str(&payload.destination).map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            success: false,
            error: "Invalid destination pubkey".into(),
        })
    ))?;

    // Derive ATA addresses
    let source_token_account = get_associated_token_address(&owner, &mint);
    let destination_token_account = get_associated_token_address(&destination, &mint);

    let program_id = spl_token::ID;

    let ix: Instruction = transfer(
        &program_id,
        &source_token_account,
        &destination_token_account,
        &owner,
        &[],
        payload.amount,
    ).map_err(|e| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            success: false,
            error: format!("Failed to create transfer instruction: {e}"),
        })
    ))?;

    let response = SendTokenResponse {
        program_id: ix.program_id.to_string(),
        accounts: ix.accounts.iter().map(|a| AccountMetaResponse {
            pubkey: a.pubkey.to_string(),
            is_signer: a.is_signer,
        }).collect(),
        instruction_data: base64::encode(ix.data),
    };

    Ok(Json(SuccessResponse {
        success: true,
        data: response,
    }))
}
