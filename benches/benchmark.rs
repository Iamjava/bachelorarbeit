use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ba_lib::a;

pub fn criterion_benchmark(c: &mut Criterion) {

    let mut group = c.benchmark_group("benchgroup");

    group.warm_up_time(std::time::Duration::from_secs(10));
    group.measurement_time(std::time::Duration::from_secs(15));
    group.sample_size(1000);
    group.bench_function("test", |b| b.iter(|| a()));
    group.finish();
}

// https://www.youtube.com/watch?v=3iC3FVS6UXQ
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);