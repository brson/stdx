extern crate walkdir;

use walkdir::{WalkDir, Error};

fn run() -> Result<(), Error> {
    let wd = WalkDir::new(".");

    for entry in wd {
        let entry = entry?;
        println!("{}", entry.path().display());
    }

    Ok(())
}

fn main() { run().unwrap(); }
