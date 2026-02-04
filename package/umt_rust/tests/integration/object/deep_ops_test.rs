use umt_rust::object::{umt_merge_deep, umt_pick_deep, Value};
use umt_rust::obj;

#[test]
fn test_merge_deep_pick_deep() {
    let base_config = obj!(
        "app" => obj!(
            "name" => "MyApp",
            "version" => "1.0.0",
            "features" => obj!("auth" => true, "logging" => false)
        )
    );

    let env_config = obj!(
        "app" => obj!(
            "version" => "1.2.0",
            "features" => obj!("logging" => true)
        )
    );

    let merged = umt_merge_deep(&base_config, &env_config);
    let essential = umt_pick_deep(&merged, &["app.name", "app.version"]);

    if let Value::Object(map) = essential {
        let app = map.get("app").unwrap().as_object().unwrap();
        assert_eq!(app.get("name").unwrap(), &Value::String("MyApp".to_string()));
        assert_eq!(app.get("version").unwrap(), &Value::String("1.2.0".to_string()));
    } else {
        panic!("Result should be object");
    }
}
