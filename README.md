# 🚀 Hello World API (Rust + Actix Web)
A RESTful web API built with [Actix Web](https://actix.rs/) that provides various endpoints for user management and system health checks. This project demonstrates how to build and test HTTP APIs in Rust with proper project structure and testing.

## 📦 Requirements
- Rust: https://www.rust-lang.org/tools/install
- Cargo (comes with Rust)

## 🔧 Setup
Clone the repository and build the project:
```bash
git clone https://github.com/AliaksandrMakaranka/rust_hello-api.git && cd rust_hello-api
cargo build
```

## 🚀 Run the Server
Start the API server:
```bash
cargo run
```
The server will be running at http://localhost:3033

## 🔍 Available Endpoints

### Hello World
```bash
curl http://localhost:3033/hello
```
Expected response:
```
Hello world from RUST with love
```

### Health Check
```bash
curl http://localhost:3033/health
```
Returns system health status and version information.

### User Management

#### List All Users
```bash
curl http://localhost:3033/users
```

#### Get User by ID
```bash
curl http://localhost:3033/users/{id}
```

#### Create New User
```bash
curl -X POST http://localhost:3033/users \
  -H "Content-Type: application/json" \
  -d '{"name": "John Doe", "email": "john@example.com"}'
```

## 🧪 Run Tests
Run all tests:
```bash
cargo test
```

The test suite includes:
- Hello endpoint tests
- Health check endpoint tests
- User management tests (create, read, list)
- Error handling tests

## 📁 Project Structure
```
src/
├── main.rs          # Main entry point and server configuration
├── handlers/        # Request handlers
│   ├── hello.rs     # Hello endpoint handler
│   ├── health.rs    # Health check handler
│   └── users.rs     # User management handlers
├── models/          # Data models
│   └── user.rs      # User-related structures
└── tests/           # Integration tests
    └── mod.rs       # Test suite
```

## 🛠 Built With
- Rust: https://www.rust-lang.org/
- Actix Web: https://actix.rs/

## 📜 License
MIT License — feel free to use this project for learning or personal use.