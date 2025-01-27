# URL Shortener

Simple URL shortener web API built using Rust and the Axum web framework

![](https://skillicons.dev/icons?i=rust)

## How to Use

In a terminal, start the server:
```
cargo run
```

Then, make a POST request to `/shorten` with a JSON body containing the URL you want to shorten:
```json
{
    "url": "https://www.google.com"
}
```

The server will respond with a JSON object containing the shortened URL:
```json
{
    "short_url": "http://localhost:3000/IhKTUx"
}
```

Finally, simply make a GET request to the shortened URL, which the server redirects to the original URL.
