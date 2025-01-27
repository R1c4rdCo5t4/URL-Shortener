use std::collections::HashMap;
use std::sync::Arc;
use serde::Deserialize;
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct AppState {
    pub(crate) base_url: String,
    pub(crate) urls: Arc<Mutex<HashMap<String, String>>>
}

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub(crate) url: String
}

