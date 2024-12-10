use umt_rust::math::umt_ncr;



#[test]
fn test_ncr() {
    assert_eq!(umt_ncr(7, 3), 35);
}

#[test]
fn test_ncr_zero() {
    assert_eq!(umt_ncr(0, 0), 1);
}

#[test]
fn test_ncr_n_equals_r() {
    assert_eq!(umt_ncr(5, 5), 1);
}

#[test]
fn test_ncr_n_less_than_r() {
    assert_eq!(umt_ncr(3, 5), 0);
}

#[test]
fn test_ncr_r_equals_zero() {
    assert_eq!(umt_ncr(5, 0), 1);
}

#[test]
fn test_ncr_negative_n() {
    umt_ncr(-5, 3);
}

#[test]
fn test_ncr_negative_r() {
    umt_ncr(5, -3);
}
