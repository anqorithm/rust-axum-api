use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://todos.db".into());
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into());

    let opts = SqliteConnectOptions::from_str(&db_url)?.create_if_missing(true);
    let pool = SqlitePool::connect_with(opts).await?;
    sqlx::migrate!().run(&pool).await?;

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("listening on {addr}");
    axum::serve(listener, rust_axum_api::app(pool)).await?;
    Ok(())
}
