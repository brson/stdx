extern crate tempdir;

use std::fs::File;
use std::io::Write;
use tempdir::TempDir;

fn main() {
    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "example".
    let tmp_dir = TempDir::new("example").expect("create temp dir");
    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = File::create(file_path).expect("create temp file");
    writeln!(tmp_file, "Brian was here. Briefly.").expect("write temp file");

    // By closing the `TempDir` explicitly, we can check that it has
    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `tmp_dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.
    drop(tmp_file);
    tmp_dir.close().expect("delete temp dir");
}
