use umt_rust::array::{umt_count_by, umt_count_by_indexed};

#[test]
fn test_count_by_floor() {
    let arr = vec![6.1, 4.2, 6.3];
    let result = umt_count_by(&arr, |x| *x as i32);
    assert_eq!(result.len(), 2);
    assert_eq!(result.get(&4), Some(&1));
    assert_eq!(result.get(&6), Some(&2));
}

#[test]
fn test_count_by_string_length() {
    let array = vec!["one", "two", "three"];
    let result = umt_count_by(&array, |s| s.len());
    assert_eq!(result.len(), 2);
    assert_eq!(result.get(&3), Some(&2));
    assert_eq!(result.get(&5), Some(&1));
}

#[test]
fn test_count_by_struct_field() {
    struct Item {
        kind: &'static str,
    }
    let array = vec![
        Item { kind: "fruit" },
        Item { kind: "vegetable" },
        Item { kind: "fruit" },
    ];
    let result = umt_count_by(&array, |item| item.kind);
    assert_eq!(result.get(&"fruit"), Some(&2));
    assert_eq!(result.get(&"vegetable"), Some(&1));
}

#[test]
fn test_count_by_empty() {
    let array: Vec<i32> = vec![];
    let result = umt_count_by(&array, |x| *x);
    assert!(result.is_empty());
}

#[test]
fn test_count_by_constant_key() {
    let result = umt_count_by(&[1, 2, 3], |_| "all");
    assert_eq!(result.len(), 1);
    assert_eq!(result.get(&"all"), Some(&3));
}

#[test]
fn test_count_by_numeric_keys() {
    let result = umt_count_by(&[1, 2, 1, 3, 2, 1], |n| *n);
    assert_eq!(result.get(&1), Some(&3));
    assert_eq!(result.get(&2), Some(&2));
    assert_eq!(result.get(&3), Some(&1));
}

#[test]
fn test_count_by_boolean_derived_keys() {
    let result = umt_count_by(&[true, false, true], |b| if *b { "yes" } else { "no" });
    assert_eq!(result.get(&"yes"), Some(&2));
    assert_eq!(result.get(&"no"), Some(&1));
}

#[test]
fn test_count_by_single_element() {
    let result = umt_count_by(&["x"], |s| *s);
    assert_eq!(result.len(), 1);
    assert_eq!(result.get(&"x"), Some(&1));
}

#[test]
fn test_count_by_unicode_keys() {
    let array = vec!["\u{3042}", "\u{3044}", "\u{3042}"];
    let result = umt_count_by(&array, |s| s.to_string());
    assert_eq!(result.get("\u{3042}"), Some(&2));
    assert_eq!(result.get("\u{3044}"), Some(&1));
}

#[test]
fn test_count_by_indexed_receives_index() {
    let array = vec!["a", "b", "c", "d", "e"];
    let result = umt_count_by_indexed(&array, |_, index| index % 2);
    assert_eq!(result.get(&0), Some(&3));
    assert_eq!(result.get(&1), Some(&2));
}

#[test]
fn test_count_by_indexed_receives_value() {
    let array = vec![10, 20, 30];
    let result = umt_count_by_indexed(&array, |value, index| *value + index as i32);
    assert_eq!(result.get(&10), Some(&1));
    assert_eq!(result.get(&21), Some(&1));
    assert_eq!(result.get(&32), Some(&1));
}

#[test]
fn test_count_by_indexed_empty() {
    let array: Vec<i32> = vec![];
    let result = umt_count_by_indexed(&array, |x, _| *x);
    assert!(result.is_empty());
}
