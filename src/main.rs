use crate::ticker_logic::get_valid_ticker;
use axum::routing::get;
use axum::Router;

mod generate_random;
mod statics;
mod ticker_logic;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_ticker));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_ticker() -> String {
    get_valid_ticker()    
}
