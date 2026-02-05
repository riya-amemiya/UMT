<<<<<<< HEAD
use umt_rust::tool::{umt_unwrap, umt_unwrap_or, umt_unwrap_or_else, umt_unwrap_or_panic};

#[test]
fn test_unwrap_with_some_value() {
    let value: Option<i32> = Some(42);
    let result = umt_unwrap(value, "Value was None");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
}

#[test]
fn test_unwrap_with_none_value() {
    let value: Option<i32> = None;
    let result = umt_unwrap(value, "Value was None");
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().message, "Value was None");
}

#[test]
fn test_unwrap_with_string_value() {
    let value: Option<String> = Some("hello".to_string());
    let result = umt_unwrap(value, "String was None");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "hello");
}

#[test]
fn test_unwrap_with_struct() {
    #[derive(Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let value: Option<Point> = Some(Point { x: 1, y: 2 });
    let result = umt_unwrap(value, "Point was None");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), Point { x: 1, y: 2 });
}

#[test]
fn test_unwrap_error_display() {
    let value: Option<i32> = None;
    let result = umt_unwrap(value, "Custom error message");
    let err = result.unwrap_err();
    assert_eq!(format!("{}", err), "Custom error message");
}

#[test]
fn test_unwrap_or_with_some_value() {
    let value: Option<i32> = Some(42);
    let result = umt_unwrap_or(value, 0);
    assert_eq!(result, 42);
}

#[test]
fn test_unwrap_or_with_none_value() {
    let value: Option<i32> = None;
    let result = umt_unwrap_or(value, 0);
    assert_eq!(result, 0);
}

#[test]
fn test_unwrap_or_else_with_some_value() {
    let value: Option<i32> = Some(42);
    let result = umt_unwrap_or_else(value, || 0);
    assert_eq!(result, 42);
}

#[test]
fn test_unwrap_or_else_with_none_value() {
    let value: Option<i32> = None;
    let result = umt_unwrap_or_else(value, || 100 + 1);
    assert_eq!(result, 101);
}

#[test]
fn test_unwrap_or_else_closure_not_called_when_some() {
    let value: Option<i32> = Some(42);
    let mut called = false;
    let result = umt_unwrap_or_else(value, || {
        called = true;
        0
    });
    assert_eq!(result, 42);
    assert!(!called);
}

#[test]
fn test_unwrap_or_panic_with_some_value() {
    let value: Option<i32> = Some(42);
    let result = umt_unwrap_or_panic(value, "Value was None");
    assert_eq!(result, 42);
}

#[test]
#[should_panic(expected = "Value was None")]
fn test_unwrap_or_panic_with_none_value() {
    let value: Option<i32> = None;
    umt_unwrap_or_panic(value, "Value was None");
}

#[test]
fn test_unwrap_preserves_complex_types() {
    let value: Option<Vec<i32>> = Some(vec![1, 2, 3]);
    let result = umt_unwrap(value, "Vec was None");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), vec![1, 2, 3]);
}

#[test]
fn test_unwrap_with_zero_value() {
    let value: Option<i32> = Some(0);
    let result = umt_unwrap(value, "Value was None");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 0);
}

#[test]
fn test_unwrap_with_empty_string() {
    let value: Option<String> = Some(String::new());
    let result = umt_unwrap(value, "String was None");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "");
}
||||||| 30d5753
=======
use umt_rust::tool::umt_unwrap;

#[test]
fn test_unwrap_some() {
    let val = Some(10);
    let result = umt_unwrap(val, "Error");
    assert_eq!(result, 10);
}

#[test]
#[should_panic(expected = "Error")]
fn test_unwrap_none() {
    let val: Option<i32> = None;
    umt_unwrap(val, "Error");
}
>>>>>>> 36e5fbd009729e51174857904826bd81d5477247
