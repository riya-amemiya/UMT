//! Duration unit definitions and millisecond lookups.
//!
//! This module mirrors the TypeScript `DurationUnit` type and the
//! `msByUnit` lookup table used by the date arithmetic helpers.

use crate::consts::clock::{ONE_DAY_MS, ONE_HOUR_MS, ONE_MINUTE_MS, ONE_SECOND_MS, ONE_WEEK_MS};

/// Supported duration units for date arithmetic.
///
/// - `Ms`: milliseconds
/// - `S`: seconds
/// - `M`: minutes
/// - `H`: hours
/// - `D`: days
/// - `W`: weeks
/// - `Month`: months (calendar-aware)
/// - `Y`: years (calendar-aware)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DurationUnit {
    /// Milliseconds.
    Ms,
    /// Seconds.
    S,
    /// Minutes.
    M,
    /// Hours.
    H,
    /// Days.
    D,
    /// Weeks.
    W,
    /// Months (calendar-aware).
    Month,
    /// Years (calendar-aware).
    Y,
}

/// Returns the fixed number of milliseconds for a duration unit.
///
/// Calendar-aware units (`Month`, `Y`) have no fixed millisecond size and
/// return `None`, matching the partial `msByUnit` record in TypeScript.
///
/// # Arguments
///
/// * `unit` - The duration unit to look up
///
/// # Returns
///
/// `Some(milliseconds)` for fixed-size units, `None` for `Month` and `Y`.
///
/// # Examples
///
/// ```
/// use umt_rust::date::{umt_ms_by_unit, DurationUnit};
///
/// assert_eq!(umt_ms_by_unit(DurationUnit::Ms), Some(1));
/// assert_eq!(umt_ms_by_unit(DurationUnit::S), Some(1000));
/// assert_eq!(umt_ms_by_unit(DurationUnit::D), Some(86_400_000));
/// assert_eq!(umt_ms_by_unit(DurationUnit::Month), None);
/// assert_eq!(umt_ms_by_unit(DurationUnit::Y), None);
/// ```
#[inline]
pub fn umt_ms_by_unit(unit: DurationUnit) -> Option<u64> {
    match unit {
        DurationUnit::Ms => Some(1),
        DurationUnit::S => Some(ONE_SECOND_MS),
        DurationUnit::M => Some(ONE_MINUTE_MS),
        DurationUnit::H => Some(ONE_HOUR_MS),
        DurationUnit::D => Some(ONE_DAY_MS),
        DurationUnit::W => Some(ONE_WEEK_MS),
        DurationUnit::Month | DurationUnit::Y => None,
    }
}
