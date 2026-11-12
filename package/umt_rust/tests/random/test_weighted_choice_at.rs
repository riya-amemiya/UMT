use umt_rust::random::{WeightedItem, umt_weighted_choice, umt_weighted_choice_at};

fn items() -> Vec<WeightedItem<&'static str>> {
    vec![
        WeightedItem {
            value: "a",
            weight: 1.0,
        },
        WeightedItem {
            value: "skip",
            weight: 0.0,
        },
        WeightedItem {
            value: "negative",
            weight: -3.0,
        },
        WeightedItem {
            value: "b",
            weight: 2.0,
        },
        WeightedItem {
            value: "c",
            weight: 3.0,
        },
    ]
}

#[test]
fn test_weighted_choice_at_empty_returns_none() {
    let empty: Vec<WeightedItem<i32>> = vec![];
    assert_eq!(umt_weighted_choice_at(&empty, 0.0), None);
}

#[test]
fn test_weighted_choice_at_selects_by_cumulative_range() {
    let items = items();
    assert_eq!(umt_weighted_choice_at(&items, 0.0), Some(&"a"));
    assert_eq!(umt_weighted_choice_at(&items, 0.999), Some(&"a"));
    assert_eq!(umt_weighted_choice_at(&items, 1.0), Some(&"b"));
    assert_eq!(umt_weighted_choice_at(&items, 2.999), Some(&"b"));
    assert_eq!(umt_weighted_choice_at(&items, 3.0), Some(&"c"));
    assert_eq!(umt_weighted_choice_at(&items, 5.999), Some(&"c"));
}

#[test]
fn test_weighted_choice_at_target_beyond_total_returns_last() {
    let items = items();
    assert_eq!(umt_weighted_choice_at(&items, 100.0), Some(&"c"));
}

#[test]
fn test_weighted_choice_at_all_non_positive_returns_last() {
    let items = vec![
        WeightedItem {
            value: 1,
            weight: 0.0,
        },
        WeightedItem {
            value: 2,
            weight: -1.0,
        },
        WeightedItem {
            value: 3,
            weight: f64::NAN,
        },
    ];
    assert_eq!(umt_weighted_choice_at(&items, 0.0), Some(&3));
}

#[test]
fn test_weighted_choice_at_nan_target_returns_first() {
    let items = items();
    assert_eq!(umt_weighted_choice_at(&items, f64::NAN), Some(&"a"));
    let single = vec![WeightedItem {
        value: "only",
        weight: f64::INFINITY,
    }];
    assert_eq!(umt_weighted_choice_at(&single, f64::NAN), Some(&"only"));
}

#[test]
fn test_weighted_choice_at_single_item() {
    let items = vec![WeightedItem {
        value: "only",
        weight: 2.0,
    }];
    assert_eq!(umt_weighted_choice_at(&items, 0.0), Some(&"only"));
    assert_eq!(umt_weighted_choice_at(&items, 5.0), Some(&"only"));
}

#[test]
fn test_weighted_choice_never_returns_non_positive_weight_items() {
    let items = items();
    for _ in 0..500 {
        let picked = umt_weighted_choice(&items).unwrap();
        assert!(picked == "a" || picked == "b" || picked == "c");
    }
}

#[test]
fn test_weighted_choice_empty_returns_none() {
    let empty: Vec<WeightedItem<i32>> = vec![];
    assert_eq!(umt_weighted_choice(&empty), None);
}
