use umt_rust::predicate::umt_every;

#[test]
fn returns_true_when_all_predicates_pass() {
    let is_positive_even = umt_every(vec![
        Box::new(|n: &i64| *n > 0),
        Box::new(|n: &i64| *n % 2 == 0),
    ]);
    assert!(is_positive_even(&4));
    assert!(is_positive_even(&8));
}

#[test]
fn returns_false_when_any_predicate_fails() {
    let is_positive_even = umt_every(vec![
        Box::new(|n: &i64| *n > 0),
        Box::new(|n: &i64| *n % 2 == 0),
    ]);
    assert!(!is_positive_even(&-2));
    assert!(!is_positive_even(&3));
}

#[test]
fn short_circuits_on_first_failure() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};

    let second_called = Arc::new(AtomicBool::new(false));
    let second_called_clone = Arc::clone(&second_called);
    let combined = umt_every(vec![
        Box::new(|_: &i64| false),
        Box::new(move |_: &i64| {
            second_called_clone.store(true, Ordering::SeqCst);
            true
        }),
    ]);
    combined(&0);
    assert!(!second_called.load(Ordering::SeqCst));
}

#[test]
fn handles_single_predicate() {
    let single = umt_every(vec![Box::new(|n: &i64| *n > 0)]);
    assert!(single(&1));
    assert!(!single(&-1));
}

#[test]
fn returns_true_for_no_predicates() {
    let always: Box<dyn Fn(&i64) -> bool> = Box::new(umt_every(Vec::new()));
    assert!(always(&0));
}
