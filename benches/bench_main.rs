use criterion::criterion_main;

mod benchmarks;

criterion_main! {
    benchmarks::math_arrmatrix::bench,
    benchmarks::color::bench,
}
