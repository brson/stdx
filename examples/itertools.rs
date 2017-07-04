extern crate itertools;

use itertools::{join, max, sorted};

fn main(){
    let a = [3, 2, 5, 8, 7];

    // Combine all iterator elements into one String,
    // seperated by *.
    println!("{:?}", join(&a, "*"));
    // Return the maximum value of the iterable.
    println!("{:?}", max(a.iter()).unwrap());
    // Collect all the iterable's elements into a
    // sorted vector in ascending order.
    println!("{:?}", sorted(a.iter()));
}