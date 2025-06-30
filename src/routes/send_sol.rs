use axum::{Json, http::StatusCode};
use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
    instruction::Instruction,
};
use std::str::FromStr;
use base64;

use crate::models::send_sol::{SendSolRequest, SendSolResponse,
    SuccessResponse, ErrorResponse};

pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> Result<Json<SuccessResponse<SendSolResponse>>, (StatusCode, Json<ErrorResponse>)> {
    // Validate pubkeys
    let from = Pubkey::from_str(&payload.from).map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            success: false,
            error: "Invalid 'from' pubkey".into(),
        })
    ))?;

    let to = Pubkey::from_str(&payload.to).map_err(|_| (
        StatusCode::BAD_REQUEST,
        Json(ErrorResponse {
            success: false,
            error: "Invalid 'to' pubkey".into(),
        })
    ))?;

    // Create instruction
    let ix: Instruction = system_instruction::transfer(&from, &to, payload.lamports);

    // Base64 encode instruction data
    let encoded_ix_data = base64::encode(ix.data.clone());

    let response = SendSolResponse {
        program_id: ix.program_id.to_string(),
        accounts: ix.accounts.iter().map(|a| a.pubkey.to_string()).collect(),
        instruction_data: encoded_ix_data,
    };

    Ok(Json(SuccessResponse {
        success: true,
        data: response,
    }))
}
