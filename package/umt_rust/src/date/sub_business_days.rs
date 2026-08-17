//! Subtract business days from a date.

use chrono::{DateTime, Utc};

use super::add_business_days::umt_add_business_days;

/// Subtracts business days from a date.
///
/// Equivalent to [`umt_add_business_days`]`(date, -amount, holidays)`.
///
/// # Arguments
///
/// * `date` - Base date
/// * `amount` - Business days to subtract
/// * `holidays` - Optional holiday dates
///
/// # Examples
///
/// ```
/// use chrono::{Datelike, TimeZone, Utc};
/// use umt_rust::date::umt_sub_business_days;
///
/// let monday = Utc.with_ymd_and_hms(2025, 4, 21, 9, 30, 0).unwrap();
/// let result = umt_sub_business_days(&monday, 1, &[]);
/// assert_eq!(result.day(), 18);
/// ```
pub fn umt_sub_business_days(
    date: &DateTime<Utc>,
    amount: i64,
    holidays: &[DateTime<Utc>],
) -> DateTime<Utc> {
    umt_add_business_days(date, -amount, holidays)
}
