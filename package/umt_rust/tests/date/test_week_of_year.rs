//! Tests for the week_of_year module.

use chrono::{TimeZone, Utc};
use umt_rust::date::{DateBoundaryUnit, umt_start_of, umt_week_of_year};

#[test]
fn test_returns_1_for_january_1() {
    let jan1 = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    assert_eq!(umt_week_of_year(&jan1), 1);
}

#[test]
fn test_keeps_dates_in_january_1_sunday_start_week_as_week_1() {
    let jan4 = Utc.with_ymd_and_hms(2025, 1, 4, 0, 0, 0).unwrap();
    assert_eq!(umt_week_of_year(&jan4), 1);
}

#[test]
fn test_increments_on_the_following_sunday() {
    let jan5 = Utc.with_ymd_and_hms(2025, 1, 5, 0, 0, 0).unwrap();
    assert_eq!(umt_week_of_year(&jan5), 2);
}

#[test]
fn test_matches_start_of_week_boundaries_across_a_year() {
    let date = Utc.with_ymd_and_hms(2025, 4, 16, 0, 0, 0).unwrap();
    let year_start = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let year_start_week = umt_start_of(&year_start, DateBoundaryUnit::Week);
    let current_week = umt_start_of(&date, DateBoundaryUnit::Week);
    let expected = (current_week - year_start_week).num_days() / 7 + 1;
    assert_eq!(umt_week_of_year(&date), expected);
}
