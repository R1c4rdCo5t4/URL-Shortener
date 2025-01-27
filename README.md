# URL Shortener

Simple URL shortener built using Rust and the Axum web framework

## How to Use

In the terminal, start the server:
```
cargo run
```

Make a POST request to `/shorten` with a JSON body containing the URL you want to shorten:
```json
{
    "url": "https://www.google.com"
}
```

The server will respond with a JSON object containing the shortened URL:
```json
{
    "short_url": "http://localhost:3000/h5DVn"
}
```

To redirect to the original URL, simply make a GET request to the shortened URL. The server then responds with a 301 redirect to the original URL.
