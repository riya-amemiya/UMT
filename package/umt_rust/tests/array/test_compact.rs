use umt_rust::array::{umt_compact, umt_compact_options};

#[test]
fn test_compact_removes_zeros() {
    assert_eq!(umt_compact(&[0, 1, 0, 2, 0, 3]), vec![1, 2, 3]);
}

#[test]
fn test_compact_all_truthy() {
    assert_eq!(umt_compact(&[1, 2, 3]), vec![1, 2, 3]);
}

#[test]
fn test_compact_all_falsy() {
    assert_eq!(umt_compact(&[0, 0, 0]), Vec::<i32>::new());
}

#[test]
fn test_compact_empty_array() {
    assert_eq!(umt_compact::<i32>(&[]), Vec::<i32>::new());
}

#[test]
fn test_compact_floats() {
    let arr = vec![0.0, 1.0, f64::NAN, 2.0, 0.0, 3.0];
    let result = umt_compact(&arr);
    assert_eq!(result, vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_compact_booleans() {
    assert_eq!(umt_compact(&[false, true, false, true]), vec![true, true]);
}

#[test]
fn test_compact_strings() {
    assert_eq!(umt_compact(&["", "a", "", "b", "c"]), vec!["a", "b", "c"]);
}

#[test]
fn test_compact_options() {
    assert_eq!(
        umt_compact_options(&[None, Some(1), None, Some(2), Some(3)]),
        vec![1, 2, 3]
    );
}

#[test]
fn test_compact_infinity_is_truthy() {
    let result = umt_compact(&[0.0, f64::INFINITY, f64::NEG_INFINITY]);
    assert_eq!(result, vec![f64::INFINITY, f64::NEG_INFINITY]);
}

#[test]
fn test_compact_does_not_modify_original() {
    let original = vec![0, 1, 0, 2];
    let result = umt_compact(&original);
    assert_eq!(result, vec![1, 2]);
    assert_eq!(original, vec![0, 1, 0, 2]);
}

#[test]
fn test_compact_i64() {
    assert_eq!(umt_compact(&[0i64, 1i64, 0i64, 2i64]), vec![1i64, 2i64]);
}

#[test]
fn test_compact_f32() {
    let arr = vec![0.0f32, 1.0f32, f32::NAN, 2.0f32];
    let result = umt_compact(&arr);
    assert_eq!(result, vec![1.0f32, 2.0f32]);
}

#[test]
fn test_compact_string() {
    let arr = vec![
        String::from(""),
        String::from("hello"),
        String::from(""),
        String::from("world"),
    ];
    let result = umt_compact(&arr);
    assert_eq!(result, vec![String::from("hello"), String::from("world")]);
}

#[test]
fn test_compact_option_with_falsy() {
    let arr: Vec<Option<i32>> = vec![None, Some(1), None, Some(2)];
    let result = umt_compact(&arr);
    assert_eq!(result, vec![Some(1), Some(2)]);
}

#[test]
fn test_compact_options_empty() {
    let arr: Vec<Option<i32>> = vec![];
    let result = umt_compact_options(&arr);
    assert!(result.is_empty());
}

#[test]
fn test_compact_options_all_none() {
    let arr: Vec<Option<i32>> = vec![None, None, None];
    let result = umt_compact_options(&arr);
    assert!(result.is_empty());
}
