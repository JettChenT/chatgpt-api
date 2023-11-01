use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

async fn conn_websocket(){
    let addr = "localhost:8181".to_string();
}

#[tokio::main]
async fn main() {
}
