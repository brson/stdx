#[macro_use]
extern crate json;

fn main() {
    let parsed = json::parse(r#"
{
    "code": 200,
    "success": true,
    "payload": {
        "features": [
            "awesome",
            "easyAPI",
            "lowLearningCurve"
        ]
    }
}

"#).unwrap();

    let instantiated = object!{
        "code" => 200,
        "success" => true,
        "payload" => object!{
            "features" => array![
                "awesome",
                "easyAPI",
                "lowLearningCurve"
            ]
        }
    };

    assert_eq!(parsed, instantiated);
}
