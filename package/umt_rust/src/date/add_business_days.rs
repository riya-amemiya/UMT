//! Add business days to a date.

use chrono::{DateTime, Duration, Utc};

use super::is_business_day::umt_is_business_day;

/// Adds business days to a date, skipping weekends and holidays.
///
/// The starting date is not counted. A zero amount returns a clone.
/// The `DateTime<Utc>` fields are treated as the working wall-clock,
/// matching the local-time semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `date` - Base date
/// * `amount` - Business days to add (may be negative)
/// * `holidays` - Optional holiday dates
///
/// # Examples
///
/// ```
/// use chrono::{Datelike, TimeZone, Utc};
/// use umt_rust::date::umt_add_business_days;
///
/// let friday = Utc.with_ymd_and_hms(2025, 4, 18, 9, 30, 0).unwrap();
/// let result = umt_add_business_days(&friday, 1, &[]);
/// assert_eq!(result.day(), 21);
/// ```
pub fn umt_add_business_days(
    date: &DateTime<Utc>,
    amount: i64,
    holidays: &[DateTime<Utc>],
) -> DateTime<Utc> {
    if amount == 0 {
        return *date;
    }
    let step = if amount > 0 { 1 } else { -1 };
    let mut remaining = amount.unsigned_abs();
    let mut result = *date;
    while remaining > 0 {
        result += Duration::days(step);
        if umt_is_business_day(&result, holidays) {
            remaining -= 1;
        }
    }
    result
}
