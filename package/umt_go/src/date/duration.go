package date

import (
	"math"
	"time"

	"github.com/riya-amemiya/umt-go/src/consts"
)

// DateBoundaryUnit identifies the calendar boundary used by StartOf and EndOf.
type DateBoundaryUnit string

// Boundary units accepted by StartOf and EndOf.
const (
	BoundarySecond  DateBoundaryUnit = "second"
	BoundaryMinute  DateBoundaryUnit = "minute"
	BoundaryHour    DateBoundaryUnit = "hour"
	BoundaryDay     DateBoundaryUnit = "day"
	BoundaryWeek    DateBoundaryUnit = "week"
	BoundaryMonth   DateBoundaryUnit = "month"
	BoundaryQuarter DateBoundaryUnit = "quarter"
	BoundaryYear    DateBoundaryUnit = "year"
)

// DurationUnit identifies a supported duration unit for date arithmetic.
//
//   - "ms": milliseconds
//   - "s": seconds
//   - "m": minutes
//   - "h": hours
//   - "d": days
//   - "w": weeks
//   - "M": months (calendar-aware)
//   - "y": years (calendar-aware)
type DurationUnit string

// Duration units accepted by AddDuration, SubDuration, and Diff.
const (
	UnitMillisecond DurationUnit = "ms"
	UnitSecond      DurationUnit = "s"
	UnitMinute      DurationUnit = "m"
	UnitHour        DurationUnit = "h"
	UnitDay         DurationUnit = "d"
	UnitWeek        DurationUnit = "w"
	UnitMonth       DurationUnit = "M"
	UnitYear        DurationUnit = "y"
)

// MsByUnit returns the number of milliseconds for a fixed-length duration unit.
// The second return value is false for calendar-aware units ("M", "y") and any
// unknown unit, mirroring the TypeScript msByUnit lookup table where those keys
// are absent.
//
// Example:
//
//	MsByUnit(UnitDay) // 86400000, true
//	MsByUnit(UnitMonth) // 0, false
func MsByUnit(unit DurationUnit) (int64, bool) {
	switch unit {
	case UnitMillisecond:
		return 1, true
	case UnitSecond:
		return consts.OneSecondMs, true
	case UnitMinute:
		return consts.OneMinuteMs, true
	case UnitHour:
		return consts.OneHourMs, true
	case UnitDay:
		return consts.OneDayMs, true
	case UnitWeek:
		return consts.OneWeekMs, true
	default:
		return 0, false
	}
}

// lastDayOfMonth returns the number of days in the given year and month, where
// month is zero-based to match JavaScript's getMonth conventions.
func lastDayOfMonth(year, monthZeroBased int) int {
	// time.Date normalizes day 0 of the next month to the last day of the
	// requested month, equivalent to new Date(year, month + 1, 0).getDate().
	t := time.Date(year, time.Month(monthZeroBased+2), 0, 0, 0, 0, 0, time.UTC)
	return t.Day()
}

// AddDuration adds a duration to a date and returns a new time.Time.
// Calendar-aware for UnitMonth and UnitYear, so end-of-month is preserved
// (e.g. Jan 31 + 1 month = Feb 28/29). The amount may be negative.
//
// Example:
//
//	AddDuration(time.Date(2025, 1, 31, 0, 0, 0, 0, time.UTC), 1, UnitMonth) // 2025-02-28
//	AddDuration(time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), 7, UnitDay) // 2025-01-08
func AddDuration(date time.Time, amount int, unit DurationUnit) time.Time {
	if ms, ok := MsByUnit(unit); ok {
		return time.UnixMilli(date.UnixMilli() + int64(amount)*ms).In(date.Location())
	}

	year := date.Year()
	monthZeroBased := int(date.Month()) - 1
	day := date.Day()
	hour, minute, second := date.Hour(), date.Minute(), date.Second()
	nsec := date.Nanosecond()
	loc := date.Location()

	if unit == UnitMonth {
		targetMonth := monthZeroBased + amount
		targetYear := year + floorDiv(targetMonth, 12)
		normalizedMonth := mod(targetMonth, 12)
		lastDay := lastDayOfMonth(targetYear, normalizedMonth)
		clampedDay := day
		if lastDay < clampedDay {
			clampedDay = lastDay
		}
		return time.Date(targetYear, time.Month(normalizedMonth+1), clampedDay, hour, minute, second, nsec, loc)
	}

	targetYear := year + amount
	lastDay := lastDayOfMonth(targetYear, monthZeroBased)
	clampedDay := day
	if lastDay < clampedDay {
		clampedDay = lastDay
	}
	return time.Date(targetYear, time.Month(monthZeroBased+1), clampedDay, hour, minute, second, nsec, loc)
}

// SubDuration subtracts a duration from a date and returns a new time.Time.
// Equivalent to AddDuration(date, -amount, unit).
//
// Example:
//
//	SubDuration(time.Date(2025, 3, 31, 0, 0, 0, 0, time.UTC), 1, UnitMonth) // 2025-02-28
func SubDuration(date time.Time, amount int, unit DurationUnit) time.Time {
	return AddDuration(date, -amount, unit)
}

// Diff returns the difference between two dates in the given unit, truncated
// toward zero. Calendar-aware for UnitMonth and UnitYear.
//
// Example:
//
//	Diff(time.Date(2025, 12, 31, 0, 0, 0, 0, time.UTC), time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), UnitDay) // 364
//	Diff(time.Date(2026, 1, 1, 0, 0, 0, 0, time.UTC), time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), UnitYear) // 1
func Diff(left, right time.Time, unit DurationUnit) int {
	if ms, ok := MsByUnit(unit); ok {
		return int(math.Trunc(float64(left.UnixMilli()-right.UnixMilli()) / float64(ms)))
	}

	yearsDelta := left.Year() - right.Year()
	monthsDelta := int(left.Month()) - int(right.Month())
	dayDelta := left.Day() - right.Day()

	subDayDelta := int64(left.Hour())*consts.OneHourMs +
		int64(left.Minute())*consts.OneMinuteMs +
		int64(left.Second())*consts.OneSecondMs +
		int64(left.Nanosecond()/1e6) -
		(int64(right.Hour())*consts.OneHourMs +
			int64(right.Minute())*consts.OneMinuteMs +
			int64(right.Second())*consts.OneSecondMs +
			int64(right.Nanosecond()/1e6))

	calendarMonths := yearsDelta*12 + monthsDelta
	if calendarMonths > 0 && (dayDelta < 0 || (dayDelta == 0 && subDayDelta < 0)) {
		calendarMonths--
	} else if calendarMonths < 0 && (dayDelta > 0 || (dayDelta == 0 && subDayDelta > 0)) {
		calendarMonths++
	}

	if unit == UnitMonth {
		return calendarMonths
	}
	return int(math.Trunc(float64(calendarMonths) / 12))
}

// floorDiv performs floored integer division, matching JavaScript's setMonth
// roll-over behavior for negative dividends.
func floorDiv(a, b int) int {
	q := a / b
	if (a%b != 0) && ((a < 0) != (b < 0)) {
		q--
	}
	return q
}

// mod returns a non-negative remainder for a positive divisor.
func mod(a, b int) int {
	r := a % b
	if r < 0 {
		r += b
	}
	return r
}
