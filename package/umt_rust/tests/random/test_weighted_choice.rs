use std::collections::HashSet;
use umt_rust::random::{WeightedItem, umt_weighted_choice};

#[test]
fn test_returns_one_of_the_weighted_values() {
    let items = vec![
        WeightedItem {
            value: "a",
            weight: 1.0,
        },
        WeightedItem {
            value: "b",
            weight: 1.0,
        },
    ];
    let allowed: HashSet<&str> = ["a", "b"].into_iter().collect();
    for _ in 0..50 {
        assert!(allowed.contains(umt_weighted_choice(&items).unwrap()));
    }
}

#[test]
fn test_favors_heavier_weights_statistically() {
    let items = vec![
        WeightedItem {
            value: "a",
            weight: 1.0,
        },
        WeightedItem {
            value: "b",
            weight: 9.0,
        },
    ];
    let mut b_count = 0;
    let trials = 2000;
    for _ in 0..trials {
        if umt_weighted_choice(&items) == Some("b") {
            b_count += 1;
        }
    }
    assert!(b_count as f64 / trials as f64 > 0.8);
}

#[test]
fn test_ignores_non_positive_weights() {
    let items = vec![
        WeightedItem {
            value: "skip",
            weight: 0.0,
        },
        WeightedItem {
            value: "winner",
            weight: 5.0,
        },
    ];
    for _ in 0..50 {
        assert_eq!(umt_weighted_choice(&items), Some("winner"));
    }
}

#[test]
fn test_treats_negative_weights_as_ignored() {
    let items = vec![
        WeightedItem {
            value: "a",
            weight: -3.0,
        },
        WeightedItem {
            value: "b",
            weight: 5.0,
        },
    ];
    for _ in 0..50 {
        assert_eq!(umt_weighted_choice(&items), Some("b"));
    }
}

#[test]
fn test_returns_none_for_empty_slice() {
    let items: Vec<WeightedItem<&str>> = vec![];
    assert_eq!(umt_weighted_choice(&items), None);
}
