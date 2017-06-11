extern crate walkdir;

fn main(){
use walkdir::WalkDir;

for entry in WalkDir::new("directory_name") {
    let entry = entry.unwrap();
    println!("{}", entry.path().display());
   }
}
