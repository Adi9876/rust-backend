# Rust Backend - Pizza API

A REST API built with Rust and Actix-web for managing pizzas. The application uses SurrealDB as the database backend.

## Features

- Get all pizzas
- Create a new pizza
- Update an existing pizza by UUID

## Prerequisites

- Rust
- SurrealDB running on localhost:8000

## Setup

1. Start SurrealDB:
```bash
surreal start file:pizzadb --user root --password root
```

2. Build and run the application:
```bash
cargo run
```

The server will start on `http://127.0.0.1:8080`



## Dependencies

- actix-web - Web framework
- async-trait - Async trait support
- derive_more - Additional derive macros
- serde - Serialization and deserialization
- serde_json - JSON support
- surrealdb - Database client
- uuid - UUID generation
- validator - Request validation

## Database Configuration

The application connects to SurrealDB with the following settings:
- Host: 127.0.0.1:8000
- Username: root
- Password: root
- Namespace: surreal
- Database: pizzas
