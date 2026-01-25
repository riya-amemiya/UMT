//! Deviation Value Simple integration tests.
//!
//! Ported from TypeScript: deviationValueSimple.test.ts

use umt_rust::simple::math::{umt_deviation_value_simple, umt_deviation_value_simple_from_array};

// ============================================================================
// With explicit values tests
// ============================================================================

#[test]
fn test_calculates_deviation_value_when_value_equals_average() {
    // When value equals average (no deviation)
    assert_eq!(umt_deviation_value_simple(50.0, 50.0, 10.0), 50.0);
}

#[test]
fn test_calculates_one_standard_deviation_above_average() {
    // One standard deviation above average
    assert_eq!(umt_deviation_value_simple(60.0, 50.0, 10.0), 60.0);
}

#[test]
fn test_calculates_one_standard_deviation_below_average() {
    // One standard deviation below average
    assert_eq!(umt_deviation_value_simple(40.0, 50.0, 10.0), 40.0);
}

#[test]
fn test_returns_50_when_standard_deviation_is_zero() {
    // When standard deviation is 0
    assert_eq!(umt_deviation_value_simple(100.0, 50.0, 0.0), 50.0);
}

#[test]
fn test_handles_two_standard_deviations() {
    // Two standard deviations above average
    assert_eq!(umt_deviation_value_simple(70.0, 50.0, 10.0), 70.0);

    // Two standard deviations below average
    assert_eq!(umt_deviation_value_simple(30.0, 50.0, 10.0), 30.0);
}

// ============================================================================
// With array input tests
// ============================================================================

#[test]
fn test_calculates_deviation_value_from_array_above_mean() {
    // Using simple array [40, 50, 60]
    // mean = 50
    // population standard deviation ~= 8.165
    let scores = vec![40.0, 50.0, 60.0];
    let result = umt_deviation_value_simple_from_array(60.0, scores);
    // +1.225 SD -> approximately 62.25
    assert!(
        (result - 62.25).abs() < 0.01,
        "Expected ~62.25, got {}",
        result
    );
}

#[test]
fn test_calculates_deviation_value_from_array_at_mean() {
    let scores = vec![40.0, 50.0, 60.0];
    let result = umt_deviation_value_simple_from_array(50.0, scores);
    assert_eq!(result, 50.0);
}

#[test]
fn test_calculates_deviation_value_from_array_below_mean() {
    let scores = vec![40.0, 50.0, 60.0];
    let result = umt_deviation_value_simple_from_array(40.0, scores);
    // -1.225 SD -> approximately 37.75
    assert!(
        (result - 37.75).abs() < 0.01,
        "Expected ~37.75, got {}",
        result
    );
}

// ============================================================================
// All same values tests
// ============================================================================

#[test]
fn test_returns_50_for_any_value_when_all_reference_values_same_at_mean() {
    let same_scores = vec![50.0, 50.0, 50.0];
    assert_eq!(umt_deviation_value_simple_from_array(50.0, same_scores), 50.0);
}

#[test]
fn test_returns_50_for_zero_when_all_reference_values_same() {
    let same_scores = vec![50.0, 50.0, 50.0];
    assert_eq!(umt_deviation_value_simple_from_array(0.0, same_scores), 50.0);
}

#[test]
fn test_returns_50_for_high_value_when_all_reference_values_same() {
    let same_scores = vec![50.0, 50.0, 50.0];
    assert_eq!(umt_deviation_value_simple_from_array(100.0, same_scores), 50.0);
}

// ============================================================================
// Additional edge cases
// ============================================================================

#[test]
fn test_handles_negative_values() {
    let scores = vec![-10.0, 0.0, 10.0];
    let result = umt_deviation_value_simple_from_array(0.0, scores);
    assert_eq!(result, 50.0);
}

#[test]
fn test_handles_large_values() {
    let scores = vec![1000.0, 2000.0, 3000.0];
    let result = umt_deviation_value_simple_from_array(2000.0, scores);
    assert_eq!(result, 50.0);
}

#[test]
fn test_handles_decimal_values() {
    let result = umt_deviation_value_simple(55.5, 50.0, 10.0);
    assert!(
        (result - 55.5).abs() < 0.01,
        "Expected ~55.5, got {}",
        result
    );
}

#[test]
fn test_calculates_with_different_sd() {
    // Standard deviation of 5 instead of 10
    // One SD above should be 60, not 55
    let result = umt_deviation_value_simple(55.0, 50.0, 5.0);
    // (55 - 50) / 5 = 1 SD -> 50 + 10 = 60
    assert_eq!(result, 60.0);
}

#[test]
fn test_calculates_with_larger_array() {
    let scores = vec![
        65.0, 70.0, 75.0, 80.0, 85.0, 90.0, 95.0, 100.0, 55.0, 60.0,
    ];
    let result = umt_deviation_value_simple_from_array(77.5, scores);
    // 77.5 is the mean, so deviation value should be 50
    assert_eq!(result, 50.0);
}
