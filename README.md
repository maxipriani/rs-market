<div align="center">

<img src=".github/logo.svg" alt="rs-market" width="150"/>

# rs-market

Gift card marketplace built in Rust. Work in progress.

<a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.93-orange.svg?logo=rust&logoColor=white" alt="Rust 1.93"/></a>
<a href="#"><img src="https://img.shields.io/badge/License-MIT-brightgreen.svg" alt="License MIT"/></a>

</div>

---

## Prerequisites

* Rust 1.93
* Docker & Docker Compose
* Just
* SQLx CLI

## Getting Started

```bash
# Configure environment
cp .env.example .env

# Start Postgres locally (also generates .env.pg with DATABASE_URL for SQLx)
just pg-up

# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features rustls,postgres

# Run database migrations
just sqlx-migrate

# Start the server
cargo run -p monolith
```

## Configuration

`rs-market.toml` is optional and defines the runtime configuration. Every field can be overridden via `RS_MARKET__…` environment variables.

```toml
[database]
# Overridden by env var RS_MARKET__DATABASE__URL
url = "" 

# Connection pool size
max_connections = 5 

# Timeout for acquiring a connection from the pool
acquire_timeout_ms = 3000

# Timeout for each SQL query
statement_timeout_ms = 5000
```
