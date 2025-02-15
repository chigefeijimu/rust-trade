use std::collections::{HashMap, VecDeque};
use std::sync::RwLock;
use super::types::{TickData, MarketDataPoint};

const MAX_HISTORY_SIZE: usize = 1000;

#[derive(Debug)]
pub struct TickBuffer {
    data: VecDeque<TickData>,
    latest_market_data: Option<MarketDataPoint>,
}

impl TickBuffer {
    pub fn new() -> Self {
        Self {
            data: VecDeque::with_capacity(MAX_HISTORY_SIZE),
            latest_market_data: None,
        }
    }

    pub fn push(&mut self, tick: TickData) {
        if self.data.len() >= MAX_HISTORY_SIZE {
            self.data.pop_front();
        }
        self.update_market_data(&tick);
        self.data.push_back(tick);
    }

    fn update_market_data(&mut self, tick: &TickData) {
        match &mut self.latest_market_data {
            Some(market_data) => {
                if market_data.timestamp.timestamp() == tick.timestamp.timestamp() {
                    market_data.high = market_data.high.max(tick.price);
                    market_data.low = market_data.low.min(tick.price);
                    market_data.close = tick.price;
                    market_data.volume += tick.volume;
                } else {
                    self.latest_market_data = Some(MarketDataPoint {
                        timestamp: tick.timestamp,
                        symbol: tick.symbol.clone(),
                        price: tick.price,
                        volume: tick.volume,
                        high: tick.price,
                        low: tick.price,
                        open: tick.price,
                        close: tick.price,
                    });
                }
            }
            None => {
                self.latest_market_data = Some(MarketDataPoint {
                    timestamp: tick.timestamp,
                    symbol: tick.symbol.clone(),
                    price: tick.price,
                    volume: tick.volume,
                    high: tick.price,
                    low: tick.price,
                    open: tick.price,
                    close: tick.price,
                });
            }
        }
    }

    pub fn latest(&self, n: usize) -> Vec<TickData> {
        let n = n.min(self.data.len());
        self.data.iter().rev().take(n).cloned().collect()
    }

    pub fn get_market_data(&self) -> Option<MarketDataPoint> {
        self.latest_market_data.clone()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

pub struct MarketDataCache {
    data: HashMap<String, RwLock<TickBuffer>>,
    max_symbols: usize,
}

impl MarketDataCache {
    pub fn new(max_symbols: usize) -> Self {
        Self {
            data: HashMap::with_capacity(max_symbols),
            max_symbols,
        }
    }

    pub fn update(&mut self, tick: TickData) {
        let symbol = tick.symbol.clone();
        if let Some(buffer) = self.data.get(&symbol) {
            if let Ok(mut buffer) = buffer.write() {
                buffer.push(tick);
            }
        } else if self.data.len() < self.max_symbols {
            let mut buffer = TickBuffer::new();
            buffer.push(tick);
            self.data.insert(symbol, RwLock::new(buffer));
        }
    }

    pub fn batch_update(&mut self, ticks: Vec<TickData>) {
        for tick in ticks {
            self.update(tick);
        }
    }

    pub fn get_history(&self, symbol: &str, n: usize) -> Option<Vec<TickData>> {
        self.data.get(symbol).and_then(|buffer| {
            buffer.read().ok().map(|guard| guard.latest(n))
        })
    }

    pub fn get_market_data(&self, symbol: &str) -> Option<MarketDataPoint> {
        self.data.get(symbol).and_then(|buffer| {
            buffer.read().ok().and_then(|guard| guard.get_market_data())
        })
    }

    pub fn get_all_market_data(&self) -> Vec<MarketDataPoint> {
        self.data.values()
            .filter_map(|buffer| {
                buffer.read().ok().and_then(|guard| guard.get_market_data())
            })
            .collect()
    }

    pub fn get_symbols(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }

    pub fn clear_symbol(&mut self, symbol: &str) {
        self.data.remove(symbol);
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;
    use chrono::Utc;

    fn create_test_tick(symbol: &str, price: f64, volume: f64) -> TickData {
        TickData {
            timestamp: Utc::now(),
            symbol: symbol.to_string(),
            price,
            volume,
            side: "buy".to_string(),
            trade_id: Uuid::new_v4().to_string(),
            is_maker: false,
        }
    }

    #[test]
    fn test_tick_buffer() {
        let mut buffer = TickBuffer::new();
        let tick = create_test_tick("BTC/USDT", 50000.0, 1.0);
        buffer.push(tick);

        assert_eq!(buffer.len(), 1);
        
        let market_data = buffer.get_market_data().unwrap();
        assert_eq!(market_data.symbol, "BTC/USDT");
        assert_eq!(market_data.price, 50000.0);
    }

    #[test]
    fn test_market_data_cache() {
        let mut cache = MarketDataCache::new(10);
        
        let tick = create_test_tick("BTC/USDT", 50000.0, 1.0);
        cache.update(tick);
        
        let ticks = vec![
            create_test_tick("ETH/USDT", 3000.0, 2.0),
            create_test_tick("BNB/USDT", 400.0, 5.0),
        ];
        cache.batch_update(ticks);
        
        let history = cache.get_history("BTC/USDT", 1).unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].price, 50000.0);
        
        let market_data = cache.get_market_data("BTC/USDT").unwrap();
        assert_eq!(market_data.symbol, "BTC/USDT");
        assert_eq!(market_data.price, 50000.0);
        
        let all_market_data = cache.get_all_market_data();
        assert_eq!(all_market_data.len(), 3);
    }
}