//! Count business days between two dates.

use chrono::{DateTime, Duration, Utc};

use super::is_business_day::umt_is_business_day;
use super::start_of::{DateBoundaryUnit, umt_start_of};

/// Counts business days from `start` to `end`.
///
/// Business days follow [`umt_is_business_day`] (weekdays that are not listed
/// holidays). Days are compared by calendar date; the time of day is
/// ignored. The start day is excluded and the end day is included, so the
/// result is the inverse of [`umt_add_business_days`](super::umt_add_business_days):
/// when `end` is a business day,
/// `umt_add_business_days(start, umt_business_days_between(start, end, holidays), holidays)`
/// lands on the calendar day of `end`. When `end` is before `start`, the
/// result is negative (the start day is still excluded and the end day is
/// still included). The `DateTime<Utc>` fields are treated as the working
/// wall-clock, matching the local-time semantics of the TypeScript reference.
///
/// # Arguments
///
/// * `start` - Start date (excluded)
/// * `end` - End date (included)
/// * `holidays` - Holiday dates (compared by calendar day)
///
/// # Returns
///
/// Signed number of business days between the dates.
///
/// # Examples
///
/// ```
/// use chrono::{TimeZone, Utc};
/// use umt_rust::date::umt_business_days_between;
///
/// let friday = Utc.with_ymd_and_hms(2025, 4, 18, 0, 0, 0).unwrap();
/// let monday = Utc.with_ymd_and_hms(2025, 4, 21, 0, 0, 0).unwrap();
/// assert_eq!(umt_business_days_between(&friday, &monday, &[]), 1);
/// assert_eq!(umt_business_days_between(&monday, &friday, &[]), -1);
/// ```
pub fn umt_business_days_between(
    start: &DateTime<Utc>,
    end: &DateTime<Utc>,
    holidays: &[DateTime<Utc>],
) -> i64 {
    let from = umt_start_of(start, DateBoundaryUnit::Day);
    let days = (end.date_naive() - start.date_naive()).num_days();
    let step = if days < 0 { -1 } else { 1 };
    (1..=days.abs())
        .filter(|offset| umt_is_business_day(&(from + Duration::days(step * offset)), holidays))
        .map(|_| step)
        .sum()
}
