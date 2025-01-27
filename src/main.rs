mod services;
mod json;
mod utils;
mod types;

use std::collections::HashMap;
use services::{shorten_url, redirect_url, get_urls};
use dotenvy::dotenv;
use std::env;
use std::sync::Arc;
use axum::{
    Router,
    routing::get,
    routing::post,
};
use tokio::sync::Mutex;
use crate::json::load_urls;
use crate::types::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("localhost:{}", port);
    let urls = load_urls().await;
    let state = AppState {
        base_url: format!("http://{}", &addr),
        urls: Arc::new(Mutex::new(urls))
    };
    let app = Router::new()
        .route("/", get(get_urls))
        .route("/shorten", post(shorten_url))
        .route("/{id}", get(redirect_url))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}

