extern crate rand;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    if rng.gen() { // random bool
        println!("i32: {}, u32: {}", rng.gen::<i32>(), rng.gen::<u32>())
    }

    let tuple = rand::random::<(f64, char)>();
    println!("{:?}", tuple)
}
