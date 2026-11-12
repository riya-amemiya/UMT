use umt_rust::math::umt_ncr;

#[test]
fn test_ncr_large_n() {
    assert_eq!(umt_ncr(20, 3), 1140);
    assert_eq!(umt_ncr(30, 15), 155117520);
    assert_eq!(umt_ncr(100, 97), 161700);
}

#[test]
fn test_ncr_symmetry() {
    assert_eq!(umt_ncr(25, 23), umt_ncr(25, 2));
}

#[test]
fn test_ncr_overflow_returns_zero() {
    assert_eq!(umt_ncr(60, 30), 0);
    assert_eq!(umt_ncr(i32::MAX, i32::MAX / 2), 0);
}

#[test]
fn test_ncr_invalid_arguments_return_zero() {
    assert_eq!(umt_ncr(2, 5), 0);
    assert_eq!(umt_ncr(-1, 2), 0);
    assert_eq!(umt_ncr(5, -1), 0);
}

#[test]
fn test_ncr_trivial_cases() {
    assert_eq!(umt_ncr(5, 0), 1);
    assert_eq!(umt_ncr(5, 5), 1);
    assert_eq!(umt_ncr(0, 0), 1);
    assert_eq!(umt_ncr(5, 2), 10);
}
