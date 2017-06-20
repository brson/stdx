extern crate walkdir;

use walkdir::{WalkDir, Error};

fn run(wd: WalkDir) -> Result<(), Error> {
    for entry in wd {
        let entry = entry?;
        println!("{}", entry.path().display());
    }
    Ok(())
}

fn main() {
    run(WalkDir::new(".")).unwrap();
}
