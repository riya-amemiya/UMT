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
