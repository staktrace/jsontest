extern crate rustc_serialize;
extern crate json;
extern crate serde_json;

fn main() {
    let questionable_json = r#"{"foo": "c\ud838"}"#;
    match json::parse(&questionable_json) {
        Ok(_) => println!("json passed"),
        Err(e) => println!("json failed: {}", e),
    };
    match rustc_serialize::json::Json::from_str(&questionable_json) {
        Ok(_) => println!("rustc_serialize passed"),
        Err(e) => println!("rustc_serialize failed: {}", e),
    };
    let v: serde_json::Result<serde_json::Value> = serde_json::from_str(&questionable_json);
    match v {
        Ok(_) => println!("serde_json passed"),
        Err(e) => println!("serde_json failed: {}", e),
    }
}
