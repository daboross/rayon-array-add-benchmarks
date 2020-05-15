use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rayon_array_add_benchmarks::*;

fn criterion_benchmark(c: &mut Criterion) {
    {
        let mut kib_array = c.benchmark_group("320 kib array");
        let v1: &[u32] = &[1; 1024 * 10];
        let mut v2 = vec![2; 1024 * 10];
        kib_array.bench_function("seq_add_all", |b| {
            b.iter(|| seq_add_all(black_box(&mut v2), black_box(&v1)))
        });
        kib_array.bench_function("par_add_all_par_iter", |b| {
            b.iter(|| par_add_all_par_iter(black_box(&mut v2), black_box(&v1)))
        });
        kib_array.bench_function("par_add_all_join_1024_chunks", |b| {
            b.iter(|| par_add_all_join(black_box(&mut v2), black_box(&v1), black_box(1024)))
        });
    }

    {
        let mut mib_array = c.benchmark_group("320 mib array");
        mib_array.measurement_time(std::time::Duration::from_secs(60));
        let v1: &[u32] = &[1; 1024 * 1024 * 10];
        let mut v2 = vec![2; 1024 * 1024 * 10];
        mib_array.bench_function("seq_add_all", |b| {
            b.iter(|| seq_add_all(black_box(&mut v2), black_box(&v1)))
        });
        mib_array.bench_function("par_add_all_par_iter", |b| {
            b.iter(|| par_add_all_par_iter(black_box(&mut v2), black_box(&v1)))
        });
        mib_array.bench_function("par_add_all_join_1024_chunks", |b| {
            b.iter(|| par_add_all_join(black_box(&mut v2), black_box(&v1), black_box(1024)))
        });
        mib_array.bench_function("par_add_all_join_1048576_chunks", |b| {
            b.iter(|| par_add_all_join(black_box(&mut v2), black_box(&v1), black_box(1024 * 1024)))
        });
    }

    {
        let mut gib_array = c.benchmark_group("gib array");
        gib_array.sample_size(30);
        gib_array.measurement_time(std::time::Duration::from_secs(60));

        let v1: &[u32] = &[1; 1024 * 1024 * 32];
        let mut v2 = vec![2; 1024 * 1024 * 32];
        gib_array.bench_function("seq_add_all", |b| {
            b.iter(|| seq_add_all(black_box(&mut v2), black_box(&v1)))
        });
        gib_array.bench_function("par_add_all_par_iter", |b| {
            b.iter(|| par_add_all_par_iter(black_box(&mut v2), black_box(&v1)))
        });
        gib_array.bench_function("par_add_all_join_1024_chunks", |b| {
            b.iter(|| par_add_all_join(black_box(&mut v2), black_box(&v1), black_box(1024)))
        });
        gib_array.bench_function("par_add_all_join_1048576_chunks", |b| {
            b.iter(|| par_add_all_join(black_box(&mut v2), black_box(&v1), black_box(1024 * 1024)))
        });
        gib_array.bench_function("par_add_all_join_4194304_chunks", |b| {
            b.iter(|| {
                par_add_all_join(
                    black_box(&mut v2),
                    black_box(&v1),
                    black_box(1024 * 1024 * 4),
                )
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
