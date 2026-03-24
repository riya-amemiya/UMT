use umt_rust::predicate::umt_not;

#[test]
fn negates_a_truthy_predicate() {
    let is_even = |n: &i64| n % 2 == 0;
    let is_odd = umt_not(is_even);

    assert!(is_odd(&1));
    assert!(!is_odd(&2));
    assert!(is_odd(&3));
    assert!(!is_odd(&4));
}

#[test]
fn negates_a_string_predicate() {
    let is_empty = |s: &&str| s.is_empty();
    let is_not_empty = umt_not(is_empty);

    assert!(!is_not_empty(&""));
    assert!(is_not_empty(&"hello"));
}

#[test]
fn double_negation_returns_original_result() {
    let is_positive = |n: &i64| *n > 0;
    let double_not = umt_not(umt_not(is_positive));

    assert!(double_not(&5));
    assert!(!double_not(&-1));
}
