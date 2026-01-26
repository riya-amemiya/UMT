//! Tests for the date_range module.

use umt_rust::date::{UmtDateTime, umt_date_range};

#[test]
fn test_date_range_generates_array_of_dates() {
    let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let end = UmtDateTime::from_ymd_hms(2025, 1, 3, 0, 0, 0).unwrap();
    let dates = umt_date_range(start, end);

    assert_eq!(dates.len(), 3);
    assert_eq!(dates[0].format_str("%Y-%m-%d"), "2025-01-01");
    assert_eq!(dates[1].format_str("%Y-%m-%d"), "2025-01-02");
    assert_eq!(dates[2].format_str("%Y-%m-%d"), "2025-01-03");
}

#[test]
fn test_date_range_single_day() {
    let date = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let dates = umt_date_range(date, date);

    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0], date);
}

#[test]
fn test_date_range_creates_new_instances() {
    let start = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let end = UmtDateTime::from_ymd_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let dates = umt_date_range(start, end);

    assert_eq!(dates.len(), 1);
    assert_eq!(dates[0].day(), 1);
}

#[test]
fn test_date_range_month_and_year_transitions() {
    let start = UmtDateTime::from_ymd_hms(2024, 12, 30, 0, 0, 0).unwrap();
    let end = UmtDateTime::from_ymd_hms(2025, 1, 2, 0, 0, 0).unwrap();
    let dates = umt_date_range(start, end);

    assert_eq!(dates.len(), 4);
    assert_eq!(dates[0].format_str("%Y-%m-%d"), "2024-12-30");
    assert_eq!(dates[1].format_str("%Y-%m-%d"), "2024-12-31");
    assert_eq!(dates[2].format_str("%Y-%m-%d"), "2025-01-01");
    assert_eq!(dates[3].format_str("%Y-%m-%d"), "2025-01-02");
}
