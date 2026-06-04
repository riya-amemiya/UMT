package date

import "time"

// StartOf returns a new time.Time set to the start of the given unit.
// Week start is Sunday. An unknown unit returns the date unchanged.
//
// Example:
//
//	StartOf(time.Date(2025, 4, 15, 10, 30, 0, 0, time.UTC), BoundaryDay) // 2025-04-15T00:00:00
//	StartOf(time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC), BoundaryMonth) // 2025-04-01T00:00:00
func StartOf(date time.Time, unit DateBoundaryUnit) time.Time {
	year, month, day := date.Year(), int(date.Month()), date.Day()
	hour, minute, second := date.Hour(), date.Minute(), date.Second()
	loc := date.Location()

	switch unit {
	case BoundarySecond:
		return time.Date(year, time.Month(month), day, hour, minute, second, 0, loc)
	case BoundaryMinute:
		return time.Date(year, time.Month(month), day, hour, minute, 0, 0, loc)
	case BoundaryHour:
		return time.Date(year, time.Month(month), day, hour, 0, 0, 0, loc)
	case BoundaryDay:
		return time.Date(year, time.Month(month), day, 0, 0, 0, 0, loc)
	case BoundaryWeek:
		dayOfWeek := int(date.Weekday())
		return time.Date(year, time.Month(month), day-dayOfWeek, 0, 0, 0, 0, loc)
	case BoundaryMonth:
		return time.Date(year, time.Month(month), 1, 0, 0, 0, 0, loc)
	case BoundaryQuarter:
		quarterStartMonth := ((month-1)/3)*3 + 1
		return time.Date(year, time.Month(quarterStartMonth), 1, 0, 0, 0, 0, loc)
	case BoundaryYear:
		return time.Date(year, time.January, 1, 0, 0, 0, 0, loc)
	default:
		return date
	}
}

// EndOf returns a new time.Time set to the end of the given unit.
// Week ends on Saturday at 23:59:59.999. An unknown unit returns the date
// unchanged. Millisecond precision matches the TypeScript behavior (.999).
//
// Example:
//
//	EndOf(time.Date(2025, 4, 15, 10, 30, 0, 0, time.UTC), BoundaryDay) // 2025-04-15T23:59:59.999
//	EndOf(time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC), BoundaryMonth) // 2025-04-30T23:59:59.999
func EndOf(date time.Time, unit DateBoundaryUnit) time.Time {
	year, month, day := date.Year(), int(date.Month()), date.Day()
	hour, minute, second := date.Hour(), date.Minute(), date.Second()
	loc := date.Location()
	const ms999 = 999 * int(time.Millisecond)

	switch unit {
	case BoundarySecond:
		return time.Date(year, time.Month(month), day, hour, minute, second, ms999, loc)
	case BoundaryMinute:
		return time.Date(year, time.Month(month), day, hour, minute, 59, ms999, loc)
	case BoundaryHour:
		return time.Date(year, time.Month(month), day, hour, 59, 59, ms999, loc)
	case BoundaryDay:
		return time.Date(year, time.Month(month), day, 23, 59, 59, ms999, loc)
	case BoundaryWeek:
		dayOfWeek := int(date.Weekday())
		return time.Date(year, time.Month(month), day+(6-dayOfWeek), 23, 59, 59, ms999, loc)
	case BoundaryMonth:
		// Day 0 of next month resolves to the last day of this month.
		return time.Date(year, time.Month(month+1), 0, 23, 59, 59, ms999, loc)
	case BoundaryQuarter:
		quarterEndMonth := ((month-1)/3)*3 + 3
		return time.Date(year, time.Month(quarterEndMonth+1), 0, 23, 59, 59, ms999, loc)
	case BoundaryYear:
		return time.Date(year, time.December, 31, 23, 59, 59, ms999, loc)
	default:
		return date
	}
}
