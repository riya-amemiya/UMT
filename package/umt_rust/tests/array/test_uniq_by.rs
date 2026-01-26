use umt_rust::array::umt_uniq_by;

#[test]
fn test_uniq_by_floor() {
    let arr = vec![1.1, 1.2, 2.1, 2.2, 3.1];
    let result = umt_uniq_by(&arr, |x| *x as i32);
    assert_eq!(result, vec![1.1, 2.1, 3.1]);
}

#[test]
fn test_uniq_by_empty() {
    let arr: Vec<i32> = vec![];
    let result = umt_uniq_by(&arr, |x| *x);
    assert_eq!(result, Vec::<i32>::new());
}

#[test]
fn test_uniq_by_first_char() {
    let arr = vec!["apple", "banana", "apricot", "blueberry"];
    let result = umt_uniq_by(&arr, |s| s.chars().next().unwrap_or('\0'));
    assert_eq!(result, vec!["apple", "banana"]);
}

#[test]
fn test_uniq_by_modulo() {
    let arr = vec![1, 2, 3, 11, 12, 13];
    let result = umt_uniq_by(&arr, |x| x % 10);
    assert_eq!(result, vec![1, 2, 3]);
}

#[test]
fn test_uniq_by_preserves_order() {
    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        item_type: String,
        name: String,
    }

    let items = vec![
        Item {
            item_type: "fruit".to_string(),
            name: "apple".to_string(),
        },
        Item {
            item_type: "vegetable".to_string(),
            name: "carrot".to_string(),
        },
        Item {
            item_type: "fruit".to_string(),
            name: "orange".to_string(),
        },
        Item {
            item_type: "meat".to_string(),
            name: "chicken".to_string(),
        },
    ];

    let result = umt_uniq_by(&items, |item| item.item_type.clone());

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].name, "apple");
    assert_eq!(result[1].name, "carrot");
    assert_eq!(result[2].name, "chicken");
}

#[test]
fn test_uniq_by_complex_selector() {
    #[derive(Clone, Debug, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    let points = vec![
        Point { x: 1, y: 2 },
        Point { x: 2, y: 1 },
        Point { x: 1, y: 3 },
        Point { x: 3, y: 1 },
    ];

    let result = umt_uniq_by(&points, |p| p.x + p.y);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0], Point { x: 1, y: 2 }); // sum = 3
    assert_eq!(result[1], Point { x: 1, y: 3 }); // sum = 4
}

#[test]
fn test_uniq_by_all_same_key() {
    let arr = vec![1, 2, 3, 4];
    let result = umt_uniq_by(&arr, |_| "same");
    assert_eq!(result, vec![1]);
}

#[test]
fn test_uniq_by_boolean_selector() {
    #[derive(Clone, Debug, PartialEq)]
    struct Item {
        value: i32,
        flag: bool,
    }

    let items = vec![
        Item {
            value: 1,
            flag: true,
        },
        Item {
            value: 2,
            flag: false,
        },
        Item {
            value: 3,
            flag: true,
        },
        Item {
            value: 4,
            flag: false,
        },
    ];

    let result = umt_uniq_by(&items, |item| item.flag);
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].value, 1);
    assert_eq!(result[1].value, 2);
}

#[test]
fn test_uniq_by_struct_id() {
    #[derive(Clone, Debug, PartialEq)]
    struct Person {
        id: i32,
        name: String,
    }

    let people = vec![
        Person {
            id: 1,
            name: "Alice".to_string(),
        },
        Person {
            id: 2,
            name: "Bob".to_string(),
        },
        Person {
            id: 1,
            name: "Alice Duplicate".to_string(),
        },
        Person {
            id: 3,
            name: "Charlie".to_string(),
        },
    ];

    let result = umt_uniq_by(&people, |p| p.id);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].name, "Alice");
    assert_eq!(result[1].name, "Bob");
    assert_eq!(result[2].name, "Charlie");
}
