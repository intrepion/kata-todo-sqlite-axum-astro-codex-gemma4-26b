use axum::{
    extract::State,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use sqlx::sqlite::SqlitePool;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db_pool: SqlitePool,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables
    dotenv().ok();

    // Get database URL from environment
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Create SQLite connection pool
    let db_pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to SQLite");

    let state = AppState { db_pool };

    // Build our application with a single route
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/db-check", get(db_check))
        .with_state(state);

    // Run it with hyper on localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> &'static str {
    "OK"
}

async fn db_check(State(state): State<AppState>) -> &'static str {
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&state.db_pool)
        .await
        .expect("Failed to execute query");
    
    if row.0 == 1 {
        "DB OK"
    } else {
        "DB Error"
    }
}
