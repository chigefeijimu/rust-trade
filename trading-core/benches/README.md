# Performance Benchmark Results

## Test Environment
- Test File: `market_data_cache.rs`
- Backend: Plotters
- Build Type: Release

## Detailed Benchmark Results

### Single Update Operation
```
Time: [190.33 ns 200.08 ns 212.03 ns]
Measurements: 100
Outliers: 1 (1.00%)
- High mild: 1 (1.00%)
```

### Batch Update Operations
#### Batch Size: 10
```
Time: [1.7923 µs 1.8361 µs 1.8884 µs]
Measurements: 50
Outliers: 3 (6.00%)
- High mild: 1 (2.00%)
- High severe: 2 (4.00%)
```

#### Batch Size: 100
```
Time: [17.281 µs 17.454 µs 17.643 µs]
Measurements: 50
Outliers: 2 (4.00%)
- High severe: 2 (4.00%)
```

#### Batch Size: 1000
```
Time: [170.69 µs 172.26 µs 173.99 µs]
Measurements: 50
Outliers: 2 (4.00%)
- High mild: 1 (2.00%)
- High severe: 1 (2.00%)
```

### Historical Data Retrieval
#### Fetch Size: 10
```
Time: [1.1326 µs 1.1407 µs 1.1490 µs]
Measurements: 100
Outliers: 6 (6.00%)
- High mild: 5 (5.00%)
- High severe: 1 (1.00%)
```

#### Fetch Size: 100
```
Time: [14.833 µs 14.961 µs 15.098 µs]
Measurements: 100
Outliers: 8 (8.00%)
- High mild: 8 (8.00%)
```

#### Fetch Size: 500
```
Time: [68.451 µs 70.060 µs 71.937 µs]
Measurements: 100
Outliers: 9 (9.00%)
- High mild: 4 (4.00%)
- High severe: 5 (5.00%)
```

### Concurrent Operations
```
Operation: concurrent_read_write
Time: [513.29 µs 555.74 µs 603.02 µs]
```

### Market Data Aggregation
```
Time: [26.978 µs 27.298 µs 27.669 µs]
Measurements: 100
Outliers: 6 (6.00%)
- High mild: 3 (3.00%)
- High severe: 3 (3.00%)
```

## Time Format
- ns: nanoseconds (1 ns = 0.000001 ms)
- µs: microseconds (1 µs = 0.001 ms)
- Time ranges are presented as [minimum median maximum]

## Measurement Details
- Each benchmark includes multiple iterations
- Outliers are classified as either "mild" or "severe"
- Sample sizes vary between tests:
  - Single update: 100 measurements
  - Batch update: 50 measurements per batch size
  - Get history: 100 measurements per fetch size
  - Market data aggregation: 100 measurements