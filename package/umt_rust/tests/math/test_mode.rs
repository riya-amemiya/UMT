use umt_rust::math::umt_mode;

#[test]
fn test_mode_single_mode() {
    assert_eq!(umt_mode(&[1.0, 2.0, 2.0, 3.0, 3.0, 3.0]), vec![3.0]);
}

#[test]
fn test_mode_multiple_modes() {
    assert_eq!(umt_mode(&[1.0, 2.0, 2.0, 3.0, 3.0]), vec![2.0, 3.0]);
}

#[test]
fn test_mode_all_unique() {
    assert_eq!(umt_mode(&[1.0, 2.0, 3.0]), vec![1.0, 2.0, 3.0]);
}

#[test]
fn test_mode_empty() {
    let result: Vec<f64> = vec![];
    assert_eq!(umt_mode(&[]), result);
}

#[test]
fn test_mode_single_element() {
    assert_eq!(umt_mode(&[42.0]), vec![42.0]);
}

#[test]
fn test_mode_negative() {
    assert_eq!(umt_mode(&[-1.0, -2.0, -2.0, -3.0]), vec![-2.0]);
}

#[test]
fn test_mode_decimals() {
    assert_eq!(umt_mode(&[1.5, 2.5, 2.5, 3.5]), vec![2.5]);
}

#[test]
fn test_mode_sorted_ascending() {
    assert_eq!(umt_mode(&[3.0, 1.0, 3.0, 1.0, 2.0, 2.0]), vec![1.0, 2.0, 3.0]);
}
