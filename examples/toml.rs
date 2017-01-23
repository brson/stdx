extern crate toml;

fn main() {
    let toml = r#"
    [test]
    foo = "bar"
"#;

    let value = toml::Parser::new(toml).parse().unwrap();
    println!("{:?}", value);
}
