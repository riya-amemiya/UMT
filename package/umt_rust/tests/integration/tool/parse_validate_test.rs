use umt_rust::object::Value;
use umt_rust::tool::umt_parse_json;

#[test]
fn test_parse_json_valid() {
    let json = r#"{"key": 123}"#;
    let result: Value = umt_parse_json(json).expect("Failed to parse");

    if let Value::Object(map) = result {
        assert_eq!(map.get("key").unwrap(), &Value::Int(123));
    } else {
        panic!("Expected object");
    }
}

#[test]
#[should_panic]
fn test_parse_json_invalid() {
    let json = "{key: invalid}";
    let _: Value = umt_parse_json(json).unwrap();
}
