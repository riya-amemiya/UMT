use umt_rust::predicate::umt_is_not_nullish;

#[test]
fn returns_false_for_none() {
    assert!(!umt_is_not_nullish(&None::<i32>));
    assert!(!umt_is_not_nullish(&None::<String>));
}

#[test]
fn returns_true_for_some_zero() {
    assert!(umt_is_not_nullish(&Some(0)));
}

#[test]
fn returns_true_for_some_empty_string() {
    assert!(umt_is_not_nullish(&Some("")));
}

#[test]
fn returns_true_for_some_false() {
    assert!(umt_is_not_nullish(&Some(false)));
}

#[test]
fn returns_true_for_some_nan() {
    assert!(umt_is_not_nullish(&Some(f64::NAN)));
}

#[test]
fn returns_true_for_some_vec() {
    assert!(umt_is_not_nullish(&Some(Vec::<i32>::new())));
}
