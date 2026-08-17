//! Tests for the get_quarter module.

use chrono::{TimeZone, Utc};
use umt_rust::date::umt_get_quarter;

#[test]
fn test_returns_1_for_january_through_march() {
    let jan = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
    let mar = Utc.with_ymd_and_hms(2025, 3, 31, 0, 0, 0).unwrap();
    assert_eq!(umt_get_quarter(&jan), 1);
    assert_eq!(umt_get_quarter(&mar), 1);
}

#[test]
fn test_returns_2_for_april_through_june() {
    let apr = Utc.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap();
    let jun = Utc.with_ymd_and_hms(2025, 6, 30, 0, 0, 0).unwrap();
    assert_eq!(umt_get_quarter(&apr), 2);
    assert_eq!(umt_get_quarter(&jun), 2);
}

#[test]
fn test_returns_3_for_july_through_september() {
    let jul = Utc.with_ymd_and_hms(2025, 7, 1, 0, 0, 0).unwrap();
    let sep = Utc.with_ymd_and_hms(2025, 9, 30, 0, 0, 0).unwrap();
    assert_eq!(umt_get_quarter(&jul), 3);
    assert_eq!(umt_get_quarter(&sep), 3);
}

#[test]
fn test_returns_4_for_october_through_december() {
    let oct = Utc.with_ymd_and_hms(2025, 10, 1, 0, 0, 0).unwrap();
    let dec = Utc.with_ymd_and_hms(2025, 12, 31, 0, 0, 0).unwrap();
    assert_eq!(umt_get_quarter(&oct), 4);
    assert_eq!(umt_get_quarter(&dec), 4);
}
