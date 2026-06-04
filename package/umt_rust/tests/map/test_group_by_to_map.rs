use umt_rust::map::umt_group_by_to_map;

#[test]
fn test_should_group_numbers_into_odd_and_even() {
    let array = [1, 2, 3, 4, 5];
    let result = umt_group_by_to_map(&array, |num, _, _| {
        if num % 2 == 0 {
            "even"
        } else {
            "odd"
        }
    });
    assert_eq!(
        result,
        vec![("odd", vec![1, 3, 5]), ("even", vec![2, 4])]
    );
}

#[test]
fn test_should_preserve_insertion_order_of_keys() {
    let array = [3, 1, 2, 1, 3, 2];
    let result = umt_group_by_to_map(&array, |num, _, _| *num);
    let keys: Vec<i32> = result.iter().map(|(key, _)| *key).collect();
    assert_eq!(keys, vec![3, 1, 2]);
}

#[test]
fn test_should_accept_object_keys() {
    #[derive(Clone, PartialEq, Debug)]
    struct Tag {
        id: String,
    }
    #[derive(Clone, PartialEq, Debug)]
    struct Item {
        tag: Tag,
        value: i32,
    }
    let a = Tag { id: "a".to_string() };
    let b = Tag { id: "b".to_string() };
    let array = vec![
        Item {
            tag: a.clone(),
            value: 1,
        },
        Item {
            tag: b.clone(),
            value: 2,
        },
        Item {
            tag: a.clone(),
            value: 3,
        },
    ];
    let result = umt_group_by_to_map(&array, |item, _, _| item.tag.clone());

    let a_bucket = result
        .iter()
        .find(|(key, _)| *key == a)
        .map(|(_, bucket)| bucket)
        .unwrap();
    assert_eq!(
        *a_bucket,
        vec![
            Item {
                tag: a.clone(),
                value: 1
            },
            Item {
                tag: a.clone(),
                value: 3
            },
        ]
    );

    let b_bucket = result
        .iter()
        .find(|(key, _)| *key == b)
        .map(|(_, bucket)| bucket)
        .unwrap();
    assert_eq!(
        *b_bucket,
        vec![Item {
            tag: b.clone(),
            value: 2
        }]
    );
}

#[test]
fn test_should_group_strings_based_on_their_length() {
    let array = vec!["one", "two", "three", "four", "five"];
    let result = umt_group_by_to_map(&array, |str, _, _| str.len());
    assert_eq!(
        result,
        vec![
            (3_usize, vec!["one", "two"]),
            (5_usize, vec!["three"]),
            (4_usize, vec!["four", "five"]),
        ]
    );
}

#[test]
fn test_should_pass_index_and_array_to_the_iteratee() {
    let array = vec!["a", "b", "c", "d"];
    let mut received_indices: Vec<usize> = Vec::new();
    umt_group_by_to_map(&array, |_value, index, source| {
        assert_eq!(source, array.as_slice());
        received_indices.push(index);
        index % 2
    });
    assert_eq!(received_indices, vec![0, 1, 2, 3]);
}

#[test]
fn test_should_return_an_empty_map_for_an_empty_array() {
    let array: Vec<i32> = vec![];
    let result = umt_group_by_to_map(&array, |num, _, _| {
        if num % 2 == 0 {
            "even"
        } else {
            "odd"
        }
    });
    assert_eq!(result.len(), 0);
}
