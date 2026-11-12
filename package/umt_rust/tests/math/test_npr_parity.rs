use umt_rust::math::umt_npr;

#[test]
fn test_npr_large_n_with_small_r() {
    assert_eq!(umt_npr(13, 5), 154440);
    assert_eq!(umt_npr(20, 2), 380);
    assert_eq!(umt_npr(100, 1), 100);
}

#[test]
fn test_npr_overflow_returns_zero() {
    assert_eq!(umt_npr(13, 13), 0);
    assert_eq!(umt_npr(100, 10), 0);
}

#[test]
fn test_npr_invalid_arguments_return_zero() {
    assert_eq!(umt_npr(2, 5), 0);
    assert_eq!(umt_npr(5, -1), 0);
    assert_eq!(umt_npr(-1, 0), 0);
}

#[test]
fn test_npr_zero_items() {
    assert_eq!(umt_npr(0, 0), 1);
    assert_eq!(umt_npr(5, 0), 1);
}

#[test]
fn test_npr_largest_factorial_that_fits() {
    assert_eq!(umt_npr(12, 12), 479001600);
}
