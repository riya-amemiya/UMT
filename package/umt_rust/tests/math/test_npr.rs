use umt_rust::math::umt_npr;

#[test]
fn test_npr() {
    assert_eq!(umt_npr(7, 3), 210);
    assert_eq!(umt_npr(5, 5), 120);
    assert_eq!(umt_npr(3, 5), 0);
    assert_eq!(umt_npr(5, 0), 1);
}

#[test]
fn test_npr_r_less_than_zero() {
    assert_eq!(umt_npr(5, -1), 0);
}

#[test]
fn test_npr_valid_input() {
    assert_eq!(umt_npr(5, 2), 20);
}

#[test]
fn test_npr_n_greater_than_12() {
    assert_eq!(umt_npr(13, 5), 0);
}
