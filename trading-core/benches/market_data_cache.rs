use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use trading_core::data::types::TickData;
use trading_core::data::cache::MarketDataCache;
use chrono::Utc;
use uuid::Uuid;
use std::time::Duration;

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

fn bench_single_update(c: &mut Criterion) {
    let mut cache = MarketDataCache::new(100);
    let tick = create_test_tick("BTC/USDT", 50000.0, 1.0);

    c.bench_function("single_update", |b| {
        b.iter(|| {
            cache.update(black_box(tick.clone()));
        });
    });
}

fn bench_batch_update(c: &mut Criterion) {
    let mut cache = MarketDataCache::new(100);
    
    let mut group = c.benchmark_group("batch_update");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(10));

    for size in [10, 100, 1000].iter() {
        let ticks: Vec<TickData> = (0..*size)
            .map(|i| create_test_tick(
                &format!("SYMBOL{}/USDT", i % 10),
                50000.0 + i as f64,
                1.0
            ))
            .collect();

        group.bench_with_input(BenchmarkId::from_parameter(size), &ticks, |b, ticks| {
            b.iter(|| {
                cache.batch_update(black_box(ticks.clone()));
            });
        });
    }
    group.finish();
}

fn bench_get_history(c: &mut Criterion) {
    let mut cache = MarketDataCache::new(100);
    let symbol = "BTC/USDT";
    
    // 预填充数据
    for i in 0..1000 {
        cache.update(create_test_tick(
            symbol,
            50000.0 + i as f64,
            1.0
        ));
    }

    let mut group = c.benchmark_group("get_history");
    group.sample_size(100);

    for size in [10, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                black_box(cache.get_history(symbol, size));
            });
        });
    }
    group.finish();
}

fn bench_concurrent_operations(c: &mut Criterion) {
    use std::thread;
    use std::sync::{Arc, RwLock};
    
    let cache = Arc::new(RwLock::new(MarketDataCache::new(100)));
    let symbol = "BTC/USDT";

    let mut group = c.benchmark_group("concurrent_operations");
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("concurrent_read_write", |b| {
        b.iter(|| {
            let cache_clone = Arc::clone(&cache);
            let write_thread = thread::spawn(move || {
                for i in 0..100 {
                    if let Ok(mut cache) = cache_clone.write() {
                        cache.update(create_test_tick(
                            symbol,
                            50000.0 + i as f64,
                            1.0
                        ));
                    }
                }
            });

            let cache_clone = Arc::clone(&cache);
            let read_thread = thread::spawn(move || {
                for _ in 0..100 {
                    if let Ok(cache) = cache_clone.read() {
                        black_box(cache.get_history(symbol, 10));
                    }
                }
            });

            write_thread.join().unwrap();
            read_thread.join().unwrap();
        });
    });

    group.finish();
}

fn bench_market_data_aggregation(c: &mut Criterion) {
    let mut cache = MarketDataCache::new(100);
    let symbol = "BTC/USDT";

    c.bench_function("market_data_aggregation", |b| {
        b.iter(|| {
            for i in 0..100 {
                cache.update(create_test_tick(
                    symbol,
                    50000.0 + (i % 10) as f64, 
                    1.0
                ));
            }
            black_box(cache.get_market_data(symbol));
        });
    });
}

criterion_group!(
    benches,
    bench_single_update,
    bench_batch_update,
    bench_get_history,
    bench_concurrent_operations,
    bench_market_data_aggregation
);
criterion_main!(benches);