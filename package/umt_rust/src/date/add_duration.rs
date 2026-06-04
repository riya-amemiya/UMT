//! Add a duration to a date.

use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Timelike, Utc};

use super::ms_by_unit::{DurationUnit, umt_ms_by_unit};

/// Returns the number of days in the given month, accounting for leap years.
fn last_day_of_month(year: i32, month: u32) -> u32 {
    // month is 1-12. Compute the first day of next month, then step back one day.
    let (next_year, next_month) = if month == 12 {
        (year + 1, 1)
    } else {
        (year, month + 1)
    };
    let first_of_next =
        NaiveDate::from_ymd_opt(next_year, next_month, 1).expect("valid first-of-month date");
    first_of_next.pred_opt().expect("valid previous day").day()
}

/// Adds a duration to a date and returns a new `DateTime<Utc>`.
///
/// Calendar-aware for `Month` (months) and `Y` (years), so end-of-month
/// is preserved (e.g. Jan 31 + 1 month = Feb 28/29). The `DateTime<Utc>`
/// fields are treated as the working wall-clock, matching the local-time
/// semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `date` - Base date
/// * `amount` - Amount to add (can be negative)
/// * `unit` - Unit of the amount
///
/// # Returns
///
/// A new `DateTime<Utc>` with the duration added.
///
/// # Examples
///
/// ```
/// use chrono::{Datelike, TimeZone, Utc};
/// use umt_rust::date::{umt_add_duration, DurationUnit};
///
/// let base = Utc.with_ymd_and_hms(2025, 1, 31, 0, 0, 0).unwrap();
/// let result = umt_add_duration(&base, 1, DurationUnit::Month);
/// assert_eq!(result.year(), 2025);
/// assert_eq!(result.month(), 2);
/// assert_eq!(result.day(), 28);
///
/// let base = Utc.with_ymd_and_hms(2025, 1, 1, 0, 0, 0).unwrap();
/// let result = umt_add_duration(&base, 7, DurationUnit::D);
/// assert_eq!(result.day(), 8);
/// ```
pub fn umt_add_duration(date: &DateTime<Utc>, amount: i64, unit: DurationUnit) -> DateTime<Utc> {
    if let Some(ms) = umt_ms_by_unit(unit) {
        return *date + chrono::Duration::milliseconds(amount * ms as i64);
    }

    let year = date.year();
    let month0 = date.month0() as i64; // 0-based month, like JS getMonth()
    let day = date.day();

    let (target_year, target_month0) = if unit == DurationUnit::Month {
        let total = month0 + amount;
        let new_month0 = total.rem_euclid(12);
        let new_year = year as i64 + total.div_euclid(12);
        (new_year as i32, new_month0 as u32)
    } else {
        ((year as i64 + amount) as i32, month0 as u32)
    };

    let target_month = target_month0 + 1; // back to 1-based
    let last_day = last_day_of_month(target_year, target_month);
    let clamped_day = day.min(last_day);

    let new_date = NaiveDate::from_ymd_opt(target_year, target_month, clamped_day)
        .expect("valid clamped target date");
    let new_datetime = new_date.and_hms_milli_opt(
        date.hour(),
        date.minute(),
        date.second(),
        date.nanosecond() / 1_000_000,
    );
    let new_datetime = new_datetime.expect("valid time-of-day");

    Utc.from_utc_datetime(&new_datetime)
}
