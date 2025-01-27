use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct AppState {
    pub(crate) base_url: String,
    pub(crate) urls: Arc<Mutex<HashMap<String, String>>>
}

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub(crate) url: String
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub(crate) short_url: String,
}