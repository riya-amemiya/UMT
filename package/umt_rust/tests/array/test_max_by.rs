use umt_rust::array::umt_max_by;

#[test]
fn test_returns_largest() {
    assert_eq!(umt_max_by(&[1, 3, 2], |n| *n), Some(3));
}

#[test]
fn test_returns_none_for_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(umt_max_by(&empty, |n| *n), None);
}

#[test]
fn test_single_element() {
    assert_eq!(umt_max_by(&[7], |n| *n), Some(7));
}

#[test]
fn test_first_wins_ties() {
    assert_eq!(
        umt_max_by(&[(1, 'x'), (2, 'a'), (2, 'b')], |p| p.0),
        Some((2, 'a'))
    );
}

#[test]
fn test_compares_strings() {
    assert_eq!(umt_max_by(&["pear", "apple", "fig"], |s| *s), Some("pear"));
}

#[test]
fn test_nan_never_replaces_maximum() {
    assert_eq!(umt_max_by(&[1.0, f64::NAN, 2.0], |n| *n), Some(2.0));
    assert!(umt_max_by(&[f64::NAN, 1.0], |n| *n).unwrap().is_nan());
}
