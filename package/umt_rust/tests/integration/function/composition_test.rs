use umt_rust::object::Value;
use umt_rust::tool::umt_parse_json;
use umt_rust::tool::umt_pipe;

#[test]
fn test_combine_curry_with_pipe() {
    // In Rust, curry implementation is specific.
    // umt_curry takes a function and returns a curried version.
    // Assuming simple arithmetic for demonstration.

    let add = |a: i32, b: i32| a + b;
    let multiply = |a: i32, b: i32| a * b;

    // Rust's pipe is strongly typed.
    // pipe(5).map(|x| x + 3).map(|x| x * 2).to_string()

    let result = umt_pipe(5)
        .map(move |x| add(x, 3))
        .map(move |x| multiply(x, 2))
        .map(|x| x.to_string())
        .end();

    assert_eq!(result, "16");
}

#[test]
fn test_complex_data_processing() {
    let json_data = r#"{"users": [{"name": "Alice", "age": 30}, {"name": "Bob", "age": 25}]}"#;

    let result = umt_pipe(json_data.to_string())
        .map(|s| umt_parse_json::<Value>(&s))
        .map(|v| {
            if let Ok(Value::Object(map)) = v {
                if let Some(Value::Array(users)) = map.get("users") {
                    // Filter adults
                    let names: Vec<String> = users
                        .iter()
                        .filter_map(|u| {
                            if let Value::Object(u_map) = u {
                                let age = u_map
                                    .get("age")
                                    .and_then(|a| {
                                        if let Value::Int(i) = a {
                                            Some(*i)
                                        } else {
                                            None
                                        }
                                    })
                                    .unwrap_or(0);
                                if age >= 25 {
                                    u_map.get("name").and_then(|n| {
                                        if let Value::String(s) = n {
                                            Some(s.clone())
                                        } else {
                                            None
                                        }
                                    })
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect();
                    return names.join(", ");
                }
            }
            "".to_string()
        })
        .end();

    assert_eq!(result, "Alice, Bob");
}
