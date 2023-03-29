use criterion::{criterion_group, criterion_main};

mod bench_array_create;
mod bench_array_sum;
mod bench_array_product;

criterion_group!(benches,
    // =====
    bench_array_create::create_array_1d_1m_benchmark,
    bench_array_create::create_array_1d_10m_benchmark,
    bench_array_create::create_array_5d_1m_benchmark,
    bench_array_create::create_array_5d_10m_benchmark,
    bench_array_create::create_array_10d_1m_benchmark,
    bench_array_create::create_array_10d_10m_benchmark,
    // =====
    bench_array_sum::array_sum_1d_1m_benchmark,
    bench_array_sum::array_sum_1d_10m_benchmark,
    bench_array_sum::array_sum_5d_1m_benchmark,
    bench_array_sum::array_sum_5d_10m_benchmark,
    bench_array_sum::array_sum_10d_1m_benchmark,
    bench_array_sum::array_sum_10d_10m_benchmark,
    // =====
    bench_array_product::array_product_1d_1m_benchmark,
    bench_array_product::array_product_1d_10m_benchmark,
    bench_array_product::array_product_5d_1m_benchmark,
    bench_array_product::array_product_5d_10m_benchmark,
    bench_array_product::array_product_10d_1m_benchmark,
    bench_array_product::array_product_10d_10m_benchmark,
);
criterion_main!(benches);
