extern crate rayon;

use rayon::prelude::*;

fn main() {
    let mut input = (0..1000).collect::<Vec<_>>();

    // Calculate the sum of squares
    let sq_sum = input.par_iter()
                      .map(|&i| i * i)
                      .sum();

    // Increment each element in parallel
    input.par_iter_mut()
        .for_each(|p| *p += 1);

    // Parallel quicksort
    let mut input = (0..1000).rev().collect::<Vec<_>>();
    quick_sort(&mut input);
}

fn quick_sort<T: PartialOrd + Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| quick_sort(lo), || quick_sort(hi));
}

fn partition<T: PartialOrd + Send>(v: &mut [T]) -> usize {
    let pivot = v.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}
