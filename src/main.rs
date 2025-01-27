mod services;
use services::{shorten_url, redirect_url, get_urls};
use dotenvy::dotenv;
use std::env;
use axum::{
    Router,
    routing::get,
    routing::post,
};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let app = Router::new()
        .route("/", get(get_urls))
        .route("/shorten", post(shorten_url))
        .route("/{url}", get(redirect_url));

    let addr = format!("localhost:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Listening on http://{}", &addr);
    axum::serve(listener, app).await.unwrap();
}

