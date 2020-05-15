use std::ops::AddAssign;

use rayon::prelude::*;

pub fn par_add_all_join<T: Send + Sync + AddAssign + Copy>(
    target: &mut [T],
    src: &[T],
    chunk_size: usize,
) {
    assert_eq!(target.len(), src.len());

    if target.len() < chunk_size {
        for (a, b) in target.iter_mut().zip(src.iter()) {
            *a += *b;
        }
    } else {
        let midpoint = target.len() / 2;
        let (left_target, right_target) = target.split_at_mut(midpoint);
        let (left_src, right_src) = src.split_at(midpoint);
        rayon::join(
            || par_add_all_join(left_target, left_src, chunk_size),
            || par_add_all_join(right_target, right_src, chunk_size),
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

#[test]
fn test_par_add_all_join() {
    let v1: &[u32] = &[1; 1024 * 10];
    let mut v2 = vec![2; 1024 * 10];
    let v3: &[u32] = &[3; 1024 * 10];

    par_add_all_join(&mut v2, v1, 1024);

    assert_eq!(v2, v3);
}

#[test]
fn test_seq_add_all() {
    let v1: &[u32] = &[1; 1024 * 10];
    let mut v2 = vec![2; 1024 * 10];
    let v3: &[u32] = &[3; 1024 * 10];

    seq_add_all(&mut v2, v1);

    assert_eq!(v2, v3);
}

#[test]
fn test_add_all_par_iter() {
    let v1: &[u32] = &[1; 1024 * 10];
    let mut v2 = vec![2; 1024 * 10];
    let v3: &[u32] = &[3; 1024 * 10];

    par_add_all_par_iter(&mut v2, v1);

    assert_eq!(v2, v3);
}
