use axum::Json;
use crate::models::balance::{BalanceRequest, BalanceResponse};
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub async fn balance_handler(Json(payload): Json<BalanceRequest>) -> Json<BalanceResponse> {
    let rpc_url = "https://api.devnet.solana.com"; 
    let client = RpcClient::new(rpc_url.to_string());

    let pubkey_result = Pubkey::from_str(&payload.wallet);
    let (balance_sol, err_msg) = match pubkey_result {
        Ok(pubkey) => match client.get_balance(&pubkey) {
            Ok(lamports) => ((lamports as f64 / 1_000_000_000.0).to_string(), None),
            Err(e) => ("0.0".to_string(), Some(format!("RPC error: {}", e))),
        },
        Err(e) => ("0.0".to_string(), Some(format!("Invalid wallet: {}", e))),
    };

    Json(BalanceResponse {
        wallet: payload.wallet,
        balance_sol,
        error: err_msg,
    })
}
