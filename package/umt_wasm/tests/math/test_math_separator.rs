use umtn_wasm::math_separator;

#[test]
fn test_math_separator() {
    assert_eq!(math_separator(123), vec![100, 23]);
    assert_eq!(math_separator(1234), vec![1000, 234]);
    assert_eq!(math_separator(12345), vec![10000, 2345]);
}

#[test]
fn test_math_separator_zero() {
    assert_eq!(math_separator(0), vec![0, 0]);
}

#[test]
fn test_math_separator_single_digit() {
    for i in 1..10 {
        assert_eq!(math_separator(i), vec![0, i]);
    }
}
