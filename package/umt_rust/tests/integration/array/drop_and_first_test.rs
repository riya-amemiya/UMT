use umt_rust::array::{DropDirection, umt_drop, umt_first};

#[test]
fn test_drop_and_first() {
    // should drop n elements from the array and return the first element
    let arr = vec![1, 2, 3, 4, 5];

    let dropped1 = umt_drop(&arr, 1, DropDirection::Left);
    assert_eq!(umt_first(&dropped1), Some(&2));

    let dropped2 = umt_drop(&arr, 2, DropDirection::Left);
    assert_eq!(umt_first(&dropped2), Some(&3));

    let dropped3 = umt_drop(&arr, 3, DropDirection::Left);
    assert_eq!(umt_first(&dropped3), Some(&4));

    let dropped4 = umt_drop(&arr, 4, DropDirection::Left);
    assert_eq!(umt_first(&dropped4), Some(&5));

    let dropped_right = umt_drop(&arr, 2, DropDirection::Right);
    assert_eq!(umt_first(&dropped_right), Some(&1));

    let dropped_left_3 = umt_drop(&arr, 3, DropDirection::Left);
    assert_eq!(umt_first(&dropped_left_3), Some(&4));
}

#[test]
fn test_edge_cases() {
    let empty: Vec<i32> = vec![];
    assert_eq!(umt_first(&umt_drop(&empty, 1, DropDirection::Left)), None);

    let single = vec![1];
    assert_eq!(umt_first(&umt_drop(&single, 1, DropDirection::Left)), None);

    let two = vec![1, 2];
    assert_eq!(umt_first(&umt_drop(&two, 5, DropDirection::Left)), None);

    let three = vec![1, 2, 3];
    assert_eq!(
        umt_first(&umt_drop(&three, 0, DropDirection::Left)),
        Some(&1)
    );
}

#[test]
fn test_mixed_types() {
    let str_arr = vec!["a", "b", "c"];
    assert_eq!(
        umt_first(&umt_drop(&str_arr, 1, DropDirection::Left)),
        Some(&"b")
    );

    let bool_arr = vec![true, false];
    assert_eq!(
        umt_first(&umt_drop(&bool_arr, 1, DropDirection::Left)),
        Some(&false)
    );

    // Rust is strongly typed, so we can't easily mix null, undefined, and numbers in a Vec
    // unless we use the Value enum or similar wrapper.
    // Simulating with Value enum if applicable, or skipping dynamic type test.
}
