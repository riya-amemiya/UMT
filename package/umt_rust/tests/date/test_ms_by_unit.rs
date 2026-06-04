//! Tests for the ms_by_unit module.

use umt_rust::date::{DurationUnit, umt_ms_by_unit};

#[test]
fn test_fixed_units_return_milliseconds() {
    assert_eq!(umt_ms_by_unit(DurationUnit::Ms), Some(1));
    assert_eq!(umt_ms_by_unit(DurationUnit::S), Some(1000));
    assert_eq!(umt_ms_by_unit(DurationUnit::M), Some(60_000));
    assert_eq!(umt_ms_by_unit(DurationUnit::H), Some(3_600_000));
    assert_eq!(umt_ms_by_unit(DurationUnit::D), Some(86_400_000));
    assert_eq!(umt_ms_by_unit(DurationUnit::W), Some(604_800_000));
}

#[test]
fn test_calendar_units_return_none() {
    assert_eq!(umt_ms_by_unit(DurationUnit::Month), None);
    assert_eq!(umt_ms_by_unit(DurationUnit::Y), None);
}
