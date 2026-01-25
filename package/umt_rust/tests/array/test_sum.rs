use umt_rust::array::{umt_sum, umt_sum_i64};

#[test]
fn test_sum_integers() {
    assert_eq!(umt_sum(&[1.0, 2.0, 3.0]), 6.0);
    assert_eq!(umt_sum(&[10.0, 20.0, 30.0]), 60.0);
    assert_eq!(umt_sum(&[-1.0, -2.0, -3.0]), -6.0);
    assert_eq!(umt_sum(&[]), 0.0);
}

#[test]
fn test_sum_decimals() {
    assert!((umt_sum(&[0.1, 0.2, 0.3]) - 0.6).abs() < 1e-10);
    assert!((umt_sum(&[1.1, 2.2, 3.3]) - 6.6).abs() < 1e-10);
    assert!((umt_sum(&[0.1, 0.02, 0.003]) - 0.123).abs() < 1e-10);
}

#[test]
fn test_sum_mixed() {
    assert!((umt_sum(&[1.0, 2.5, 3.7]) - 7.2).abs() < 1e-10);
    assert!((umt_sum(&[-1.5, 2.0, -3.7]) - (-3.2)).abs() < 1e-10);
}

#[test]
fn test_sum_large_numbers() {
    let large_num = 1_000_000_000.0;
    assert_eq!(umt_sum(&[large_num, large_num]), 2.0 * large_num);
    assert_eq!(umt_sum(&[large_num, -large_num]), 0.0);
    assert_eq!(umt_sum(&[large_num, 1.0, -1.0]), large_num);

    let medium_num = 1_000_000.0;
    let count = 1000;
    let arr: Vec<f64> = vec![medium_num; count];
    assert_eq!(umt_sum(&arr), medium_num * count as f64);
}

#[test]
fn test_sum_very_small_numbers() {
    let small_num = f64::MIN_POSITIVE;
    assert_eq!(umt_sum(&[small_num, small_num]), 2.0 * small_num);
    assert_eq!(umt_sum(&[small_num, -small_num]), 0.0);
}

#[test]
fn test_sum_precision() {
    assert!((umt_sum(&[0.1, 0.2]) - 0.3).abs() < 1e-10);
    assert!((umt_sum(&[0.1, 0.2, 0.3, 0.4, 0.5]) - 1.5).abs() < 1e-10);
}

#[test]
fn test_sum_many_decimals() {
    assert!((umt_sum(&[0.0001, 0.0002, 0.0003]) - 0.0006).abs() < 1e-10);
    assert!((umt_sum(&[1.23456, 2.34567, 3.45678]) - 7.03701).abs() < 1e-5);
}

#[test]
fn test_sum_i64() {
    assert_eq!(umt_sum_i64(&[1, 2, 3]), 6);
    assert_eq!(umt_sum_i64(&[10, 20, 30]), 60);
    assert_eq!(umt_sum_i64(&[-1, -2, -3]), -6);
    assert_eq!(umt_sum_i64(&[]), 0);
}
