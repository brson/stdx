#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct Contact {
    name: String,
    age: u32,
}

fn main() {
    let contact = Contact {
        name: "Brian".to_string(),
        age: 21,
    };

    // Serialize data structures to strings in JSON format
    let contact: String = serde_json::to_string(&contact).unwrap();
    println!("{}", contact);

    // Deserialize data structures from JSON strings
    let contact: Contact = serde_json::from_str(&contact).unwrap();
    println!("{:?}", contact);

    // Convert to arbitrary JSON `Value` type
    let contact: Value = serde_json::to_value(&contact).unwrap();
    println!("{:?}", contact);
}
