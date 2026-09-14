# Spec: Server Initialization

## Goal
To establish a functional, production-ready foundation for the Axum-based REST API. This includes project scaffolding, dependency management, and a basic connectivity check to the SQLite database.

## Requirements

### 1. Project Scaffolding
- Create a `server/` directory at the project root.
- Initialize a new Rust Cargo project within `server/`.

### 2. Dependency Management
The `Cargo.toml` must include the following crates:
- `axum`: Web framework.
- `tokio`: Async runtime.
- `serde` & `serde_json`: For JSON serialization/deserialization.
- `sqlx`: For asynchronous SQL interactions (with `sqlite` feature).
- `uuid`: For generating unique identifiers.
- `dotenvy`: For managing environment variables.
- `tracing` & `tracing-subscriber`: For structured logging.

### 3. Core Infrastructure
- **Environment Configuration**: Support loading a `DATABASE_URL` from a `.env` file.
- **Database Connection**: Implement a mechanism to initialize a `sqlx::SqlitePool` on startup.
- **Basic Routing**: Implement at least one health-check endpoint (`/health`) that returns a 200 OK status.
- **Logging**: Configure `tracing` to output logs to stdout for development.

## Success Criteria
- `cargo build` completes without errors.
- The server can be started and responds to `/health` via `curl` or a browser.
- The server successfully initializes the SQLite connection pool without panicking.
