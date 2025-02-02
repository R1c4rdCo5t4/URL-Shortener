<div align="center">

# URL Shortener 🔗

**Simple URL shortener web API built using Rust and the Axum web framework**

[![Rust](https://img.shields.io/badge/Built_with-Rust-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![GitHub repo size](https://img.shields.io/github/repo-size/R1c4rdCo5t4/url-shortener)

</div>

---

## Running 🚀

### 📋 Prerequisites

- [Rust](https://www.rust-lang.org/)
- [Postman](https://www.postman.com/) or any other HTTP client

### 🛠️ Setup

1. Clone the repository

```sh
git clone https://github.com/R1c4rdCo5t4/url-shortener.git
```

2. Change to the project directory

```sh
cd url-shortener
```

3. Build the project

```sh
cargo build
```

4. Run the project

```sh
cargo run
```

### 🌐 Usage

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


## 🤝 Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## 📄 License

See [`LICENSE`](/LICENSE) for more information.