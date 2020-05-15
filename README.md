# rayon-array-add-benchmarks

Attempts to benchmark three versions of an `add_all` function to add all elements from one vec into another.

## The three versions:

```rust
const CHUNK_SIZE: usize = 1024;

pub fn par_add_all_join<T: Send + Sync + AddAssign + Copy>(target: &mut [T], src: &[T]) {
    assert_eq!(target.len(), src.len());

    if target.len() < CHUNK_SIZE {
        for (a, b) in target.iter_mut().zip(src.iter()) {
            *a += *b;
        }
    } else {
        let midpoint = target.len() / 2;
        let (left_target, right_target) = target.split_at_mut(midpoint);
        let (left_src, right_src) = src.split_at(midpoint);
        rayon::join(
            || par_add_all_join(left_target, left_src),
            || par_add_all_join(right_target, right_src),
        );
    }
}

pub fn seq_add_all<T: Send + Sync + AddAssign + Copy>(target: &mut [T], src: &[T]) {
    assert_eq!(target.len(), src.len());
    for (a, b) in target.iter_mut().zip(src.iter()) {
        *a += *b;
    }
}

pub fn par_add_all_par_iter<T: Send + Sync + AddAssign + Copy>(target: &mut [T], src: &[T]) {
    assert_eq!(target.len(), src.len());
    target
        .par_iter_mut()
        .zip(src.par_iter())
        .for_each(|(a, b)| {
            *a += *b;
        });
}
```

## Run the benchmark

To run the benchmark locally, install rust and use `cargo bench`.

## Benchmark Results

Benchmark results on my dual-core laptop with an i7-7600U CPU @ 2.80 Ghz and
LPDDR3 ram.

```text
par_add_all_join        time:   [11.485 us 11.670 us 11.911 us]
Found 16 outliers among 100 measurements (16.00%)
  3 (3.00%) high mild
  13 (13.00%) high severe

seq_add_all             time:   [2.0696 us 2.0930 us 2.1172 us]
Found 3 outliers among 100 measurements (3.00%)
  2 (2.00%) high mild
  1 (1.00%) high severe

par_add_all_par_iter    time:   [21.167 us 21.571 us 22.014 us]
Found 9 outliers among 100 measurements (9.00%)
  8 (8.00%) high mild
  1 (1.00%) high severe
```

I think more benchmark results on more parallel computers are needed to have
actually serious results, though. PRs adding more benchmark results welcome.
