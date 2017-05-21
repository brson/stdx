extern crate toml;

use toml::Value;

fn main() {
    let toml = r#"
    [test]
    foo = "bar"
"#;

    let value = toml.parse::<Value>().unwrap();
    println!("{:?}", value);
}
