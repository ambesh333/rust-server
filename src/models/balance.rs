use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BalanceRequest {
    pub wallet: String,
}

#[derive(Serialize)]
pub struct BalanceResponse {
    pub wallet: String,
    pub balance_sol: String,
    pub error: Option<String>,
}
