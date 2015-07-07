extern crate stdx;

#[test]
fn import() {
    #![allow(unused_imports)]
    use stdx::bitflags;
}

#[test]
fn rand_example() {
    use stdx::rand::random;

    println!("{}", random::<i32>())
}
