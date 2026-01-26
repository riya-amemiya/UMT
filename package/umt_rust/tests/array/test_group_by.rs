use umt_rust::array::{umt_group_by, umt_group_by_indexed};

#[test]
fn test_group_by_odd_even() {
    let array = vec![1, 2, 3, 4, 5];
    let result = umt_group_by(&array, |num| if num % 2 == 0 { "even" } else { "odd" });
    assert_eq!(result.get(&"odd"), Some(&vec![1, 3, 5]));
    assert_eq!(result.get(&"even"), Some(&vec![2, 4]));
}

#[test]
fn test_group_by_string_length() {
    let array = vec!["one", "two", "three", "four", "five"];
    let result = umt_group_by(&array, |s| s.len());
    assert_eq!(result.get(&3), Some(&vec!["one", "two"]));
    assert_eq!(result.get(&4), Some(&vec!["four", "five"]));
    assert_eq!(result.get(&5), Some(&vec!["three"]));
}

#[test]
fn test_group_by_floor() {
    let arr = vec![6.1, 4.2, 6.3];
    let result = umt_group_by(&arr, |x| *x as i32);
    assert_eq!(result.get(&4), Some(&vec![4.2]));
    assert_eq!(result.get(&6), Some(&vec![6.1, 6.3]));
}

#[test]
fn test_group_by_empty() {
    let array: Vec<i32> = vec![];
    let result = umt_group_by(&array, |num| if num % 2 == 0 { "even" } else { "odd" });
    assert!(result.is_empty());
}

#[test]
fn test_group_by_single_group() {
    let array = vec![1, 2, 3, 4, 5];
    let result = umt_group_by(&array, |_| "all");
    assert_eq!(result.get(&"all"), Some(&vec![1, 2, 3, 4, 5]));
}

#[test]
fn test_group_by_modulo() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let result = umt_group_by(&arr, |x| x % 3);
    assert_eq!(result.get(&0), Some(&vec![3, 6, 9]));
    assert_eq!(result.get(&1), Some(&vec![1, 4, 7]));
    assert_eq!(result.get(&2), Some(&vec![2, 5, 8]));
}

#[test]
fn test_group_by_first_char() {
    let arr = vec!["apple", "apricot", "banana", "blueberry", "cherry"];
    let result = umt_group_by(&arr, |s| s.chars().next().unwrap_or('\0'));
    assert_eq!(result.get(&'a'), Some(&vec!["apple", "apricot"]));
    assert_eq!(result.get(&'b'), Some(&vec!["banana", "blueberry"]));
    assert_eq!(result.get(&'c'), Some(&vec!["cherry"]));
}

#[test]
fn test_group_by_boolean() {
    let array = vec![true, false, true, false, true];
    let result = umt_group_by(&array, |&b| b);
    assert_eq!(result.get(&true), Some(&vec![true, true, true]));
    assert_eq!(result.get(&false), Some(&vec![false, false]));
}

#[test]
fn test_group_by_struct() {
    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        category: String,
        name: String,
    }

    let items = vec![
        Item {
            category: "fruit".to_string(),
            name: "apple".to_string(),
        },
        Item {
            category: "vegetable".to_string(),
            name: "carrot".to_string(),
        },
        Item {
            category: "fruit".to_string(),
            name: "banana".to_string(),
        },
    ];

    let result = umt_group_by(&items, |item| item.category.clone());

    assert_eq!(result.get(&"fruit".to_string()).unwrap().len(), 2);
    assert_eq!(result.get(&"vegetable".to_string()).unwrap().len(), 1);
}

#[test]
fn test_group_by_indexed() {
    let array = vec![10, 20, 30, 40, 50];
    let result = umt_group_by_indexed(
        &array,
        |_, idx| if idx % 2 == 0 { "even_idx" } else { "odd_idx" },
    );
    assert_eq!(result.get(&"even_idx"), Some(&vec![10, 30, 50]));
    assert_eq!(result.get(&"odd_idx"), Some(&vec![20, 40]));
}

#[test]
fn test_group_by_indexed_empty() {
    let array: Vec<i32> = vec![];
    let result = umt_group_by_indexed(&array, |_, idx| idx);
    assert!(result.is_empty());
}

#[test]
fn test_group_by_indexed_with_value() {
    let array = vec!["a", "bb", "ccc", "dd", "e"];
    let result = umt_group_by_indexed(&array, |s, idx| s.len() + idx);
    // idx=0, len=1 -> 1
    // idx=1, len=2 -> 3
    // idx=2, len=3 -> 5
    // idx=3, len=2 -> 5
    // idx=4, len=1 -> 5
    assert_eq!(result.get(&1), Some(&vec!["a"]));
    assert_eq!(result.get(&3), Some(&vec!["bb"]));
    assert_eq!(result.get(&5).unwrap().len(), 3);
}
