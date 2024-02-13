# Market Data REST API - Rust with Axum

This is the REST API service for market data. It is written in Rust using the Axum web framework and the Tokio runtime. Return values of the API are like Binance API.

## Features

- [x] Get available symbols.
- [x] Get market data by symbol and interval and limit and start time or end time.
- [x] Get grouped market data by symbol, interval, limit, start time and end time.

## Usage

Look to Postman collection for usage.

## Installation

```bash
git clone
cd market_data_api
cargo run
```

Don't forget to install Rust and Cargo before running the project. You can install them from [here](https://www.rust-lang.org/tools/install).

Also, fill the .env file with your database credentials.
