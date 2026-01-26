use umt_rust::math::umt_nhr;

#[test]
fn test_nhr_basic() {
    assert_eq!(umt_nhr(5, 2), 15);
    assert_eq!(umt_nhr(3, 3), 10);
}

#[test]
fn test_nhr_choose_one() {
    assert_eq!(umt_nhr(3, 1), 3);
    assert_eq!(umt_nhr(5, 1), 5);
}

#[test]
fn test_nhr_same_as_total() {
    assert_eq!(umt_nhr(2, 2), 3);
    assert_eq!(umt_nhr(4, 4), 35);
}

#[test]
fn test_nhr_larger() {
    assert_eq!(umt_nhr(10, 3), 220);
    assert_eq!(umt_nhr(6, 4), 126);
}

#[test]
fn test_nhr_zero_n() {
    assert_eq!(umt_nhr(0, 5), -1);
}

#[test]
fn test_nhr_zero_r() {
    assert_eq!(umt_nhr(5, 0), -1);
}

#[test]
fn test_nhr_both_zero() {
    assert_eq!(umt_nhr(0, 0), -1);
}

#[test]
fn test_nhr_negative_n() {
    assert_eq!(umt_nhr(-1, 5), -1);
}

#[test]
fn test_nhr_negative_r() {
    assert_eq!(umt_nhr(5, -1), -1);
}

#[test]
fn test_nhr_both_negative() {
    assert_eq!(umt_nhr(-1, -1), -1);
}
