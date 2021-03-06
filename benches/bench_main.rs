use criterion::criterion_main;
mod benchmarks;

criterion_main!(
    benchmarks::square::bench_square,
    benchmarks::generics::bench_generics,
);
