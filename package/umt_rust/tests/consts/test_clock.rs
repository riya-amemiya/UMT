//! Tests for the clock module.

use umt_rust::consts::{
    ONE_DAY_MS, ONE_HOUR_MS, ONE_MINUTE_MS, ONE_MONTH_MS, ONE_MONTH_MS_28, ONE_MONTH_MS_29,
    ONE_MONTH_MS_31, ONE_SECOND_MS, ONE_WEEK_MS, ONE_YEAR_MS, ONE_YEAR_MS_366,
};

mod basic_time_unit_values {
    use super::*;

    #[test]
    fn test_one_second_is_1000_milliseconds() {
        assert_eq!(ONE_SECOND_MS, 1000);
    }

    #[test]
    fn test_one_minute_is_60000_milliseconds() {
        assert_eq!(ONE_MINUTE_MS, 60_000);
    }

    #[test]
    fn test_one_hour_is_3600000_milliseconds() {
        assert_eq!(ONE_HOUR_MS, 3_600_000);
    }

    #[test]
    fn test_one_day_is_86400000_milliseconds() {
        assert_eq!(ONE_DAY_MS, 86_400_000);
    }

    #[test]
    fn test_one_week_is_604800000_milliseconds() {
        assert_eq!(ONE_WEEK_MS, 604_800_000);
    }
}

mod time_unit_relationships {
    use super::*;

    #[test]
    fn test_correct_relationships_between_units() {
        assert_eq!(ONE_MINUTE_MS, ONE_SECOND_MS * 60);
        assert_eq!(ONE_HOUR_MS, ONE_MINUTE_MS * 60);
        assert_eq!(ONE_DAY_MS, ONE_HOUR_MS * 24);
        assert_eq!(ONE_WEEK_MS, ONE_DAY_MS * 7);
    }
}

mod month_variations {
    use super::*;

    #[test]
    fn test_correct_relationships_for_different_month_lengths() {
        assert_eq!(ONE_MONTH_MS_28, ONE_DAY_MS * 28);
        assert_eq!(ONE_MONTH_MS_29, ONE_DAY_MS * 29);
        assert_eq!(ONE_MONTH_MS, ONE_DAY_MS * 30);
        assert_eq!(ONE_MONTH_MS_31, ONE_DAY_MS * 31);
    }

    #[test]
    fn test_correct_order_of_month_lengths() {
        assert!(ONE_MONTH_MS_28 < ONE_MONTH_MS_29);
        assert!(ONE_MONTH_MS_29 < ONE_MONTH_MS);
        assert!(ONE_MONTH_MS < ONE_MONTH_MS_31);
    }
}

mod year_variations {
    use super::*;

    #[test]
    fn test_correct_relationships_for_different_year_lengths() {
        assert_eq!(ONE_YEAR_MS, ONE_DAY_MS * 365);
        assert_eq!(ONE_YEAR_MS_366, ONE_DAY_MS * 366);
    }

    #[test]
    fn test_correct_relationship_between_regular_and_leap_years() {
        assert!(ONE_YEAR_MS < ONE_YEAR_MS_366);
        assert_eq!(ONE_YEAR_MS_366 - ONE_YEAR_MS, ONE_DAY_MS);
    }
}

mod exact_values {
    use super::*;

    #[test]
    fn test_one_month_28_days_is_2419200000_milliseconds() {
        assert_eq!(ONE_MONTH_MS_28, 2_419_200_000);
    }

    #[test]
    fn test_one_month_29_days_is_2505600000_milliseconds() {
        assert_eq!(ONE_MONTH_MS_29, 2_505_600_000);
    }

    #[test]
    fn test_one_month_30_days_is_2592000000_milliseconds() {
        assert_eq!(ONE_MONTH_MS, 2_592_000_000);
    }

    #[test]
    fn test_one_month_31_days_is_2678400000_milliseconds() {
        assert_eq!(ONE_MONTH_MS_31, 2_678_400_000);
    }

    #[test]
    fn test_one_year_365_days_is_31536000000_milliseconds() {
        assert_eq!(ONE_YEAR_MS, 31_536_000_000);
    }

    #[test]
    fn test_one_year_366_days_is_31622400000_milliseconds() {
        assert_eq!(ONE_YEAR_MS_366, 31_622_400_000);
    }
}

use umt_rust::consts::*;

#[test]
fn test_basic_time_unit_values() {
    assert_eq!(ONE_SECOND_MS, 1000);
    assert_eq!(ONE_MINUTE_MS, 60_000);
    assert_eq!(ONE_HOUR_MS, 3_600_000);
    assert_eq!(ONE_DAY_MS, 86_400_000);
    assert_eq!(ONE_WEEK_MS, 604_800_000);
}

#[test]
fn test_exact_month_values() {
    assert_eq!(ONE_MONTH_MS_28, 2_419_200_000);
    assert_eq!(ONE_MONTH_MS_29, 2_505_600_000);
    assert_eq!(ONE_MONTH_MS, 2_592_000_000);
    assert_eq!(ONE_MONTH_MS_31, 2_678_400_000);
}

#[test]
fn test_exact_year_values() {
    assert_eq!(ONE_YEAR_MS, 31_536_000_000);
    assert_eq!(ONE_YEAR_MS_366, 31_622_400_000);
}

#[test]
fn test_month_order() {
    assert!(ONE_MONTH_MS_28 < ONE_MONTH_MS_29);
    assert!(ONE_MONTH_MS_29 < ONE_MONTH_MS);
    assert!(ONE_MONTH_MS < ONE_MONTH_MS_31);
}

#[test]
fn test_month_variations() {
    assert_eq!(ONE_MONTH_MS_28, ONE_DAY_MS * 28);
    assert_eq!(ONE_MONTH_MS_29, ONE_DAY_MS * 29);
    assert_eq!(ONE_MONTH_MS, ONE_DAY_MS * 30);
    assert_eq!(ONE_MONTH_MS_31, ONE_DAY_MS * 31);
}

#[test]
fn test_time_unit_relationships() {
    assert_eq!(ONE_MINUTE_MS, ONE_SECOND_MS * 60);
    assert_eq!(ONE_HOUR_MS, ONE_MINUTE_MS * 60);
    assert_eq!(ONE_DAY_MS, ONE_HOUR_MS * 24);
    assert_eq!(ONE_WEEK_MS, ONE_DAY_MS * 7);
}

#[test]
fn test_year_relationship() {
    assert!(ONE_YEAR_MS < ONE_YEAR_MS_366);
    assert_eq!(ONE_YEAR_MS_366 - ONE_YEAR_MS, ONE_DAY_MS);
}

#[test]
fn test_year_variations() {
    assert_eq!(ONE_YEAR_MS, ONE_DAY_MS * 365);
    assert_eq!(ONE_YEAR_MS_366, ONE_DAY_MS * 366);
}
