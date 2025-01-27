use axum::{
    Json,
    extract::{Path},
};


pub async fn shorten_url(Json(url): Json<String>) -> Json<String> {
    // get url from body
    // generate new random but short id for url
    // store key-value pair in json file
    // return key value and respond with 200 ok
}

pub async fn redirect_url(Path(url): Path<String>) {
    // get url from path
    // get corresponding short url from json
    // respond with 301 redirect to original url
}

pub async fn get_urls() -> Json<Vec<String>> {
    // return json with 200 ok
}