extern crate reqwest;

use std::collections::HashMap;
use std::io::{BufRead, BufReader};

fn main() {
    // Make a GET request
    let resp = reqwest::get("https://www.rust-lang.org").unwrap();
    assert!(resp.status().is_success());

    let lines = BufReader::new(resp)
                          .lines()
                          .filter_map(|l| l.ok())
                          .take(10);
    for line in lines {
        println!("{}", line);
    }

    // Make a POST request
    let client = reqwest::Client::new().unwrap();
    let res = client.post("http://httpbin.org/post").unwrap()
        .body("the exact body that is sent")
        .send();

    // Convert to/from JSON automatically
    let mut map = HashMap::new();
    map.insert("lang", "rust");
    map.insert("body", "json");

    // This will POST a body of `{"lang":"rust","body":"json"}`
    let client = reqwest::Client::new().unwrap();
    let res = client.post("http://httpbin.org/post").unwrap()
        .json(&map).unwrap()
        .send();
}
