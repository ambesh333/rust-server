pub mod hello;
pub mod echo;
pub mod balance;
pub mod keypair;
pub mod token;
pub mod mint;
pub mod message;
pub mod verify;
pub mod send_sol;
pub mod send_token;

use axum::{Router};
use axum::routing::{get, post};

use hello::hello_handler;
use echo::echo_handler;
use balance::balance_handler;
use keypair::generate_keypair;
use token::create_token;
use mint::mint_token;
use message::sign_message;
use verify::verify_message;
use send_sol::send_sol;
use send_token::send_token;

pub fn register_routes() -> Router {
    Router::new()
        .route("/", get(|| async { "Hello world 🌍" }))
        .route("/hello", get(hello_handler))
        .route("/echo", post(echo_handler))
        .route("/balance", post(balance_handler))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(create_token))
        .route("/token/mint", post(mint_token))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(send_sol))
        .route("/send/token", post(send_token))
}
