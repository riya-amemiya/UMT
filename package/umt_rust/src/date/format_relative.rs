//! Relative-time formatting utility.

use chrono::{DateTime, Utc};

use crate::consts::clock::{
    ONE_DAY_MS, ONE_HOUR_MS, ONE_MINUTE_MS, ONE_MONTH_MS, ONE_SECOND_MS, ONE_WEEK_MS, ONE_YEAR_MS,
};
use crate::math::js_round;

/// A relative-time unit, mirroring the subset of `Intl.RelativeTimeFormat`
/// units used by the threshold table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RelativeUnit {
    Year,
    Month,
    Week,
    Day,
    Hour,
    Minute,
    Second,
}

const THRESHOLDS: [(RelativeUnit, u64); 7] = [
    (RelativeUnit::Year, ONE_YEAR_MS),
    (RelativeUnit::Month, ONE_MONTH_MS),
    (RelativeUnit::Week, ONE_WEEK_MS),
    (RelativeUnit::Day, ONE_DAY_MS),
    (RelativeUnit::Hour, ONE_HOUR_MS),
    (RelativeUnit::Minute, ONE_MINUTE_MS),
    (RelativeUnit::Second, ONE_SECOND_MS),
];

fn unit_singular(unit: RelativeUnit) -> &'static str {
    match unit {
        RelativeUnit::Year => "year",
        RelativeUnit::Month => "month",
        RelativeUnit::Week => "week",
        RelativeUnit::Day => "day",
        RelativeUnit::Hour => "hour",
        RelativeUnit::Minute => "minute",
        RelativeUnit::Second => "second",
    }
}

/// Special wording for `numeric: "auto"` at value +1 / -1, returning `None`
/// when the unit has no special phrasing (hour, minute, second).
fn auto_special(unit: RelativeUnit, value: i64) -> Option<&'static str> {
    match (unit, value) {
        (RelativeUnit::Year, 1) => Some("next year"),
        (RelativeUnit::Year, -1) => Some("last year"),
        (RelativeUnit::Month, 1) => Some("next month"),
        (RelativeUnit::Month, -1) => Some("last month"),
        (RelativeUnit::Week, 1) => Some("next week"),
        (RelativeUnit::Week, -1) => Some("last week"),
        (RelativeUnit::Day, 1) => Some("tomorrow"),
        (RelativeUnit::Day, -1) => Some("yesterday"),
        _ => None,
    }
}

/// Formats a `value`/`unit` pair the way `Intl.RelativeTimeFormat("en",
/// { numeric: "auto" })` would.
fn format_en_auto(value: i64, unit: RelativeUnit) -> String {
    if value == 0 && unit == RelativeUnit::Second {
        return "now".to_string();
    }
    if let Some(special) = auto_special(unit, value) {
        return special.to_string();
    }

    let abs = value.abs();
    let mut label = unit_singular(unit).to_string();
    if abs != 1 {
        label.push('s');
    }

    if value > 0 {
        format!("in {} {}", abs, label)
    } else {
        format!("{} {} ago", abs, label)
    }
}

fn unit_ja(unit: RelativeUnit) -> &'static str {
    match unit {
        RelativeUnit::Year => "\u{5e74}",
        RelativeUnit::Month => "\u{304b}\u{6708}",
        RelativeUnit::Week => "\u{9031}\u{9593}",
        RelativeUnit::Day => "\u{65e5}",
        RelativeUnit::Hour => "\u{6642}\u{9593}",
        RelativeUnit::Minute => "\u{5206}",
        RelativeUnit::Second => "\u{79d2}",
    }
}

/// Japanese special wording for `numeric: "auto"`, returning `None` when the
/// value has no special phrasing for the unit.
fn auto_special_ja(unit: RelativeUnit, value: i64) -> Option<&'static str> {
    match (unit, value) {
        (RelativeUnit::Year, 1) => Some("\u{6765}\u{5e74}"),
        (RelativeUnit::Year, -1) => Some("\u{6628}\u{5e74}"),
        (RelativeUnit::Month, 1) => Some("\u{6765}\u{6708}"),
        (RelativeUnit::Month, -1) => Some("\u{5148}\u{6708}"),
        (RelativeUnit::Week, 1) => Some("\u{6765}\u{9031}"),
        (RelativeUnit::Week, -1) => Some("\u{5148}\u{9031}"),
        (RelativeUnit::Day, 2) => Some("\u{660e}\u{5f8c}\u{65e5}"),
        (RelativeUnit::Day, 1) => Some("\u{660e}\u{65e5}"),
        (RelativeUnit::Day, -1) => Some("\u{6628}\u{65e5}"),
        (RelativeUnit::Day, -2) => Some("\u{4e00}\u{6628}\u{65e5}"),
        (RelativeUnit::Second, 0) => Some("\u{4eca}"),
        _ => None,
    }
}

/// Formats a `value`/`unit` pair the way `Intl.RelativeTimeFormat("ja",
/// { numeric: "auto" })` would.
fn format_ja_auto(value: i64, unit: RelativeUnit) -> String {
    if let Some(special) = auto_special_ja(unit, value) {
        return special.to_string();
    }

    if value > 0 {
        format!("{} {}\u{5f8c}", value, unit_ja(unit))
    } else {
        format!("{} {}\u{524d}", value.abs(), unit_ja(unit))
    }
}

/// Formats a date relative to a base date.
///
/// Picks the largest unit that fits the absolute delta and formats it the
/// way `Intl.RelativeTimeFormat(locale, { numeric: "auto" })` would for the
/// English (`"en"`) and Japanese (`"ja"`) locales. Other locales fall back
/// to English wording, because the Rust standard library and `chrono` do not
/// provide `Intl`-style locale data.
///
/// # Arguments
///
/// * `date` - The target date
/// * `base_date` - Reference date
/// * `locale` - BCP 47 locale tag (`"en"` and `"ja"` are supported; the
///   language subtag decides, so `"ja-JP"` formats as Japanese)
///
/// # Returns
///
/// A relative-time string.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_format_relative;
///
/// let base = Utc.with_ymd_and_hms(2025, 4, 15, 12, 0, 0).unwrap();
/// let target = base - chrono::Duration::seconds(5);
/// assert_eq!(umt_format_relative(&target, &base, Some("en")), "5 seconds ago");
///
/// let target = base + chrono::Duration::days(1);
/// assert_eq!(umt_format_relative(&target, &base, Some("en")), "tomorrow");
/// ```
pub fn umt_format_relative(
    date: &DateTime<Utc>,
    base_date: &DateTime<Utc>,
    locale: Option<&str>,
) -> String {
    let is_japanese = locale.is_some_and(|tag| {
        tag.split('-')
            .next()
            .is_some_and(|language| language.eq_ignore_ascii_case("ja"))
    });
    let formatter = if is_japanese {
        format_ja_auto
    } else {
        format_en_auto
    };
    let delta_ms = date.timestamp_millis() - base_date.timestamp_millis();
    let abs_delta = delta_ms.unsigned_abs();

    for (unit, ms) in THRESHOLDS {
        if abs_delta >= ms {
            let value = js_round(delta_ms as f64 / ms as f64) as i64;
            return formatter(value, unit);
        }
    }
    formatter(0, RelativeUnit::Second)
}
