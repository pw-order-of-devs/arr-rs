use std::time::Duration;
use criterion::{Criterion, BenchmarkGroup};
use criterion::measurement::WallTime;
use arr_rs::prelude::*;

#[allow(dead_code)]
#[cfg(not(tarpaulin_include))]
pub(crate) fn config<'a>(c: &'a mut Criterion, name: &'a str) -> BenchmarkGroup<'a, WallTime> {
    let mut criterion = c.benchmark_group(name);
    criterion
        .confidence_level(0.9)
        .significance_level(0.1)
        .sample_size(10)
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(10));
    criterion
}


#[allow(dead_code)]
pub(crate) fn create_array_1d_1m_benchmark(c: &mut Criterion) {
    let mut criterion = config(c, "array create");
    criterion.bench_function("dim: 1D, elements: 1M", |b| b.iter(|| Array::<f64>::rand(vec![1000000])));
    criterion.finish();
}

#[allow(dead_code)]
pub(crate) fn create_array_1d_10m_benchmark(c: &mut Criterion) {
    let mut criterion = config(c, "array create");
    criterion.bench_function("dim: 1D, elements: 10M", |b| b.iter(|| Array::<f64>::rand(vec![10000000])));
    criterion.finish();
}

#[allow(dead_code)]
pub(crate) fn create_array_5d_1m_benchmark(c: &mut Criterion) {
    let mut criterion = config(c, "array create");
    criterion.bench_function("dim: 5D, elements: 1M", |b| b.iter(|| Array::<f64>::rand(vec![16; 5])));
    criterion.finish();
}

#[allow(dead_code)]
pub(crate) fn create_array_5d_10m_benchmark(c: &mut Criterion) {
    let mut criterion = config(c, "array create");
    criterion.bench_function("dim: 5D, elements: 10M", |b| b.iter(|| Array::<f64>::rand(vec![25; 5])));
    criterion.finish();
}

#[allow(dead_code)]
pub(crate) fn create_array_10d_1m_benchmark(c: &mut Criterion) {
    let mut criterion = config(c, "array create");
    criterion.bench_function("dim: 10D, elements: 1M", |b| b.iter(|| Array::<f64>::rand(vec![4; 10])));
    criterion.finish();
}

#[allow(dead_code)]
pub(crate) fn create_array_10d_10m_benchmark(c: &mut Criterion) {
    let mut criterion = config(c, "array create");
    criterion.bench_function("dim: 10D, elements: 10M", |b| b.iter(|| Array::<f64>::rand(vec![5; 10])));
    criterion.finish();
}
