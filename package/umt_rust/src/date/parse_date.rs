//! Date parsing utility.

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use regex::Regex;
use std::sync::LazyLock;

static FORMAT_TOKEN_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[([^\]]+)]|YYYY|YY|MM|M|DD|D|d|HH|H|hh|h|mm|m|ss|s|SSS|A|a|ZZ|Z|(?s:(.))")
        .unwrap()
});

fn token_pattern(token: &str) -> &'static str {
    match token {
        "YYYY" => "[0-9]{4}",
        "YY" | "MM" | "DD" | "HH" | "hh" | "mm" | "ss" => "[0-9]{2}",
        "M" | "D" | "H" | "h" | "m" | "s" => "[0-9]{1,2}",
        "d" => "[0-6]",
        "SSS" => "[0-9]{3}",
        "A" => "AM|PM",
        "a" => "am|pm",
        "ZZ" => "[+-][0-9]{4}",
        _ => "[+-][0-9]{2}:[0-9]{2}",
    }
}

struct ParsedFields {
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: u32,
    millisecond: u32,
    twelve_hour: bool,
    pm: Option<bool>,
    day_of_week: Option<u32>,
    offset: Option<i64>,
    offset_in_range: bool,
}

impl ParsedFields {
    fn apply(&mut self, token: &str, value: &str) {
        let number = || value.parse::<u32>().unwrap_or(0);
        match token {
            "YYYY" => self.year = number() as i32,
            "YY" => self.year = 2000 + number() as i32,
            "MM" | "M" => self.month = number(),
            "DD" | "D" => self.day = number(),
            "d" => self.day_of_week = Some(number()),
            "HH" | "H" => {
                self.hour = number();
                self.twelve_hour = false;
            }
            "hh" | "h" => {
                self.hour = number();
                self.twelve_hour = true;
            }
            "mm" | "m" => self.minute = number(),
            "ss" | "s" => self.second = number(),
            "SSS" => self.millisecond = number(),
            "A" | "a" => self.pm = Some(value.eq_ignore_ascii_case("pm")),
            _ => {
                let digits = value.replace(':', "");
                let hours = digits[1..3].parse::<i64>().unwrap_or(0);
                let minutes = digits[3..5].parse::<i64>().unwrap_or(0);
                let sign = if digits.starts_with('-') { -1 } else { 1 };
                self.offset = Some(sign * (hours * 60 + minutes));
                self.offset_in_range = hours <= 23 && minutes <= 59;
            }
        }
    }
}

/// Parses a string into a date according to a format pattern.
///
/// Uses the same tokens as [`umt_format`](super::umt_format): `YYYY`, `YY`,
/// `MM`, `M`, `DD`, `D`, `d`, `HH`, `H`, `hh`, `h`, `mm`, `m`, `ss`, `s`,
/// `SSS`, `A`, `a`, `Z`, `ZZ`, and `[text]` for escaped literal text. Any
/// other character in the pattern must match literally.
///
/// - `YY` is read as 2000-2099.
/// - Fields missing from the pattern default to 1970-01-01 00:00:00.000.
/// - `hh` / `h` are converted with `A` / `a` when present (12 AM is 0,
///   12 PM is 12); without a meridiem token the hour is used as written.
/// - `d` (day of week, 0 = Sunday) must agree with the parsed calendar date.
/// - Without `Z` / `ZZ`, the fields are returned as wall-clock
///   `DateTime<Utc>` fields, matching the local-time semantics of the
///   TypeScript reference. With `Z` / `ZZ`, the offset is subtracted so the
///   result is the UTC instant.
/// - When a token appears more than once, the last occurrence wins.
///
/// # Arguments
///
/// * `string` - The date string to parse
/// * `format_string` - The format pattern string
///
/// # Returns
///
/// `Some(DateTime<Utc>)` on success, or `None` when the string does not match
/// the pattern or a field is out of range (month 1-12, day within the month,
/// hour 0-23 or 1-12 for 12-hour tokens, minute and second 0-59, offset hours
/// 0-23 and minutes 0-59).
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_parse_date;
///
/// assert_eq!(
///     umt_parse_date("2025-04-04", "YYYY-MM-DD"),
///     Some(Utc.with_ymd_and_hms(2025, 4, 4, 0, 0, 0).unwrap())
/// );
/// assert_eq!(
///     umt_parse_date("04/04/2025 3:30 PM", "MM/DD/YYYY h:mm A"),
///     Some(Utc.with_ymd_and_hms(2025, 4, 4, 15, 30, 0).unwrap())
/// );
/// assert_eq!(
///     umt_parse_date("2025-04-04T15:30:00+09:00", "YYYY-MM-DDTHH:mm:ssZ"),
///     Some(Utc.with_ymd_and_hms(2025, 4, 4, 6, 30, 0).unwrap())
/// );
/// assert_eq!(umt_parse_date("2025-13-01", "YYYY-MM-DD"), None);
/// ```
pub fn umt_parse_date(string: &str, format_string: &str) -> Option<DateTime<Utc>> {
    let mut tokens: Vec<&str> = Vec::new();
    let mut source = String::from("^");
    for captures in FORMAT_TOKEN_RE.captures_iter(format_string) {
        match captures.get(1).or_else(|| captures.get(2)) {
            Some(text) => source.push_str(&regex::escape(text.as_str())),
            None => {
                let token = captures.get(0).unwrap().as_str();
                tokens.push(token);
                source.push('(');
                source.push_str(token_pattern(token));
                source.push(')');
            }
        }
    }
    source.push('$');
    let pattern = Regex::new(&source).ok()?;
    let matched = pattern.captures(string)?;

    let mut fields = ParsedFields {
        year: 1970,
        month: 1,
        day: 1,
        hour: 0,
        minute: 0,
        second: 0,
        millisecond: 0,
        twelve_hour: false,
        pm: None,
        day_of_week: None,
        offset: None,
        offset_in_range: true,
    };
    for (index, token) in tokens.iter().enumerate() {
        fields.apply(token, matched.get(index + 1).map_or("", |m| m.as_str()));
    }

    let date = NaiveDate::from_ymd_opt(fields.year, fields.month, fields.day)?;
    let hour_in_range = if fields.twelve_hour {
        (1..=12).contains(&fields.hour)
    } else {
        fields.hour <= 23
    };
    let day_of_week_matches = fields
        .day_of_week
        .is_none_or(|day| day == date.weekday().num_days_from_sunday());
    if !(hour_in_range
        && fields.minute <= 59
        && fields.second <= 59
        && day_of_week_matches
        && fields.offset_in_range)
    {
        return None;
    }

    let hour = match (fields.twelve_hour, fields.pm) {
        (true, Some(pm)) => fields.hour % 12 + if pm { 12 } else { 0 },
        _ => fields.hour,
    };
    let wall_clock =
        date.and_hms_milli_opt(hour, fields.minute, fields.second, fields.millisecond)?;
    Some(Utc.from_utc_datetime(&wall_clock) - Duration::minutes(fields.offset.unwrap_or(0)))
}
