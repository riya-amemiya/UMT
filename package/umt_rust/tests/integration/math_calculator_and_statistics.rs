//! Integration tests for Math utility functions
//!
//! Tests the interaction between calculator and statistical functions:
//! - Using calculator results in statistical calculations
//! - Complex mathematical workflows
//! - Real-world calculation scenarios

use umt_rust::array::umt_sum;
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
        // Simulating calculator operations: 10+5=15, 20-8=12, 3*6=18, 100/4=25
        let values = vec![15.0, 12.0, 18.0, 25.0];

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
        // Simulating grades with weights
        let grades = [
            (85.0, 0.3),
            (90.0, 0.4),
            (78.0, 0.3),
        ];

        let weighted_sum: f64 = grades
            .iter()
            .map(|(score, weight)| score * weight)
            .sum();

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

        let non_zero_rates: Vec<f64> = growth_rates.iter().filter(|&&r| r != 0.0).cloned().collect();
        let avg_growth_rate = umt_average(non_zero_rates);
        assert!(
            close_to(avg_growth_rate, 32.083, 2),
            "Average growth rate mismatch: {} vs expected ~32.083",
            avg_growth_rate
        );
    }

    #[test]
    fn should_calculate_complex_expressions_and_analyze_results() {
        let results = [13.0, 12.0, 10.0, 27.0, 2.0];
        let total = umt_sum(&results);
        let avg = umt_average(results.to_vec());

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
        let z_scores: Vec<f64> = data
            .iter()
            .map(|&val| (val - avg) / std_dev)
            .collect();

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
}
