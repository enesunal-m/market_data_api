use std::collections::HashMap;

use serde_json::{Number, Value};

use crate::models::{Candle, CandleWithoutSymbol};

pub fn candle_to_array(candle: Candle) -> Vec<serde_json::Value> {
    vec![
        Value::from(candle.bucket.timestamp()),
        Value::String(candle.open.to_string()),
        Value::String(candle.high.to_string()),
        Value::String(candle.low.to_string()),
        Value::String(candle.close.to_string()),
        Value::String(candle.volume.to_string()),
    ]
}

pub fn candles_to_array(candles: Vec<Candle>) -> Vec<Vec<serde_json::Value>> {
    candles.into_iter().map(candle_to_array).collect()
}

fn candle_without_symbol_to_array(candle: &CandleWithoutSymbol) -> Vec<Value> {
    vec![
        Value::from(candle.bucket.timestamp()),
        Value::String(candle.open.to_string()),
        Value::String(candle.high.to_string()),
        Value::String(candle.low.to_string()),
        Value::String(candle.close.to_string()),
        Value::String(candle.volume.to_string()),
    ]
}

pub fn grouped_candles_to_json(
    grouped_candles: HashMap<String, Vec<CandleWithoutSymbol>>,
) -> Value {
    let mut result = HashMap::new();

    for (symbol, candles) in grouped_candles {
        let candle_arrays: Vec<Vec<Value>> =
            candles.iter().map(candle_without_symbol_to_array).collect();
        result.insert(
            symbol,
            Value::Array(
                candle_arrays
                    .iter()
                    .map(|c| Value::Array(c.clone()))
                    .collect(),
            ),
        );
    }

    Value::Object(result.into_iter().map(|(k, v)| (k, v)).collect())
}
