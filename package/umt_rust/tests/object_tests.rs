use umt_rust::obj;
use umt_rust::object::{
    Value, umt_key_by, umt_merge, umt_merge_deep, umt_omit, umt_pick, umt_pick_deep,
};

#[test]
fn test_key_by() {
    let users = vec![
        obj!("id" => "a", "age" => 20),
        obj!("id" => "b", "age" => 30),
    ];
    let collection = Value::from(users);

    let result = umt_key_by(&collection, "id");

    if let Value::Object(map) = result {
        assert!(map.contains_key("a"));
        assert!(map.contains_key("b"));

        let user_a = map.get("a").unwrap();
        if let Value::Object(u) = user_a {
            assert_eq!(u.get("age").unwrap(), &Value::Int(20));
        } else {
            panic!("Expected object");
        }
    } else {
        panic!("Expected object result");
    }
}

#[test]
fn test_merge() {
    let obj1 = obj!("a" => 1, "b" => 2);
    let obj2 = obj!("b" => 3, "c" => 4);

    let result = umt_merge(&obj1, &[obj2]);

    if let Value::Object(map) = result {
        assert_eq!(map.get("a").unwrap(), &Value::Int(1));
        assert_eq!(map.get("b").unwrap(), &Value::Int(3)); // Overwritten
        assert_eq!(map.get("c").unwrap(), &Value::Int(4));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_merge_deep() {
    let obj1 = obj!("a" => 1, "nested" => obj!("x" => 10, "y" => 20));
    let obj2 = obj!("b" => 2, "nested" => obj!("y" => 30, "z" => 40));

    let result = umt_merge_deep(&obj1, &[obj2]);

    if let Value::Object(map) = result {
        assert_eq!(map.get("a").unwrap(), &Value::Int(1));
        assert_eq!(map.get("b").unwrap(), &Value::Int(2));

        let nested = map.get("nested").unwrap();
        if let Value::Object(n) = nested {
            assert_eq!(n.get("x").unwrap(), &Value::Int(10));
            assert_eq!(n.get("y").unwrap(), &Value::Int(30)); // Overwritten
            assert_eq!(n.get("z").unwrap(), &Value::Int(40));
        } else {
            panic!("Nested should be object");
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_omit() {
    let obj = obj!("a" => 1, "b" => 2, "c" => 3);
    let result = umt_omit(&obj, &["a", "c"]);

    if let Value::Object(map) = result {
        assert!(!map.contains_key("a"));
        assert!(map.contains_key("b"));
        assert!(!map.contains_key("c"));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_pick() {
    let obj = obj!("a" => 1, "b" => 2, "c" => 3);
    let result = umt_pick(&obj, &["a", "c"]);

    if let Value::Object(map) = result {
        assert!(map.contains_key("a"));
        assert!(!map.contains_key("b"));
        assert!(map.contains_key("c"));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_pick_deep() {
    let obj = obj!("a" => 1, "nested" => obj!("x" => 10, "y" => 20));
    let result = umt_pick_deep(&obj, &["a", "nested.y"]);

    if let Value::Object(map) = result {
        assert!(map.contains_key("a"));

        let nested = map.get("nested").unwrap();
        if let Value::Object(n) = nested {
            assert!(!n.contains_key("x"));
            assert!(n.contains_key("y"));
        } else {
            panic!("Nested should be object");
        }
    } else {
        panic!("Expected object");
    }
}
