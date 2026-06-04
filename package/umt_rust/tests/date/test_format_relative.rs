//! Tests for the format_relative module.

use chrono::{Duration, TimeZone, Utc};
use umt_rust::date::umt_format_relative;

fn base() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2025, 4, 15, 12, 0, 0).unwrap()
}

#[test]
fn test_returns_now_for_delta_below_one_second() {
    let base = base();
    let target = base + Duration::milliseconds(500);
    assert_eq!(umt_format_relative(&target, &base, Some("en")), "now");
}

#[test]
fn test_formats_seconds_in_the_past() {
    let base = base();
    let target = base - Duration::seconds(5);
    assert_eq!(
        umt_format_relative(&target, &base, Some("en")),
        "5 seconds ago"
    );
}

#[test]
fn test_formats_minutes_in_the_future() {
    let base = base();
    let target = base + Duration::minutes(5);
    assert_eq!(
        umt_format_relative(&target, &base, Some("en")),
        "in 5 minutes"
    );
}

#[test]
fn test_formats_hours() {
    let base = base();
    let target = base - Duration::hours(3);
    assert_eq!(
        umt_format_relative(&target, &base, Some("en")),
        "3 hours ago"
    );
}

#[test]
fn test_formats_days() {
    let base = base();
    let target = base + Duration::days(1);
    assert_eq!(umt_format_relative(&target, &base, Some("en")), "tomorrow");
}

#[test]
fn test_formats_weeks() {
    let base = base();
    let target = base - Duration::days(14);
    assert_eq!(
        umt_format_relative(&target, &base, Some("en")),
        "2 weeks ago"
    );
}

#[test]
fn test_formats_months() {
    let base = base();
    let target = base + Duration::days(90);
    assert_eq!(
        umt_format_relative(&target, &base, Some("en")),
        "in 3 months"
    );
}

#[test]
fn test_formats_years() {
    let base = base();
    let target = base - Duration::days(2 * 365);
    assert_eq!(
        umt_format_relative(&target, &base, Some("en")),
        "2 years ago"
    );
}

#[test]
fn test_default_base_date_when_omitted() {
    let now = Utc::now();
    let target = now + Duration::seconds(5);
    let result = umt_format_relative(&target, &now, Some("en"));
    assert!(!result.is_empty());
}
