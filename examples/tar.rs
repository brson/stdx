// Note: this is the same example as tar.rs

extern crate flate2;
extern crate tar;

use flate2::read::GzDecoder;
use std::env;
use std::fs::File;
use std::io::{self, BufReader};
use tar::Archive;

fn run() -> Result<(), io::Error> {
    let mut args = env::args().skip(1);
    let tarball = args.next().expect("incorrect argument");
    let outdir = args.next().expect("incorrect arguments");

    let archive = File::open(tarball)?;
    let archive = BufReader::new(archive);
    let archive = GzDecoder::new(archive)?;
    let mut archive = Archive::new(archive);

    archive.unpack(outdir)?;

    Ok(())
}

fn main() { run().unwrap() }
