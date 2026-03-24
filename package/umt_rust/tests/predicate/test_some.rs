use umt_rust::predicate::umt_some;

#[test]
fn returns_true_when_at_least_one_predicate_passes() {
    let is_zero_or_negative = umt_some(vec![
        Box::new(|n: &i64| *n == 0),
        Box::new(|n: &i64| *n < 0),
    ]);
    assert!(is_zero_or_negative(&0));
    assert!(is_zero_or_negative(&-5));
}

#[test]
fn returns_false_when_no_predicate_passes() {
    let is_zero_or_negative = umt_some(vec![
        Box::new(|n: &i64| *n == 0),
        Box::new(|n: &i64| *n < 0),
    ]);
    assert!(!is_zero_or_negative(&5));
}

#[test]
fn short_circuits_on_first_success() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    let second_called = Arc::new(AtomicBool::new(false));
    let second_called_clone = Arc::clone(&second_called);
    let combined = umt_some(vec![
        Box::new(|_: &i64| true),
        Box::new(move |_: &i64| {
            second_called_clone.store(true, Ordering::SeqCst);
            false
        }),
    ]);
    combined(&0);
    assert!(!second_called.load(Ordering::SeqCst));
}

#[test]
fn handles_single_predicate() {
    let single = umt_some(vec![Box::new(|n: &i64| *n > 0)]);
    assert!(single(&1));
    assert!(!single(&-1));
}

#[test]
fn returns_false_for_no_predicates() {
    let never: Box<dyn Fn(&i64) -> bool> = Box::new(umt_some(Vec::new()));
    assert!(!never(&0));
}
