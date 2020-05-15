use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon_array_add_benchmarks::*;

fn criterion_benchmark(c: &mut Criterion) {
    let v1: &[u32] = &[1; 1024 * 10];
    let mut v2 = vec![2; 1024 * 10];
    c.bench_function("par_add_all_join", |b| {
        b.iter(|| par_add_all_join(black_box(&mut v2), black_box(&v1)))
    });
    c.bench_function("seq_add_all", |b| {
        b.iter(|| seq_add_all(black_box(&mut v2), black_box(&v1)))
    });
    c.bench_function("par_add_all_par_iter", |b| {
        b.iter(|| par_add_all_par_iter(black_box(&mut v2), black_box(&v1)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
