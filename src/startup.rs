use crate::error::AppError;
use sqlx::{
    postgres::{PgConnectOptions, PgPool, PgPoolOptions},
    ConnectOptions,
};
use std::{str::FromStr, time::Duration};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn logging() {
    let filter = EnvFilter::builder()
        .with_default_directive(tracing::Level::INFO.into())
        .from_env_lossy();

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_env_filter(filter)
        .finish();

    let _ = tracing::subscriber::set_global_default(subscriber);
}

pub async fn database_connection() -> Result<PgPool, AppError> {
    tracing::debug!("Setting up database connection");
    
    if let Err(e) = dotenvy::dotenv() {
        println!("Could not load .env file: {:?}", e);
    }
    
    let db_url = dotenvy::var("DATABASE_URL")
        .map_err(|_| AppError::Internal("Failed to get database url from env".to_string()))?;

    let options = PgConnectOptions::from_str(&db_url)
        .map_err(|e| AppError::Internal(format!("Failed to parse database url: {}", e)))?
        .disable_statement_logging();

    let pg_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_with(options)
        .await?; // Automatically turns sqlx::Error into AppError via #[from]

    tracing::debug!("Successfully connected");

    // Run database migrations
    sqlx::migrate!("./migrations")
        .run(&pg_pool)
        .await?; // Automatically turns sqlx::migrate error into AppError

    tracing::debug!("Migrations applied successfully");

    Ok(pg_pool)
}