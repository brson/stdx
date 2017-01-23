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

    let mid = v.len() / 2;
    let (lo, hi) = v.split_at_mut(mid);
    rayon::join(|| quick_sort(lo), || quick_sort(hi));
}
