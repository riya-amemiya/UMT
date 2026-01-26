//! Tests for the is_leap_year module.

use umt_rust::date::umt_is_leap_year;

#[test]
fn test_leap_year_divisible_by_4_not_100() {
    assert!(umt_is_leap_year(2020));
    assert!(umt_is_leap_year(2024));
    assert!(umt_is_leap_year(1996));
    assert!(umt_is_leap_year(2004));
    assert!(umt_is_leap_year(2008));
}

#[test]
fn test_not_leap_year_divisible_by_100_not_400() {
    assert!(!umt_is_leap_year(1900));
    assert!(!umt_is_leap_year(2100));
    assert!(!umt_is_leap_year(1700));
    assert!(!umt_is_leap_year(1800));
    assert!(!umt_is_leap_year(2200));
}

#[test]
fn test_leap_year_divisible_by_400() {
    assert!(umt_is_leap_year(2000));
    assert!(umt_is_leap_year(1600));
    assert!(umt_is_leap_year(2400));
    assert!(umt_is_leap_year(800));
    assert!(umt_is_leap_year(1200));
}

#[test]
fn test_not_leap_year() {
    assert!(!umt_is_leap_year(2023));
    assert!(!umt_is_leap_year(2025));
    assert!(!umt_is_leap_year(1997));
    assert!(!umt_is_leap_year(2001));
    assert!(!umt_is_leap_year(2003));
}

#[test]
fn test_early_years() {
    assert!(umt_is_leap_year(4));
    assert!(!umt_is_leap_year(100));
    assert!(umt_is_leap_year(400));
    assert!(umt_is_leap_year(8));
    assert!(!umt_is_leap_year(1));
}

#[test]
fn test_edge_cases_and_boundary_values() {
    assert!(umt_is_leap_year(0));
    assert!(umt_is_leap_year(-4));
    assert!(!umt_is_leap_year(-100));
    assert!(umt_is_leap_year(-400));
}

#[test]
fn test_very_large_years() {
    assert!(umt_is_leap_year(4000));
    assert!(umt_is_leap_year(8000));
    assert!(!umt_is_leap_year(9999));
    assert!(umt_is_leap_year(10_000));
}

#[test]
fn test_leap_year_pattern() {
    let recent_leap_years = [1996, 2000, 2004, 2008, 2012, 2016, 2020, 2024];
    let recent_non_leap_years = [
        1997, 1998, 1999, 2001, 2002, 2003, 2005, 2006, 2007, 2009, 2010, 2011, 2013, 2014, 2015,
        2017, 2018, 2019, 2021, 2022, 2023,
    ];

    for year in recent_leap_years {
        assert!(umt_is_leap_year(year), "{} should be a leap year", year);
    }

    for year in recent_non_leap_years {
        assert!(
            !umt_is_leap_year(year),
            "{} should not be a leap year",
            year
        );
    }
}
