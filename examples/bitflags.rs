#[macro_use]
extern crate bitflags;

bitflags! {
    flags Flags: u32 {
        const FLAG_A       = 0b00000001,
        const FLAG_B       = 0b00000010,
        const FLAG_C       = 0b00000100,
        const FLAG_ABC     = FLAG_A.bits
                           | FLAG_B.bits
                           | FLAG_C.bits,
    }
}

fn main() {
    let e1 = FLAG_A | FLAG_C;
    let e2 = FLAG_B | FLAG_C;
    assert_eq!((e1 | e2), FLAG_ABC);   // union
    assert_eq!((e1 & e2), FLAG_C);     // intersection
    assert_eq!((e1 - e2), FLAG_A);     // set difference
    assert_eq!(!e2, FLAG_A);           // set complement
}
