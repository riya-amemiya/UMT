package date

import "time"

// IsWeekend returns true when the given date is Saturday or Sunday in its own
// location.
//
// Example:
//
//	IsWeekend(time.Date(2025, 4, 19, 0, 0, 0, 0, time.UTC)) // true (Saturday)
//	IsWeekend(time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC)) // false (Monday)
func IsWeekend(date time.Time) bool {
	dayOfWeek := date.Weekday()
	return dayOfWeek == time.Sunday || dayOfWeek == time.Saturday
}

// IsSameDay returns true when two dates represent the same calendar day in
// their own locations.
//
// Example:
//
//	IsSameDay(time.Date(2025, 4, 15, 1, 0, 0, 0, time.UTC), time.Date(2025, 4, 15, 23, 0, 0, 0, time.UTC)) // true
//	IsSameDay(time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC), time.Date(2025, 4, 16, 0, 0, 0, 0, time.UTC)) // false
func IsSameDay(left, right time.Time) bool {
	ly, lm, ld := left.Date()
	ry, rm, rd := right.Date()
	return ly == ry && lm == rm && ld == rd
}

// IsBusinessDay returns true when the given date is a weekday and not present
// in the holidays list. Holidays are compared by calendar day.
//
// Example:
//
//	IsBusinessDay(time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC)) // true (Monday)
//	IsBusinessDay(time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC), time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC)) // false
func IsBusinessDay(date time.Time, holidays ...time.Time) bool {
	if IsWeekend(date) {
		return false
	}
	for _, holiday := range holidays {
		if IsSameDay(holiday, date) {
			return false
		}
	}
	return true
}
