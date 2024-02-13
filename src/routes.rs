use crate::constants::DEFAULT_COUNT;
use crate::db::get_candles_grouped_by_symbol;
use crate::models::CandleQuery;
use crate::utils::{candles_to_array, grouped_candles_to_json};
use crate::{db, models::Ticker};
use axum::{extract::Query, Extension, Json};
use serde_json::Value;
use sqlx::PgPool;

pub async fn fetch_candles(
    Query(query): Query<CandleQuery>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Vec<serde_json::Value>>>, String> {
    tracing::info!("Fetching candles: {}", query);

    db::get_candles(
        &pool,
        query.symbols,
        &query.interval,
        query.start_time,
        query.end_time,
        query.count.unwrap_or(DEFAULT_COUNT),
    )
    .await
    .map_err(|e| e.to_string())
    .map(candles_to_array)
    .map(Json)
}

pub async fn fetch_candles_grouped(
    Query(query): Query<CandleQuery>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Value>, String> {
    // Note the return type is Json<Value>
    tracing::info!("Fetching candles grouped by symbol: {}", query);

    let grouped_candles = get_candles_grouped_by_symbol(
        &pool,
        query.symbols,
        &query.interval,
        query.start_time,
        query.end_time,
        query.count.unwrap_or(DEFAULT_COUNT),
    )
    .await
    .map_err(|e| e.to_string())?;

    let json_response = grouped_candles_to_json(grouped_candles);

    Ok(Json(json_response))
}

pub async fn fetch_tickers(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<Ticker>>, String> {
    tracing::info!("Fetching all tickers");

    db::get_all_tickers(&pool)
        .await
        .map_err(|e| e.to_string())
        .map(Json)
}
