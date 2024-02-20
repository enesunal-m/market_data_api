use axum::{routing::get, Extension, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

use tower_http::compression::{CompressionLayer, DefaultPredicate};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

mod constants;
mod db;
mod models;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize logging
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    let fmt_layer = fmt::layer()
        .with_target(true) // include the target (e.g., module path) in log output
        .with_writer(std::io::stdout)
        .with_ansi(true); // logs to stdout

    let file_appender = tracing_appender::rolling::daily("./logs", "market_data_api.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().with_writer(non_blocking).with_ansi(false); // logs to file

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(file_layer)
        .init();

    // Database setup remains unchanged
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    let predicate = DefaultPredicate::new();

/*     let comression_layer: CompressionLayer = CompressionLayer::new()
        .no_deflate()
        .no_gzip()
        .no_zstd()
        .no_br()
        .quality(tower_http::CompressionLevel::Best)
        .compress_when(predicate); // Add a predicate function as an argument to the compress_when() method */

    let market_routes = Router::new()
        .route("/symbols", get(routes::fetch_tickers))
        .route("/candles", get(routes::fetch_candles))
    //    .layer(comression_layer)
        .route("/candles/grouped", get(routes::fetch_candles_grouped))
        .layer(Extension(pool));

    let api_routes = Router::new().nest("/market", market_routes);

    // Router setup remains unchanged
    let app = Router::new().nest("/api", api_routes);

    // Server setup with improved logging
    let port = 3000;
    let host = "127.0.0.1";
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind port");

    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}
