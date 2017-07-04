extern crate fnv;

use fnv::FnvHashMap;

fn main() {
	let mut map = FnvHashMap::default();
	map.insert(1, "one");
	map.insert(2, "two");
	map.insert(3, "three");

	for (number, word) in map.iter() {
	    println!("Number {}: {}", number, word);
	}

	map.remove(&(2));
	println!("The length of HashMap is {}.", map.len());
	println!("The first element is {}.", map.get(&(1)).unwrap());
}

