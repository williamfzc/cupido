use criterion::{criterion_group, criterion_main, Criterion};
use cupido::collector;
use cupido::collector::Config;
use std::time::Duration;

const TEST_DIR: &str = ".";

fn criterion_benchmark(c: &mut Criterion) {
    tracing_subscriber::fmt::init();

    let mut group = c.benchmark_group("repo_walk");
    group.measurement_time(Duration::from_secs(30));
    group.sample_size(10);

    group.bench_function("repo_walking_default_config", |b| {
        b.iter(|| {
            let mut config = Config::default();
            config.repo_path = String::from(TEST_DIR);
            collector::walk(config);
        })
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
