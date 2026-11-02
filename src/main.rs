use std::net::SocketAddr;
use dotenvy::dotenv;
use store_v5::{error::AppError, startup, state::AppState, create_router};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenv().ok();

    // 1. Initialize the global logging subscriber
    startup::logging();
    tracing::info!("Starting application boot sequence...");

    // 2. Establish connection to the PostgreSQL database pool
    let pool = startup::database_connection().await?;
    tracing::info!("Database connection pool established successfully.");

    // 3. Wrap the database pool inside the shared application state
    let state = AppState { pool };
    tracing::info!("Application state initialized.");
    tracing::info!("CORS middleware layer configured.");

    // 4. Build the Axum router using the library helper function
    let app = create_router(state);

    // 5. Resolve server address from environment variable or fallback
    let addr_str = dotenvy::var("SERVER_ADDRESS").unwrap_or_else(|_| {
        tracing::warn!("SERVER_ADDRESS not found in environment, falling back to 127.0.0.1:3000");
        "127.0.0.1:3000".to_string()
    });

    let addr: SocketAddr = addr_str
        .parse()
        .map_err(|e| AppError::Internal(format!("Failed to parse server address: {}", e)))?;

    // 6. Bind the TCP listener and start the Axum web server
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Stationery Server successfully running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}