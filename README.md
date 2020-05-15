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
320 kib array/seq_add_all                                                                             
                        time:   [1.6336 us 1.6351 us 1.6369 us]
Found 21 outliers among 100 measurements (21.00%)
  3 (3.00%) low severe
  3 (3.00%) low mild
  6 (6.00%) high mild
  9 (9.00%) high severe
320 kib array/par_add_all_par_iter                                                                             
                        time:   [17.487 us 17.563 us 17.647 us]
Found 12 outliers among 100 measurements (12.00%)
  1 (1.00%) low mild
  5 (5.00%) high mild
  6 (6.00%) high severe
320 kib array/par_add_all_join_1024_chunks                                                                             
                        time:   [9.9634 us 9.9947 us 10.032 us]
Found 13 outliers among 100 measurements (13.00%)
  2 (2.00%) low mild
  8 (8.00%) high mild
  3 (3.00%) high severe

320 mib array/seq_add_all                                                                            
                        time:   [5.5562 ms 5.5585 ms 5.5609 ms]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe
320 mib array/par_add_all_par_iter                                                                            
                        time:   [5.2304 ms 5.2932 ms 5.3608 ms]
Found 16 outliers among 100 measurements (16.00%)
  6 (6.00%) high mild
  10 (10.00%) high severe
320 mib array/par_add_all_join_1024_chunks                                                                            
                        time:   [5.1380 ms 5.1402 ms 5.1427 ms]
Found 12 outliers among 100 measurements (12.00%)
  4 (4.00%) high mild
  8 (8.00%) high severe
320 mib array/par_add_all_join_1048576_chunks                                                                            
                        time:   [5.1421 ms 5.1444 ms 5.1467 ms]
Found 8 outliers among 100 measurements (8.00%)
  3 (3.00%) high mild
  5 (5.00%) high severe

gib array/seq_add_all   time:   [18.221 ms 18.227 ms 18.233 ms]                                  
Found 2 outliers among 30 measurements (6.67%)
  1 (3.33%) high mild
  1 (3.33%) high severe
gib array/par_add_all_par_iter                                                                            
                        time:   [16.714 ms 16.718 ms 16.724 ms]
Found 2 outliers among 30 measurements (6.67%)
  1 (3.33%) high mild
  1 (3.33%) high severe
gib array/par_add_all_join_1024_chunks                                                                            
                        time:   [16.691 ms 16.696 ms 16.700 ms]
Found 3 outliers among 30 measurements (10.00%)
  1 (3.33%) high mild
  2 (6.67%) high severe
gib array/par_add_all_join_1048576_chunks                                                                            
                        time:   [16.706 ms 16.712 ms 16.718 ms]
Found 4 outliers among 30 measurements (13.33%)
  3 (10.00%) high mild
  1 (3.33%) high severe
gib array/par_add_all_join_4194304_chunks                                                                            
                        time:   [16.715 ms 17.092 ms 17.615 ms]
Found 5 outliers among 30 measurements (16.67%)
  2 (6.67%) high mild
  3 (10.00%) high severe
```

I think more benchmark results on more parallel computers are needed to have
actually serious results, though. PRs adding more benchmark results welcome.
