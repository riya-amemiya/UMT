use umt_rust::number::{umt_to_percentage, umt_to_percentage_default};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_basic_percentage() {
        assert_eq!(umt_to_percentage_default(25.0, 100.0), 25.0);
        assert_eq!(umt_to_percentage_default(50.0, 100.0), 50.0);
        assert_eq!(umt_to_percentage_default(100.0, 100.0), 100.0);
    }

    #[test]
    fn should_return_result_with_2_decimal_places_by_default() {
        assert_eq!(umt_to_percentage_default(1.0, 3.0), 33.33);
        assert_eq!(umt_to_percentage_default(2.0, 3.0), 66.67);
    }

    #[test]
    fn should_respect_custom_decimal_places() {
        assert_eq!(umt_to_percentage(1.0, 3.0, 0), 33.0);
        assert_eq!(umt_to_percentage(1.0, 3.0, 1), 33.3);
        assert_eq!(umt_to_percentage(1.0, 3.0, 4), 33.3333);
    }

    #[test]
    fn should_return_0_when_total_is_0() {
        assert_eq!(umt_to_percentage_default(0.0, 0.0), 0.0);
        assert_eq!(umt_to_percentage_default(5.0, 0.0), 0.0);
        assert_eq!(umt_to_percentage_default(-5.0, 0.0), 0.0);
    }

    #[test]
    fn should_handle_0_value() {
        assert_eq!(umt_to_percentage_default(0.0, 100.0), 0.0);
    }

    #[test]
    fn should_handle_values_greater_than_total() {
        assert_eq!(umt_to_percentage_default(150.0, 100.0), 150.0);
        assert_eq!(umt_to_percentage_default(200.0, 100.0), 200.0);
    }

    #[test]
    fn should_handle_negative_values() {
        assert_eq!(umt_to_percentage_default(-25.0, 100.0), -25.0);
    }
}
