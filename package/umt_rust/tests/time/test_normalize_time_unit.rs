use umt_rust::time::{umt_normalize_time_unit, NormalizeFormat, TimeUnit};

#[test]
fn test_converts_long_format_to_long_format() {
    assert_eq!(
        umt_normalize_time_unit("milliseconds", NormalizeFormat::Long),
        Some("milliseconds".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("seconds", NormalizeFormat::Long),
        Some("seconds".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("minutes", NormalizeFormat::Long),
        Some("minutes".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("hours", NormalizeFormat::Long),
        Some("hours".to_string())
    );
}

#[test]
fn test_converts_long_format_to_short_format() {
    assert_eq!(
        umt_normalize_time_unit("milliseconds", NormalizeFormat::Short),
        Some("ms".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("seconds", NormalizeFormat::Short),
        Some("s".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("minutes", NormalizeFormat::Short),
        Some("m".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("hours", NormalizeFormat::Short),
        Some("h".to_string())
    );
}

#[test]
fn test_converts_short_format_to_long_format() {
    assert_eq!(
        umt_normalize_time_unit("ms", NormalizeFormat::Long),
        Some("milliseconds".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("s", NormalizeFormat::Long),
        Some("seconds".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("m", NormalizeFormat::Long),
        Some("minutes".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("h", NormalizeFormat::Long),
        Some("hours".to_string())
    );
}

#[test]
fn test_converts_short_format_to_short_format() {
    assert_eq!(
        umt_normalize_time_unit("ms", NormalizeFormat::Short),
        Some("ms".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("s", NormalizeFormat::Short),
        Some("s".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("m", NormalizeFormat::Short),
        Some("m".to_string())
    );
    assert_eq!(
        umt_normalize_time_unit("h", NormalizeFormat::Short),
        Some("h".to_string())
    );
}

#[test]
fn test_handles_all_supported_units_comprehensively() {
    let long_units = ["milliseconds", "seconds", "minutes", "hours"];
    let short_units = ["ms", "s", "m", "h"];

    for (index, unit) in long_units.iter().enumerate() {
        assert_eq!(
            umt_normalize_time_unit(unit, NormalizeFormat::Short),
            Some(short_units[index].to_string())
        );
        assert_eq!(
            umt_normalize_time_unit(unit, NormalizeFormat::Long),
            Some(unit.to_string())
        );
    }

    for (index, unit) in short_units.iter().enumerate() {
        assert_eq!(
            umt_normalize_time_unit(unit, NormalizeFormat::Long),
            Some(long_units[index].to_string())
        );
        assert_eq!(
            umt_normalize_time_unit(unit, NormalizeFormat::Short),
            Some(unit.to_string())
        );
    }
}

#[test]
fn test_time_unit_enum() {
    // Test TimeUnit enum methods
    assert_eq!(TimeUnit::Milliseconds.to_long(), "milliseconds");
    assert_eq!(TimeUnit::Seconds.to_long(), "seconds");
    assert_eq!(TimeUnit::Minutes.to_long(), "minutes");
    assert_eq!(TimeUnit::Hours.to_long(), "hours");

    assert_eq!(TimeUnit::Milliseconds.to_short(), "ms");
    assert_eq!(TimeUnit::Seconds.to_short(), "s");
    assert_eq!(TimeUnit::Minutes.to_short(), "m");
    assert_eq!(TimeUnit::Hours.to_short(), "h");
}

#[test]
fn test_time_unit_from_str() {
    assert_eq!(TimeUnit::from_str("milliseconds"), Some(TimeUnit::Milliseconds));
    assert_eq!(TimeUnit::from_str("ms"), Some(TimeUnit::Milliseconds));
    assert_eq!(TimeUnit::from_str("seconds"), Some(TimeUnit::Seconds));
    assert_eq!(TimeUnit::from_str("s"), Some(TimeUnit::Seconds));
    assert_eq!(TimeUnit::from_str("minutes"), Some(TimeUnit::Minutes));
    assert_eq!(TimeUnit::from_str("m"), Some(TimeUnit::Minutes));
    assert_eq!(TimeUnit::from_str("hours"), Some(TimeUnit::Hours));
    assert_eq!(TimeUnit::from_str("h"), Some(TimeUnit::Hours));
    assert_eq!(TimeUnit::from_str("invalid"), None);
}

#[test]
fn test_invalid_units() {
    assert_eq!(umt_normalize_time_unit("invalid", NormalizeFormat::Long), None);
    assert_eq!(umt_normalize_time_unit("invalid", NormalizeFormat::Short), None);
    assert_eq!(umt_normalize_time_unit("", NormalizeFormat::Long), None);
    assert_eq!(umt_normalize_time_unit("", NormalizeFormat::Short), None);
}
