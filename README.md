# 🚀 Hello World API (Rust + Actix Web)
A simple web API built with [Actix Web](https://actix.rs/) that responds with "Hello world" at the /hello endpoint. This is a basic starter project for learning how to build and test HTTP APIs in Rust.
## 📦 Requirements
- Rust: https://www.rust-lang.org/tools/install
- Cargo (comes with Rust)
## 🔧 Setup
Clone the repository and build the project:
git clone https://github.com/your-username/hello-world-api.git && cd hello-world-api
cargo build
## 🚀 Run the Server
Start the API server:
cargo run
The server will be running at http://localhost:3033
## 🔍 Test the Endpoint
Test in browser or using curl:
curl http://localhost:3033/hello
Expected response:
Hello world from RUST with love
## 🧪 Run Tests
Run unit or integration tests for the /hello endpoint:
cargo test
Expected output:
running 1 test
test tests::test_hello_endpoint ... ok
## 📁 Project Structure
src/
└── main.rs     # Main entry point with route definition
Cargo.toml      # Project dependencies and metadata
## 🛠 Built With
- Rust: https://www.rust-lang.org/
- Actix Web: https://actix.rs/
## 📜 License
MIT License — feel free to use this project for learning or personal use.