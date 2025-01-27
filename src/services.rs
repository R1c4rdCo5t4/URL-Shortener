use std::collections::HashMap;
use axum::{
    Json,
    extract::{Path},
};
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::AppState;
use crate::types::{ShortenRequest, ShortenResponse};
use crate::json::save_urls;
use crate::utils::generate_short_id;

pub async fn shorten_url(
    State(state): State<AppState>,
    Json(body): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, (StatusCode, String)> {
    let mut urls = state.urls.lock().await;

    // generate id for short url
    let mut id;
    loop {
        id = generate_short_id();
        if !urls.contains_key(&id) {
            urls.insert(id.clone(), body.url.clone());
            break;
        }
    }

    // save to json
    if let Err(e) = save_urls(&urls).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to shorten url: {}", e)
        ))
    }

    // send response
    let short_url = format!("{}/{}", state.base_url, id);
    Ok(Json(ShortenResponse { short_url }))
}

pub async fn redirect_url(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let urls = state.urls.lock().await;
    match urls.get(&id) {
        Some(url) => Ok((
            StatusCode::MOVED_PERMANENTLY,
            [(axum::http::header::LOCATION, url.clone())],
        )),
        None => Err((StatusCode::NOT_FOUND, "URL not found".to_string()))
    }
}

pub async fn get_urls(State(state): State<AppState>) -> Json<HashMap<String, String>>  {
    let urls = state.urls.lock().await;

    // concatenate base url with short ids
    Json(urls
        .iter()
        .map(|(id, url)| (format!("{}/{}", state.base_url, id), url.clone()))
        .collect()
    )
}