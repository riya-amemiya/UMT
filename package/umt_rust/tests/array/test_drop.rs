use umt_rust::array::{DropDirection, umt_drop, umt_drop_left, umt_drop_right};

#[test]
fn test_drop_left() {
    assert_eq!(
        umt_drop(&[1, 2, 3, 4, 5], 2, DropDirection::Left),
        vec![3, 4, 5]
    );
    assert_eq!(
        umt_drop(&[1, 2, 3, 4, 5], 1, DropDirection::Left),
        vec![2, 3, 4, 5]
    );
}

#[test]
fn test_drop_right() {
    assert_eq!(
        umt_drop(&[1, 2, 3, 4, 5], 2, DropDirection::Right),
        vec![1, 2, 3]
    );
}

#[test]
fn test_drop_between() {
    assert_eq!(
        umt_drop(&[1, 2, 3, 4, 5], 1, DropDirection::Between),
        vec![1, 2, 4, 5]
    );
    assert_eq!(
        umt_drop(&[1, 2, 3, 4, 5, 6], 2, DropDirection::Between),
        vec![1, 2, 5, 6]
    );
}

#[test]
fn test_drop_exceeds_length() {
    assert_eq!(
        umt_drop(&[1, 2, 3], 4, DropDirection::Left),
        Vec::<i32>::new()
    );
    assert_eq!(
        umt_drop(&[1, 2, 3], 3, DropDirection::Left),
        Vec::<i32>::new()
    );
}

#[test]
fn test_drop_zero() {
    assert_eq!(umt_drop(&[1, 2, 3], 0, DropDirection::Left), vec![1, 2, 3]);
    assert_eq!(umt_drop(&[1, 2, 3], 0, DropDirection::Right), vec![1, 2, 3]);
    assert_eq!(
        umt_drop(&[1, 2, 3], 0, DropDirection::Between),
        vec![1, 2, 3]
    );
}

#[test]
fn test_drop_empty_array() {
    assert_eq!(
        umt_drop::<i32>(&[], 1, DropDirection::Left),
        Vec::<i32>::new()
    );
    assert_eq!(
        umt_drop::<i32>(&[], 2, DropDirection::Right),
        Vec::<i32>::new()
    );
    assert_eq!(
        umt_drop::<i32>(&[], 3, DropDirection::Between),
        Vec::<i32>::new()
    );
}

#[test]
fn test_drop_left_convenience() {
    assert_eq!(umt_drop_left(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5]);
}

#[test]
fn test_drop_right_convenience() {
    assert_eq!(umt_drop_right(&[1, 2, 3, 4, 5], 2), vec![1, 2, 3]);
}

#[test]
fn test_drop_with_strings() {
    assert_eq!(
        umt_drop(&["a", "b", "c", "d", "e"], 2, DropDirection::Left),
        vec!["c", "d", "e"]
    );
    assert_eq!(
        umt_drop(&["a", "b", "c", "d", "e"], 2, DropDirection::Right),
        vec!["a", "b", "c"]
    );
}
