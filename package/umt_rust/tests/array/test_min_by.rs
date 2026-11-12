use umt_rust::array::umt_min_by;

#[test]
fn test_returns_smallest() {
    assert_eq!(umt_min_by(&[3, 1, 2], |n| *n), Some(1));
}

#[test]
fn test_returns_none_for_empty() {
    let empty: Vec<i32> = vec![];
    assert_eq!(umt_min_by(&empty, |n| *n), None);
}

#[test]
fn test_single_element() {
    assert_eq!(umt_min_by(&[7], |n| *n), Some(7));
}

#[test]
fn test_first_wins_ties() {
    assert_eq!(
        umt_min_by(&[(2, 'x'), (1, 'a'), (1, 'b')], |p| p.0),
        Some((1, 'a'))
    );
}

#[test]
fn test_compares_strings() {
    assert_eq!(umt_min_by(&["pear", "apple", "fig"], |s| *s), Some("apple"));
}

#[test]
fn test_nan_never_replaces_minimum() {
    assert_eq!(umt_min_by(&[2.0, f64::NAN, 1.0], |n| *n), Some(1.0));
    assert!(umt_min_by(&[f64::NAN, 1.0], |n| *n).unwrap().is_nan());
}
