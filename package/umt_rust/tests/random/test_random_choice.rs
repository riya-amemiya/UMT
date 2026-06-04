use umt_rust::random::umt_random_choice;

#[test]
fn test_returns_an_element_from_the_array() {
    let items = ["a", "b", "c"];
    for _ in 0..50 {
        let picked = umt_random_choice(&items).unwrap();
        assert!(items.contains(&picked));
    }
}

#[test]
fn test_returns_the_only_element_for_single_item_arrays() {
    assert_eq!(umt_random_choice(&[42]), Some(42));
}

#[test]
fn test_returns_none_for_empty_slice() {
    let empty: [i32; 0] = [];
    assert_eq!(umt_random_choice(&empty), None);
}
