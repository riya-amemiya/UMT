use umtn_wasm::ncr;

#[test]
fn test_ncr() {
    assert_eq!(ncr(7, 3), 35);
}

#[test]
fn test_ncr_zero() {
    assert_eq!(ncr(0, 0), 1);
}

#[test]
fn test_ncr_n_equals_r() {
    assert_eq!(ncr(5, 5), 1);
}

#[test]
fn test_ncr_n_less_than_r() {
    assert_eq!(ncr(3, 5), 0);
}

#[test]
fn test_ncr_r_equals_zero() {
    assert_eq!(ncr(5, 0), 1);
}

#[test]
fn test_ncr_negative_n() {
    ncr(-5, 3);
}

#[test]
fn test_ncr_negative_r() {
    ncr(5, -3);
}
