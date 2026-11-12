use std::cell::Cell;
use umt_rust::array::umt_sort_by;

#[test]
fn test_sorts_by_single_selector() {
    let ages = vec![30, 20, 25];
    assert_eq!(umt_sort_by(&ages, &[&|n: &i32| *n]), vec![20, 25, 30]);
}

#[test]
fn test_later_selectors_break_ties() {
    let words = vec!["bb", "a", "ab", "c"];
    let by_length = |s: &&str| s.len().to_string();
    let by_value = |s: &&str| s.to_string();
    assert_eq!(
        umt_sort_by(&words, &[&by_length, &by_value]),
        vec!["a", "c", "ab", "bb"]
    );
}

#[test]
fn test_is_stable_for_equal_keys() {
    let items = vec![(1, 'a'), (0, 'b'), (1, 'c'), (0, 'd')];
    let result = umt_sort_by(&items, &[&|item: &(i32, char)| item.0]);
    assert_eq!(result, vec![(0, 'b'), (0, 'd'), (1, 'a'), (1, 'c')]);
}

#[test]
fn test_is_stable_for_large_inputs() {
    let items: Vec<(usize, usize)> = (0..200).map(|i| (i % 3, i)).collect();
    let result = umt_sort_by(&items, &[&|item: &(usize, usize)| item.0]);
    let mut expected = items.clone();
    expected.sort_by_key(|item| item.0);
    assert_eq!(result, expected);
}

#[test]
fn test_without_selectors_keeps_order() {
    let source = vec![3, 1, 2];
    let selectors: &[&dyn Fn(&i32) -> i32] = &[];
    assert_eq!(umt_sort_by(&source, selectors), vec![3, 1, 2]);
}

#[test]
fn test_empty_and_single_element() {
    let empty: Vec<i32> = vec![];
    assert!(umt_sort_by(&empty, &[&|n: &i32| *n]).is_empty());
    assert_eq!(umt_sort_by(&[1], &[&|n: &i32| *n]), vec![1]);
}

#[test]
fn test_calls_each_selector_once_per_element() {
    let calls = Cell::new(0);
    let selector = |n: &i32| {
        calls.set(calls.get() + 1);
        *n
    };
    umt_sort_by(&[5, 3, 4, 1, 2], &[&selector]);
    assert_eq!(calls.get(), 5);
}

#[test]
fn test_nan_keys_compare_as_equal() {
    let values = vec![2.0, f64::NAN, 1.0];
    let result = umt_sort_by(&values, &[&|n: &f64| *n]);
    assert_eq!(result.len(), 3);
    assert!(result[1].is_nan());
}

#[test]
fn test_descending_through_negated_key() {
    assert_eq!(umt_sort_by(&[1, 3, 2], &[&|n: &i32| -n]), vec![3, 2, 1]);
}
