extern crate stdx;

#[test]
fn import() {
    #![allow(unused_imports)]
    use stdx::bitflags;
}

#[test]
fn rand_example() {
    use stdx::rand::{self, Rng};

    let mut rng = rand::thread_rng();
    println!("{}", rng.gen::<i32>())
}
