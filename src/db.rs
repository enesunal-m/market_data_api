use std::collections::HashMap;

use crate::models::{Candle, CandleWithoutSymbol};
use sqlx::PgPool;

pub async fn get_candles(
    pool: &PgPool,
    symbols: Vec<String>,
    interval: &str,
    start_time: Option<i64>,
    end_time: Option<i64>,
    count: i64,
) -> Result<Vec<Candle>, sqlx::Error> {
    let table_name = match interval {
        "3m" => "candle_3m",
        "5m" => "candle_5m",
        "15m" => "candle_15m",
        "30m" => "candle_30m",
        "1h" => "candle_1h",
        "4h" => "candle_4h",
        "12h" => "candle_12h",
        "1d" => "candle_1d",
        "3d" => "candle_3d",
        "1w" => "candle_1w",
        _ => "candle_1m", // Default to "candle_1m" or handle error
    };

    // Start building the query dynamically based on provided parameters
    let mut query = format!("SELECT * FROM {} WHERE tickersymbol = ANY($1)", table_name);

    // Tracking the current position of bind parameters
    let mut current_bind_pos = 2;

    // Conditionally add time constraints to the query
    if start_time.is_some() {
        query += &format!(" AND bucket >= ${}", current_bind_pos);
        current_bind_pos += 1;
    }
    if end_time.is_some() {
        query += &format!(" AND bucket <= ${}", current_bind_pos);
        current_bind_pos += 1;
    }

    query += &format!(" ORDER BY bucket DESC LIMIT ${}", current_bind_pos);

    // Prepare the query
    let mut db_query = sqlx::query_as::<_, Candle>(&query);

    // Bind parameters
    db_query = db_query.bind(symbols);

    if let Some(start) = start_time {
        db_query = db_query.bind(chrono::NaiveDateTime::from_timestamp_opt(start, 0));
    }
    if let Some(end) = end_time {
        db_query = db_query.bind(chrono::NaiveDateTime::from_timestamp_opt(end, 0));
    }

    db_query = db_query.bind(count);

    // Execute the query
    db_query.fetch_all(pool).await
}

pub async fn get_candles_grouped_by_symbol(
    pool: &PgPool,
    symbols: Vec<String>,
    interval: &str,
    start_time: Option<i64>,
    end_time: Option<i64>,
    count: i64,
) -> Result<HashMap<String, Vec<CandleWithoutSymbol>>, sqlx::Error> {
    let candles = match get_candles(pool, symbols, interval, start_time, end_time, count).await {
        Ok(results) => results,
        Err(err) => return Err(err),
    };

    let mut grouped_candles: HashMap<String, Vec<CandleWithoutSymbol>> = HashMap::new();
    for candle in candles {
        grouped_candles
            .entry(candle.tickersymbol.clone())
            .or_insert_with(Vec::new)
            .push(candle.into());
    }

    Ok(grouped_candles)
}

use crate::models::Ticker;

pub async fn get_all_tickers(pool: &PgPool) -> Result<Vec<Ticker>, sqlx::Error> {
    let tickers = match sqlx::query_as!(Ticker, "SELECT id, symbol FROM ticker")
        .fetch_all(pool)
        .await
    {
        Ok(result) => result,
        Err(err) => {
            tracing::error!("Error occured while fetching all tickers: {}", err);
            return Err(err);
        }
    };

    Ok(tickers)
}
