use std::collections::HashMap;

const FILE_PATH: &str = "urls.json";

pub async fn load_urls() -> HashMap<String, String> {
    match tokio::fs::read_to_string(FILE_PATH).await {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

pub async fn save_urls(urls: &HashMap<String, String>) -> Result<(), String> {
    let data = serde_json::to_string(urls).map_err(|e| e.to_string())?;
    tokio::fs::write(FILE_PATH, data)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}