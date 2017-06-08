extern crate memmap;

use memmap::{Mmap, Protection};
use std::env;
use std::io;
use std::str;

fn run() -> Result<(), io::Error> {
    let mut args = env::args().skip(1);
    let input = args.next().expect("incorrect argument");

    let map = Mmap::open_path(input, Protection::Read)?;
    unsafe {
        let all_bytes = map.as_slice();
        if let Ok(file_str) = str::from_utf8(all_bytes) {
            println!("{}", file_str);
        } else {
            println!("not utf8");
        }
    }
    Ok(())
}

fn main() { run().unwrap() }
