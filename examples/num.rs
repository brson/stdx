extern crate num;

use num::FromPrimitive;
use num::bigint::BigInt;
use num::rational::{Ratio, BigRational};

fn approx_sqrt(number: u64, iterations: usize) -> BigRational {
    let start: Ratio<BigInt>
        = Ratio::from_integer(FromPrimitive::from_u64(number).unwrap());

    let mut approx = start.clone();

    for _ in 0..iterations {
        approx = (&approx + (&start / &approx)) /
            Ratio::from_integer(FromPrimitive::from_u64(2).unwrap());
    }

    approx
}

fn main() {
    println!("{}", approx_sqrt(10, 4)); // prints 4057691201/1283082416
}
