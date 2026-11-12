use umt_rust::array::ordered_unique::ordered_unique_by;

#[test]
fn test_keeps_first_occurrence_in_order() {
    assert_eq!(ordered_unique_by(&[3, 1, 3, 2, 1], |x| *x), vec![3, 1, 2]);
}

#[test]
fn test_empty_input() {
    let empty: [i32; 0] = [];
    assert_eq!(ordered_unique_by(&empty, |x| *x), Vec::<i32>::new());
}

#[test]
fn test_uses_key() {
    let words = ["apple", "avocado", "banana"];
    assert_eq!(
        ordered_unique_by(&words, |s| s.chars().next()),
        vec!["apple", "banana"]
    );
}

#[test]
fn test_chained_sources() {
    let first = [1, 2];
    let second = [2, 3];
    assert_eq!(
        ordered_unique_by(first.iter().chain(second.iter()), |x| x),
        vec![1, 2, 3]
    );
}

#[test]
fn test_keeps_first_value_unchanged() {
    let result = ordered_unique_by(&[-0.0_f64, 0.0], |x| if *x == 0.0 { 0 } else { 1 });
    assert_eq!(result.len(), 1);
    assert!(result[0].is_sign_negative());
}
