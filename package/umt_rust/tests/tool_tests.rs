use umt_rust::tool::umt_unwrap;

#[test]
fn test_unwrap_some() {
    let val = Some(10);
    let result = umt_unwrap(val, "Should not panic");
    assert_eq!(result, 10);
}

#[test]
#[should_panic(expected = "Value is missing")]
fn test_unwrap_none() {
    let val: Option<i32> = None;
    umt_unwrap(val, "Value is missing");
}
