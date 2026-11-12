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

#[test]
fn test_formats_japanese_special_words() {
    let base = base();
    assert_eq!(
        umt_format_relative(&(base + Duration::milliseconds(500)), &base, Some("ja")),
        "\u{4eca}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::days(1)), &base, Some("ja")),
        "\u{660e}\u{65e5}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::days(2)), &base, Some("ja")),
        "\u{660e}\u{5f8c}\u{65e5}"
    );
    assert_eq!(
        umt_format_relative(&(base - Duration::days(1)), &base, Some("ja")),
        "\u{6628}\u{65e5}"
    );
    assert_eq!(
        umt_format_relative(&(base - Duration::days(2)), &base, Some("ja")),
        "\u{4e00}\u{6628}\u{65e5}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::days(7)), &base, Some("ja")),
        "\u{6765}\u{9031}"
    );
    assert_eq!(
        umt_format_relative(&(base - Duration::days(7)), &base, Some("ja")),
        "\u{5148}\u{9031}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::days(30)), &base, Some("ja")),
        "\u{6765}\u{6708}"
    );
    assert_eq!(
        umt_format_relative(&(base - Duration::days(30)), &base, Some("ja")),
        "\u{5148}\u{6708}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::days(365)), &base, Some("ja")),
        "\u{6765}\u{5e74}"
    );
    assert_eq!(
        umt_format_relative(&(base - Duration::days(365)), &base, Some("ja")),
        "\u{6628}\u{5e74}"
    );
}

#[test]
fn test_formats_japanese_generic_forms() {
    let base = base();
    assert_eq!(
        umt_format_relative(&(base - Duration::seconds(5)), &base, Some("ja")),
        "5 \u{79d2}\u{524d}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::minutes(1)), &base, Some("ja")),
        "1 \u{5206}\u{5f8c}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::hours(1)), &base, Some("ja")),
        "1 \u{6642}\u{9593}\u{5f8c}"
    );
    assert_eq!(
        umt_format_relative(&(base - Duration::days(3)), &base, Some("ja")),
        "3 \u{65e5}\u{524d}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::days(14)), &base, Some("ja")),
        "2 \u{9031}\u{9593}\u{5f8c}"
    );
    assert_eq!(
        umt_format_relative(&(base + Duration::days(90)), &base, Some("ja")),
        "3 \u{304b}\u{6708}\u{5f8c}"
    );
    assert_eq!(
        umt_format_relative(&(base - Duration::days(730)), &base, Some("ja")),
        "2 \u{5e74}\u{524d}"
    );
}

#[test]
fn test_unsupported_or_missing_locale_uses_english() {
    let base = base();
    let target = base + Duration::days(1);
    assert_eq!(umt_format_relative(&target, &base, None), "tomorrow");
    assert_eq!(umt_format_relative(&target, &base, Some("fr")), "tomorrow");
}

#[test]
fn test_region_subtag_resolves_by_language() {
    let base = base();
    let target = base + Duration::days(1);
    assert_eq!(
        umt_format_relative(&target, &base, Some("ja-JP")),
        "\u{660e}\u{65e5}"
    );
    assert_eq!(
        umt_format_relative(&target, &base, Some("JA-jp")),
        "\u{660e}\u{65e5}"
    );
    assert_eq!(
        umt_format_relative(&target, &base, Some("en-US")),
        "tomorrow"
    );
    assert_eq!(umt_format_relative(&target, &base, Some("jam")), "tomorrow");
    assert_eq!(umt_format_relative(&target, &base, Some("")), "tomorrow");
}
