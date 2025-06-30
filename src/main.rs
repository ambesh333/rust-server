mod routes;

use tokio::net::TcpListener;
use routes::register_routes;
mod models;

#[tokio::main]
async fn main() {
    let app = register_routes();

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
