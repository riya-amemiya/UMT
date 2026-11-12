use umt_rust::math::{umt_percentile, umt_quantile};

#[test]
fn test_quantile_quartiles() {
    let values = [1.0, 2.0, 3.0, 4.0, 5.0];
    assert_eq!(umt_quantile(&values, 0.25).unwrap(), 2.0);
    assert_eq!(umt_quantile(&values, 0.5).unwrap(), 3.0);
    assert_eq!(umt_quantile(&values, 0.75).unwrap(), 4.0);
}

#[test]
fn test_quantile_minimum_and_maximum() {
    let values = [5.0, 1.0, 4.0, 2.0, 3.0];
    assert_eq!(umt_quantile(&values, 0.0).unwrap(), 1.0);
    assert_eq!(umt_quantile(&values, 1.0).unwrap(), 5.0);
}

#[test]
fn test_quantile_interpolation() {
    assert_eq!(umt_quantile(&[1.0, 2.0, 3.0, 4.0], 0.5).unwrap(), 2.5);
    assert_eq!(umt_quantile(&[10.0, 20.0], 0.1).unwrap(), 11.0);
}

#[test]
fn test_quantile_matches_percentile() {
    let values: Vec<f64> = (0..101).map(|index| index as f64 * 1.5).collect();
    assert_eq!(
        umt_quantile(&values, 0.29).unwrap(),
        umt_percentile(&values, 29.0).unwrap()
    );
    assert_eq!(
        umt_quantile(&values, 0.07).unwrap(),
        umt_percentile(&values, 7.0).unwrap()
    );
}

#[test]
fn test_quantile_empty() {
    assert!(umt_quantile(&[], 0.5).unwrap().is_nan());
}

#[test]
fn test_quantile_out_of_range() {
    assert!(umt_quantile(&[1.0, 2.0, 3.0], 1.5).is_err());
}
