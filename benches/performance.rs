use std::time::Duration;

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use feishu_sdk::core::{Cache, Config, InMemoryCache};

fn cache_benchmark(c: &mut Criterion) {
    let cache = InMemoryCache::new();

    c.bench_function("cache_set", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| {
                let cache = &cache;
                async move {
                    cache
                        .set(
                            "key".to_string(),
                            "value".to_string(),
                            Duration::from_secs(60),
                        )
                        .await
                }
            })
    });

    c.bench_function("cache_get", |b| {
        b.to_async(tokio::runtime::Runtime::new().unwrap())
            .iter(|| {
                let cache = &cache;
                async move { cache.get("key").await }
            })
    });
}

fn config_benchmark(c: &mut Criterion) {
    c.bench_function("config_build", |b| {
        b.iter(|| Config::builder(black_box("app_id"), black_box("app_secret")).build())
    });
}

criterion_group!(benches, cache_benchmark, config_benchmark);
criterion_main!(benches);
