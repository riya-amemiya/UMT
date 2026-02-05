//! Integration tests for Math utility functions
//!
//! Tests the interaction between calculator and statistical functions:
//! - Using calculator results in statistical calculations
//! - Complex mathematical workflows
//! - Real-world calculation scenarios

use umt_rust::array::umt_sum;
use umt_rust::math::calculator::umt_calculator;
use umt_rust::math::{umt_average, umt_deviation_value, umt_standard_deviation};

#[cfg(test)]
mod tests {
    use super::*;

    fn close_to(actual: f64, expected: f64, precision: i32) -> bool {
        let factor = 10_f64.powi(precision);
        (actual * factor).round() == (expected * factor).round()
    }

    #[test]
    fn should_calculate_average_from_calculator_results() {
        // Using actual calculator function
        let values: Vec<f64> = vec![
            umt_calculator("10 + 5", None).parse().unwrap_or(0.0),
            umt_calculator("20 - 8", None).parse().unwrap_or(0.0),
            umt_calculator("3 * 6", None).parse().unwrap_or(0.0),
            umt_calculator("100 / 4", None).parse().unwrap_or(0.0),
        ];

        let avg = umt_average(values);
        assert_eq!(avg, 17.5);
    }

    #[test]
    fn should_compute_standard_deviation_from_complex_calculations() {
        let scores = [90.0, 80.0, 90.0, 70.0, 85.0];

        let avg = umt_average(scores.to_vec());
        let std_dev = umt_standard_deviation(&scores);

        assert_eq!(avg, 83.0);
        assert!(
            close_to(std_dev, 7.483_314_773_547_883, 5),
            "Standard deviation mismatch: {} vs expected 7.483314773547883",
            std_dev
        );
    }

    #[test]
    fn should_calculate_deviation_values_for_test_scores() {
        let raw_scores = [65.0, 72.0, 88.0, 91.0, 79.0];

        let avg = umt_average(raw_scores.to_vec());
        let std_dev = umt_standard_deviation(&raw_scores);

        let deviation_scores: Vec<f64> = raw_scores
            .iter()
            .map(|score| umt_deviation_value(*score, avg, std_dev))
            .collect();

        assert!(
            close_to(deviation_scores[0], 35.56, 0),
            "First deviation score mismatch: {} vs expected ~35.56",
            deviation_scores[0]
        );
        assert!(
            close_to(deviation_scores[2], 59.28, 0),
            "Third deviation score mismatch: {} vs expected ~59.28",
            deviation_scores[2]
        );
        assert!(
            close_to(deviation_scores[4], 50.0, 0),
            "Fifth deviation score mismatch: {} vs expected ~50",
            deviation_scores[4]
        );
    }

    #[test]
    fn should_handle_currency_conversion_in_statistical_calculations() {
        let prices_in_yen = [1500.0, 3750.0, 2325.0, 4500.0];

        let avg_price = umt_average(prices_in_yen.to_vec());
        assert_eq!(avg_price, 3018.75);
    }

    #[test]
    fn should_calculate_weighted_averages_using_calculator() {
        // Using actual calculator function for grades
        let grades = [
            (
                umt_calculator("85", None).parse::<f64>().unwrap_or(0.0),
                0.3,
            ),
            (
                umt_calculator("90", None).parse::<f64>().unwrap_or(0.0),
                0.4,
            ),
            (
                umt_calculator("78", None).parse::<f64>().unwrap_or(0.0),
                0.3,
            ),
        ];

        let weighted_sum: f64 = grades.iter().map(|(score, weight)| score * weight).sum();

        assert_eq!(weighted_sum, 84.9);
    }

    #[test]
    fn should_perform_multi_step_calculations_with_statistical_analysis() {
        let initial_values = [100.0, 150.0, 200.0, 250.0, 300.0];

        let growth_rates: Vec<f64> = initial_values
            .iter()
            .enumerate()
            .map(|(i, val)| {
                if i == 0 {
                    0.0
                } else {
                    (val - initial_values[i - 1]) / initial_values[i - 1] * 100.0
                }
            })
            .collect();

        let non_zero_rates: Vec<f64> = growth_rates
            .iter()
            .filter(|&&r| r != 0.0)
            .cloned()
            .collect();
        let avg_growth_rate = umt_average(non_zero_rates);
        assert!(
            close_to(avg_growth_rate, 32.083, 2),
            "Average growth rate mismatch: {} vs expected ~32.083",
            avg_growth_rate
        );
    }

    #[test]
    fn should_calculate_complex_expressions_and_analyze_results() {
        // Using actual calculator function for complex expressions
        let results: Vec<f64> = vec![
            umt_calculator("10 + 3", None).parse().unwrap_or(0.0), // 13
            umt_calculator("15 - 3", None).parse().unwrap_or(0.0), // 12
            umt_calculator("2 * 5", None).parse().unwrap_or(0.0),  // 10
            umt_calculator("9 * 3", None).parse().unwrap_or(0.0),  // 27
            umt_calculator("10 / 5", None).parse().unwrap_or(0.0), // 2
        ];

        let total = umt_sum(&results);
        let avg = umt_average(results.clone());

        assert_eq!(results, [13.0, 12.0, 10.0, 27.0, 2.0]);
        assert_eq!(total, 64.0);
        assert_eq!(avg, 12.8);
    }

    #[test]
    fn should_handle_statistical_edge_cases() {
        // Single value
        let single = [42.0];
        assert_eq!(umt_average(single.to_vec()), 42.0);
        assert_eq!(umt_standard_deviation(&single), 0.0);

        // Same values
        let same = [5.0, 5.0, 5.0, 5.0];
        assert_eq!(umt_average(same.to_vec()), 5.0);
        assert_eq!(umt_standard_deviation(&same), 0.0);
    }

    #[test]
    fn should_integrate_multiple_statistical_functions() {
        let data = [10.0, 20.0, 30.0, 40.0, 50.0];

        let avg = umt_average(data.to_vec());
        let std_dev = umt_standard_deviation(&data);
        let sum = umt_sum(&data);

        // Calculate z-scores
        let z_scores: Vec<f64> = data.iter().map(|&val| (val - avg) / std_dev).collect();

        assert_eq!(avg, 30.0);
        assert_eq!(sum, 150.0);
        assert!(
            (z_scores[0] - -1.4142135623730951).abs() < 0.0001,
            "Z-score for 10 mismatch"
        );
        assert!(
            (z_scores[4] - 1.4142135623730951).abs() < 0.0001,
            "Z-score for 50 mismatch"
        );
    }

    #[test]
    fn should_use_calculator_with_parentheses_in_analysis() {
        // Using calculator for expressions with parentheses
        let results: Vec<f64> = vec![
            umt_calculator("(2 + 3) * 4", None).parse().unwrap_or(0.0), // 20
            umt_calculator("(10 - 2) / 2", None).parse().unwrap_or(0.0), // 4
            umt_calculator("2 ^ 3", None).parse().unwrap_or(0.0),       // 8
        ];

        let avg = umt_average(results.clone());
        assert_eq!(results, [20.0, 4.0, 8.0]);
        assert!(close_to(avg, 10.666666666666666, 5));
    }

    #[test]
    fn should_handle_calculator_with_negative_numbers() {
        let results: Vec<f64> = vec![
            umt_calculator("-5 + 10", None).parse().unwrap_or(0.0), // 5
            umt_calculator("10 - 15", None).parse().unwrap_or(0.0), // -5
            umt_calculator("-3 * -2", None).parse().unwrap_or(0.0), // 6
        ];

        let sum = umt_sum(&results);
        assert_eq!(sum, 6.0);
    }
}
