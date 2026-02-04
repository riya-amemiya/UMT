use umt_rust::math::calculator::umt_calculator;
use umt_rust::math::{umt_average, umt_standard_deviation, umt_deviation_value};
use umt_rust::array::umt_sum;

#[test]
fn test_calc_and_stats() {
    // calculate average from calculator results
    let values: Vec<f64> = vec![
        umt_calculator("10 + 5", None),
        umt_calculator("20 - 8", None),
        umt_calculator("3 * 6", None),
        umt_calculator("100 / 4", None),
    ].iter().map(|s| s.parse().unwrap()).collect();

    // 15, 12, 18, 25
    let avg = umt_average(values);
    assert_eq!(avg, 17.5);
}

#[test]
fn test_std_dev() {
    let scores = vec![90.0, 80.0, 90.0, 70.0, 85.0];
    let avg = umt_average(scores.clone());
    let std_dev = umt_standard_deviation(&scores);

    assert_eq!(avg, 83.0);
    assert!((std_dev - 7.4833).abs() < 0.001);
}

#[test]
fn test_deviation_value() {
    let raw_scores = vec![65.0, 72.0, 88.0, 91.0, 79.0];
    let avg = umt_average(raw_scores.clone());
    let std_dev = umt_standard_deviation(&raw_scores);

    let score = raw_scores[0];
    let dev_val = umt_deviation_value(score, avg, std_dev);

    // Approx 35.56
    assert!((dev_val - 35.56).abs() < 0.1);
}

#[test]
fn test_complex_expressions() {
    let results = vec![13.0, 12.0, 10.0, 27.0, 2.0];
    let total = umt_sum(&results);
    let avg = umt_average(results.clone());

    assert_eq!(total, 64.0);
    assert_eq!(avg, 12.8);
}
