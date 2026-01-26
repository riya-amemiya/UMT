package date

import (
	"fmt"
	"strings"
	"time"
)

// IsLeapYear determines if a given year is a leap year.
func IsLeapYear(year int) bool {
	return (year%4 == 0 && year%100 != 0) || year%400 == 0
}

// DayOfWeek returns the name of the day of the week for a given date.
// Returns the full English name, e.g. "Monday", "Tuesday", etc.
func DayOfWeek(year, month, day int) string {
	t := time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC)
	return t.Weekday().String()
}

// Birthday calculates the age based on a birth date.
// Uses the current time to determine age.
func Birthday(year, month, day int) int {
	now := time.Now()
	birthDate := time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC)

	currentYear := now.Year()
	birthYear := birthDate.Year()

	age := currentYear - birthYear

	// Check if birthday hasn't occurred yet this year
	thisYearBirthday := time.Date(currentYear, birthDate.Month(), birthDate.Day(), 0, 0, 0, 0, time.UTC)
	today := time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC)

	if today.Before(thisYearBirthday) {
		age--
	}

	if age < 0 {
		return 0
	}

	return age
}

// DateRange generates a slice of dates from start to end (inclusive), with one-day steps.
func DateRange(start, end time.Time) []time.Time {
	// Normalize to start of day
	start = time.Date(start.Year(), start.Month(), start.Day(), 0, 0, 0, 0, start.Location())
	end = time.Date(end.Year(), end.Month(), end.Day(), 0, 0, 0, 0, end.Location())

	var dates []time.Time
	current := start
	for !current.After(end) {
		dates = append(dates, current)
		current = current.AddDate(0, 0, 1)
	}
	return dates
}

// GetDay returns the day of the week as a number (0=Sunday, 6=Saturday).
func GetDay(year, month, day int) int {
	t := time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC)
	return int(t.Weekday())
}

// GetDayName converts a day number (0=Sunday) to a day name in the specified language.
// Supported languages: "en", "ja", "ko", "de", "fr". Default is "ja".
func GetDayName(dayNum int, lang string) string {
	dayList := map[string][]string{
		"de": {"So", "Mo", "Di", "Mi", "Do", "Fr", "Sa"},
		"ko": {"일", "월", "화", "수", "목", "금", "토"},
		"en": {"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"},
		"ja": {"日", "月", "火", "水", "木", "金", "土"},
		"fr": {"Dim", "Lun", "Mar", "Mer", "Jeu", "Ven", "Sam"},
	}

	if lang == "" {
		lang = "ja"
	}

	days, ok := dayList[lang]
	if !ok {
		days = dayList["ja"]
	}

	if dayNum >= 0 && dayNum <= 6 {
		return days[dayNum]
	}
	// Default to Sunday for invalid day numbers
	return days[0]
}

// FormatDate formats a time.Time value using a custom format string.
// Supported tokens:
//   - YYYY: Full year (e.g., 2025)
//   - YY: Short year (e.g., 25)
//   - MM: Month with leading zero (01-12)
//   - M: Month without leading zero (1-12)
//   - DD: Day with leading zero (01-31)
//   - D: Day without leading zero (1-31)
//   - d: Day of week (0-6)
//   - HH: Hours with leading zero (00-23)
//   - H: Hours without leading zero (0-23)
//   - hh: Hours (12-hour) with leading zero (01-12)
//   - h: Hours (12-hour) without leading zero (1-12)
//   - mm: Minutes with leading zero (00-59)
//   - m: Minutes without leading zero (0-59)
//   - ss: Seconds with leading zero (00-59)
//   - s: Seconds without leading zero (0-59)
//   - SSS: Milliseconds with leading zeros (000-999)
//   - A: AM/PM
//   - a: am/pm
func FormatDate(t time.Time, format string) string {
	hours := t.Hour()
	year := t.Year()
	month := int(t.Month())
	day := t.Day()
	minute := t.Minute()
	second := t.Second()
	ms := t.Nanosecond() / 1e6
	weekday := int(t.Weekday())

	ampm := "AM"
	if hours >= 12 {
		ampm = "PM"
	}

	h12 := hours % 12
	if h12 == 0 {
		h12 = 12
	}

	matches := map[string]string{
		"YYYY": fmt.Sprintf("%04d", year),
		"YY":   fmt.Sprintf("%02d", year%100),
		"MM":   fmt.Sprintf("%02d", month),
		"M":    fmt.Sprintf("%d", month),
		"DD":   fmt.Sprintf("%02d", day),
		"D":    fmt.Sprintf("%d", day),
		"d":    fmt.Sprintf("%d", weekday),
		"HH":   fmt.Sprintf("%02d", hours),
		"H":    fmt.Sprintf("%d", hours),
		"hh":   fmt.Sprintf("%02d", h12),
		"h":    fmt.Sprintf("%d", h12),
		"mm":   fmt.Sprintf("%02d", minute),
		"m":    fmt.Sprintf("%d", minute),
		"ss":   fmt.Sprintf("%02d", second),
		"s":    fmt.Sprintf("%d", second),
		"SSS":  fmt.Sprintf("%03d", ms),
		"A":    ampm,
		"a":    strings.ToLower(ampm),
	}

	result := []byte{}
	i := 0
	fmtBytes := []byte(format)

	for i < len(fmtBytes) {
		// Handle escaped characters [...]
		if fmtBytes[i] == '[' {
			end := i + 1
			for end < len(fmtBytes) && fmtBytes[end] != ']' {
				end++
			}
			if end < len(fmtBytes) {
				result = append(result, fmtBytes[i+1:end]...)
				i = end + 1
				continue
			}
		}

		// Try to match tokens in order of decreasing length
		matched := false
		// Order matters: longer tokens must be tried first
		tokens := []string{
			"YYYY", "SSS", "YY", "MM", "DD", "HH", "hh", "mm", "ss", "M", "D", "d", "H", "h", "m", "s", "A", "a",
		}

		for _, token := range tokens {
			if i+len(token) <= len(fmtBytes) && string(fmtBytes[i:i+len(token)]) == token {
				result = append(result, []byte(matches[token])...)
				i += len(token)
				matched = true
				break
			}
		}

		if !matched {
			result = append(result, fmtBytes[i])
			i++
		}
	}

	return string(result)
}

// Now returns the current time.
func Now() time.Time {
	return time.Now()
}

// NewDate creates a new time.Time for the given year, month, and day in UTC.
func NewDate(year, month, day int) time.Time {
	return time.Date(year, time.Month(month), day, 0, 0, 0, 0, time.UTC)
}
