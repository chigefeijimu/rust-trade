use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TickData {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub side: String,
    pub trade_id: String,
    pub is_maker: bool,
}

#[derive(Clone)]
pub struct MarketDataManager {
    pub pool: PgPool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MarketDataPoint {
    pub timestamp: DateTime<Utc>,
    pub symbol: String,
    pub price: f64,
    pub volume: f64,
    pub high: f64,
    pub low: f64,
    pub open: f64,
    pub close: f64,
}