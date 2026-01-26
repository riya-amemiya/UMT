//! Tests for the day_of_week module.

use umt_rust::date::{umt_day_of_week, umt_today_day_of_week};

#[test]
fn test_day_of_week_for_specific_date() {
    // January 1, 2020 was a Wednesday (day 3)
    assert_eq!(
        umt_day_of_week(Some(2020), Some(1), Some(1), Some(0)),
        Some(3)
    );
    // January 2, 2020 was a Thursday (day 4)
    assert_eq!(
        umt_day_of_week(Some(2020), Some(1), Some(2), Some(0)),
        Some(4)
    );
}

#[test]
fn test_day_of_week_known_dates() {
    // January 1, 2025 is a Wednesday (day 3)
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(1), Some(0)),
        Some(3)
    );

    // January 5, 2025 is a Sunday (day 0)
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(5), Some(0)),
        Some(0)
    );

    // January 4, 2025 is a Saturday (day 6)
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(4), Some(0)),
        Some(6)
    );
}

#[test]
fn test_day_of_week_invalid_date() {
    // February 30 is invalid
    let dow = umt_day_of_week(Some(2025), Some(2), Some(30), Some(0));
    assert!(dow.is_none());
}

#[test]
fn test_today_day_of_week_returns_valid_value() {
    let dow = umt_today_day_of_week(Some(0));
    assert!(dow <= 6);
}

#[test]
fn test_day_of_week_all_days() {
    // Week of Jan 5-11, 2025
    // Sun=5, Mon=6, Tue=7, Wed=8, Thu=9, Fri=10, Sat=11
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(5), Some(0)),
        Some(0)
    ); // Sun
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(6), Some(0)),
        Some(1)
    ); // Mon
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(7), Some(0)),
        Some(2)
    ); // Tue
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(8), Some(0)),
        Some(3)
    ); // Wed
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(9), Some(0)),
        Some(4)
    ); // Thu
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(10), Some(0)),
        Some(5)
    ); // Fri
    assert_eq!(
        umt_day_of_week(Some(2025), Some(1), Some(11), Some(0)),
        Some(6)
    ); // Sat
}

use umt_rust::date::*;

#[test]
fn test_day_of_week_known_date() {
    // January 1, 2025 is a Wednesday
    let dow = umt_day_of_week(Some(2025), Some(1), Some(1), Some(0));
    assert_eq!(dow, Some(3)); // Wednesday = 3
}

#[test]
fn test_day_of_week_saturday() {
    // January 4, 2025 is a Saturday
    let dow = umt_day_of_week(Some(2025), Some(1), Some(4), Some(0));
    assert_eq!(dow, Some(6)); // Saturday = 6
}

#[test]
fn test_day_of_week_sunday() {
    // January 5, 2025 is a Sunday
    let dow = umt_day_of_week(Some(2025), Some(1), Some(5), Some(0));
    assert_eq!(dow, Some(0)); // Sunday = 0
}

#[test]
fn test_today_day_of_week() {
    let dow = umt_today_day_of_week(Some(0));
    assert!(dow <= 6);
}
