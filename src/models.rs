use core::fmt;

use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use sqlx::{types::BigDecimal, FromRow};

#[derive(Serialize, Deserialize)]
pub struct Ticker {
    pub id: i32,
    pub symbol: String,
    /*     pub price: Option<String>,
    pub volume: Option<String>,
    pub dailyhigh: Option<String>,
    pub dailylow: Option<String>,
    pub maincoinsymbol: Option<String>,
    pub quotecoinsymbol: Option<String>, */
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Candle {
    #[serde(rename(serialize = "s"))]
    pub tickersymbol: String,
    #[serde(serialize_with = "serialize_as_timestamp")]
    #[serde(rename(serialize = "t"))]
    pub bucket: chrono::NaiveDateTime,
    #[serde(rename(serialize = "o"))]
    pub open: BigDecimal,
    #[serde(serialize_with = "serialize_big_decimal")]
    #[serde(rename(serialize = "h"))]
    pub high: BigDecimal,
    #[serde(serialize_with = "serialize_big_decimal")]
    #[serde(rename(serialize = "l"))]
    pub low: BigDecimal,
    #[serde(serialize_with = "serialize_big_decimal")]
    #[serde(rename(serialize = "c"))]
    pub close: BigDecimal,
    #[serde(serialize_with = "serialize_big_decimal")]
    #[serde(rename(serialize = "v"))]
    pub volume: BigDecimal,
}

fn serialize_big_decimal<S>(big_decimal: &BigDecimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let rounded = big_decimal.with_scale(2); // Adjust scale as needed
    serializer.serialize_str(&rounded.to_string())
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CandleWithoutSymbol {
    #[serde(serialize_with = "serialize_as_timestamp")]
    #[serde(rename(serialize = "t"))]
    pub bucket: chrono::NaiveDateTime,
    #[serde(rename(serialize = "o"))]
    pub open: BigDecimal,
    #[serde(rename(serialize = "h"))]
    pub high: BigDecimal,
    #[serde(rename(serialize = "l"))]
    pub low: BigDecimal,
    #[serde(rename(serialize = "c"))]
    pub close: BigDecimal,
    #[serde(rename(serialize = "v"))]
    pub volume: BigDecimal,
}

// Conversion from CandleWithSymbol to CandleWithoutSymbol
impl From<Candle> for CandleWithoutSymbol {
    fn from(candle_with_symbol: Candle) -> Self {
        CandleWithoutSymbol {
            bucket: candle_with_symbol.bucket,
            open: candle_with_symbol.open,
            high: candle_with_symbol.high,
            low: candle_with_symbol.low,
            close: candle_with_symbol.close,
            volume: candle_with_symbol.volume,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct CandleQuery {
    #[serde(default, deserialize_with = "deserialize_symbols")]
    pub symbols: Vec<String>,
    pub interval: String,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub count: Option<i64>,
}

impl fmt::Display for CandleQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "symbols: {:?}, interval: {}, start_time: {:?}, end_time: {:?}, count: {:?}",
            self.symbols, self.interval, self.start_time, self.end_time, self.count
        )
    }
}

// Custom deserializer that splits a comma-separated string into a Vec<String>
fn deserialize_symbols<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.split(',').map(|s| s.to_string()).collect())
}

fn serialize_as_timestamp<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let timestamp = date.timestamp();
    serializer.serialize_i64(timestamp)
}
