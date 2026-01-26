package date

import (
	"testing"
	"time"
)

// --- IsLeapYear tests ---

func TestIsLeapYear(t *testing.T) {
	t.Run("divisible by 4 but not 100", func(t *testing.T) {
		leapYears := []int{2020, 2024, 1996, 2004, 2008}
		for _, y := range leapYears {
			if !IsLeapYear(y) {
				t.Errorf("%d should be a leap year", y)
			}
		}
	})

	t.Run("divisible by 100 but not 400", func(t *testing.T) {
		nonLeapYears := []int{1900, 2100, 1700, 1800, 2200}
		for _, y := range nonLeapYears {
			if IsLeapYear(y) {
				t.Errorf("%d should not be a leap year", y)
			}
		}
	})

	t.Run("divisible by 400", func(t *testing.T) {
		leapYears := []int{2000, 1600, 2400, 800, 1200}
		for _, y := range leapYears {
			if !IsLeapYear(y) {
				t.Errorf("%d should be a leap year", y)
			}
		}
	})

	t.Run("non-leap years", func(t *testing.T) {
		nonLeapYears := []int{2023, 2025, 1997, 2001, 2003}
		for _, y := range nonLeapYears {
			if IsLeapYear(y) {
				t.Errorf("%d should not be a leap year", y)
			}
		}
	})

	t.Run("early years", func(t *testing.T) {
		if !IsLeapYear(4) {
			t.Error("4 should be a leap year")
		}
		if IsLeapYear(100) {
			t.Error("100 should not be a leap year")
		}
		if !IsLeapYear(400) {
			t.Error("400 should be a leap year")
		}
		if !IsLeapYear(8) {
			t.Error("8 should be a leap year")
		}
		if IsLeapYear(1) {
			t.Error("1 should not be a leap year")
		}
	})

	t.Run("edge cases", func(t *testing.T) {
		if !IsLeapYear(0) {
			t.Error("0 should be a leap year")
		}
		if !IsLeapYear(-4) {
			t.Error("-4 should be a leap year")
		}
		if IsLeapYear(-100) {
			t.Error("-100 should not be a leap year")
		}
		if !IsLeapYear(-400) {
			t.Error("-400 should be a leap year")
		}
	})

	t.Run("very large years", func(t *testing.T) {
		if !IsLeapYear(4000) {
			t.Error("4000 should be a leap year")
		}
		if !IsLeapYear(8000) {
			t.Error("8000 should be a leap year")
		}
		if IsLeapYear(9999) {
			t.Error("9999 should not be a leap year")
		}
		if !IsLeapYear(10000) {
			t.Error("10000 should be a leap year")
		}
	})

	t.Run("verify leap year pattern", func(t *testing.T) {
		recentLeapYears := []int{1996, 2000, 2004, 2008, 2012, 2016, 2020, 2024}
		for _, y := range recentLeapYears {
			if !IsLeapYear(y) {
				t.Errorf("%d should be a leap year", y)
			}
		}

		recentNonLeapYears := []int{
			1997, 1998, 1999, 2001, 2002, 2003, 2005, 2006, 2007,
			2009, 2010, 2011, 2013, 2014, 2015, 2017, 2018, 2019,
			2021, 2022, 2023,
		}
		for _, y := range recentNonLeapYears {
			if IsLeapYear(y) {
				t.Errorf("%d should not be a leap year", y)
			}
		}
	})
}

// --- DayOfWeek tests ---

func TestDayOfWeek(t *testing.T) {
	t.Run("known dates", func(t *testing.T) {
		// January 1, 2020 was a Wednesday
		if got := DayOfWeek(2020, 1, 1); got != "Wednesday" {
			t.Errorf("DayOfWeek(2020, 1, 1) = %s, want Wednesday", got)
		}
		// January 2, 2020 was a Thursday
		if got := DayOfWeek(2020, 1, 2); got != "Thursday" {
			t.Errorf("DayOfWeek(2020, 1, 2) = %s, want Thursday", got)
		}
		// June 10, 2023 was a Saturday
		if got := DayOfWeek(2023, 6, 10); got != "Saturday" {
			t.Errorf("DayOfWeek(2023, 6, 10) = %s, want Saturday", got)
		}
	})
}

// --- Birthday tests ---

func TestBirthday(t *testing.T) {
	currentYear := time.Now().Year()

	t.Run("past birthday this year", func(t *testing.T) {
		// Someone born on January 1 of a past year
		age := Birthday(currentYear-25, 1, 1)
		// Should be 25 if we're past Jan 1, or 24 if before
		now := time.Now()
		expected := 25
		thisYearBday := time.Date(currentYear, 1, 1, 0, 0, 0, 0, time.UTC)
		today := time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC)
		if today.Before(thisYearBday) {
			expected = 24
		}
		if age != expected {
			t.Errorf("Birthday(%d, 1, 1) = %d, want %d", currentYear-25, age, expected)
		}
	})

	t.Run("future birthday this year", func(t *testing.T) {
		// Someone born on December 31 of a past year
		age := Birthday(currentYear-25, 12, 31)
		now := time.Now()
		expected := 24
		thisYearBday := time.Date(currentYear, 12, 31, 0, 0, 0, 0, time.UTC)
		today := time.Date(now.Year(), now.Month(), now.Day(), 0, 0, 0, 0, time.UTC)
		if !today.Before(thisYearBday) {
			expected = 25
		}
		if age != expected {
			t.Errorf("Birthday(%d, 12, 31) = %d, want %d", currentYear-25, age, expected)
		}
	})

	t.Run("future birth year returns 0", func(t *testing.T) {
		age := Birthday(currentYear+10, 1, 1)
		if age != 0 {
			t.Errorf("Birthday(%d, 1, 1) = %d, want 0", currentYear+10, age)
		}
	})
}

// --- DateRange tests ---

func TestDateRange(t *testing.T) {
	t.Run("should generate array of dates between start and end", func(t *testing.T) {
		start := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)
		end := time.Date(2025, 1, 3, 0, 0, 0, 0, time.UTC)
		dates := DateRange(start, end)

		if len(dates) != 3 {
			t.Errorf("expected 3 dates, got %d", len(dates))
		}
		if !dates[0].Equal(time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)) {
			t.Errorf("first date should be 2025-01-01, got %v", dates[0])
		}
		if !dates[1].Equal(time.Date(2025, 1, 2, 0, 0, 0, 0, time.UTC)) {
			t.Errorf("second date should be 2025-01-02, got %v", dates[1])
		}
		if !dates[2].Equal(time.Date(2025, 1, 3, 0, 0, 0, 0, time.UTC)) {
			t.Errorf("third date should be 2025-01-03, got %v", dates[2])
		}
	})

	t.Run("should handle single day range", func(t *testing.T) {
		date := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)
		dates := DateRange(date, date)

		if len(dates) != 1 {
			t.Errorf("expected 1 date, got %d", len(dates))
		}
		if !dates[0].Equal(date) {
			t.Errorf("date should be 2025-01-01, got %v", dates[0])
		}
	})

	t.Run("should handle month and year transitions", func(t *testing.T) {
		start := time.Date(2024, 12, 30, 0, 0, 0, 0, time.UTC)
		end := time.Date(2025, 1, 2, 0, 0, 0, 0, time.UTC)
		dates := DateRange(start, end)

		if len(dates) != 4 {
			t.Errorf("expected 4 dates, got %d", len(dates))
		}
		expectedDates := []time.Time{
			time.Date(2024, 12, 30, 0, 0, 0, 0, time.UTC),
			time.Date(2024, 12, 31, 0, 0, 0, 0, time.UTC),
			time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC),
			time.Date(2025, 1, 2, 0, 0, 0, 0, time.UTC),
		}
		for i, expected := range expectedDates {
			if !dates[i].Equal(expected) {
				t.Errorf("date[%d] should be %v, got %v", i, expected, dates[i])
			}
		}
	})
}

// --- GetDay tests ---

func TestGetDay(t *testing.T) {
	t.Run("known dates", func(t *testing.T) {
		// January 1, 2020 was a Wednesday = 3
		if got := GetDay(2020, 1, 1); got != 3 {
			t.Errorf("GetDay(2020, 1, 1) = %d, want 3", got)
		}
		// January 2, 2020 was a Thursday = 4
		if got := GetDay(2020, 1, 2); got != 4 {
			t.Errorf("GetDay(2020, 1, 2) = %d, want 4", got)
		}
	})
}

// --- GetDayName tests ---

func TestGetDayName(t *testing.T) {
	t.Run("Sunday in different languages", func(t *testing.T) {
		if got := GetDayName(0, "en"); got != "Sun" {
			t.Errorf("GetDayName(0, 'en') = %s, want Sun", got)
		}
		if got := GetDayName(0, "ja"); got != "日" {
			t.Errorf("GetDayName(0, 'ja') = %s, want 日", got)
		}
		if got := GetDayName(0, "ko"); got != "일" {
			t.Errorf("GetDayName(0, 'ko') = %s, want 일", got)
		}
		if got := GetDayName(0, "de"); got != "So" {
			t.Errorf("GetDayName(0, 'de') = %s, want So", got)
		}
		if got := GetDayName(0, "fr"); got != "Dim" {
			t.Errorf("GetDayName(0, 'fr') = %s, want Dim", got)
		}
	})

	t.Run("Wednesday in different languages", func(t *testing.T) {
		if got := GetDayName(3, "en"); got != "Wed" {
			t.Errorf("GetDayName(3, 'en') = %s, want Wed", got)
		}
		if got := GetDayName(3, "ja"); got != "水" {
			t.Errorf("GetDayName(3, 'ja') = %s, want 水", got)
		}
		if got := GetDayName(3, "ko"); got != "수" {
			t.Errorf("GetDayName(3, 'ko') = %s, want 수", got)
		}
		if got := GetDayName(3, "de"); got != "Mi" {
			t.Errorf("GetDayName(3, 'de') = %s, want Mi", got)
		}
		if got := GetDayName(3, "fr"); got != "Mer" {
			t.Errorf("GetDayName(3, 'fr') = %s, want Mer", got)
		}
	})

	t.Run("default is Japanese", func(t *testing.T) {
		japaneseDays := []string{"日", "月", "火", "水", "木", "金", "土"}
		for i, expected := range japaneseDays {
			if got := GetDayName(i, ""); got != expected {
				t.Errorf("GetDayName(%d, '') = %s, want %s", i, got, expected)
			}
		}
	})

	t.Run("invalid day numbers default to Sunday", func(t *testing.T) {
		if got := GetDayName(-1, ""); got != "日" {
			t.Errorf("GetDayName(-1, '') = %s, want 日", got)
		}
		if got := GetDayName(7, ""); got != "日" {
			t.Errorf("GetDayName(7, '') = %s, want 日", got)
		}
		if got := GetDayName(100, ""); got != "日" {
			t.Errorf("GetDayName(100, '') = %s, want 日", got)
		}
	})

	t.Run("all days in English", func(t *testing.T) {
		days := []string{"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"}
		for i, expected := range days {
			if got := GetDayName(i, "en"); got != expected {
				t.Errorf("GetDayName(%d, 'en') = %s, want %s", i, got, expected)
			}
		}
	})
}

// --- FormatDate tests ---

func TestFormatDate(t *testing.T) {
	t.Run("basic date formats", func(t *testing.T) {
		date := time.Date(2023, 6, 10, 15, 30, 45, 123000000, time.UTC)
		if got := FormatDate(date, "YYYY-MM-DD"); got != "2023-06-10" {
			t.Errorf("FormatDate YYYY-MM-DD = %s, want 2023-06-10", got)
		}
		if got := FormatDate(date, "YYYY/MM/DD HH:mm:ss"); got != "2023/06/10 15:30:45" {
			t.Errorf("FormatDate = %s, want 2023/06/10 15:30:45", got)
		}
		if got := FormatDate(date, "YYYY-MM-DD HH:mm:ss.SSS"); got != "2023-06-10 15:30:45.123" {
			t.Errorf("FormatDate = %s, want 2023-06-10 15:30:45.123", got)
		}
	})

	t.Run("escaped characters", func(t *testing.T) {
		date := time.Date(2023, 6, 10, 15, 30, 45, 123000000, time.UTC)
		if got := FormatDate(date, "[Year:] YYYY [Month:] MM [Day:] DD"); got != "Year: 2023 Month: 06 Day: 10" {
			t.Errorf("FormatDate with escapes = %s, want 'Year: 2023 Month: 06 Day: 10'", got)
		}
	})

	t.Run("different formats", func(t *testing.T) {
		date := time.Date(2023, 6, 10, 15, 30, 45, 123000000, time.UTC)
		if got := FormatDate(date, "YY-M-D"); got != "23-6-10" {
			t.Errorf("FormatDate YY-M-D = %s, want 23-6-10", got)
		}
		if got := FormatDate(date, "hh:mm:ss A"); got != "03:30:45 PM" {
			t.Errorf("FormatDate hh:mm:ss A = %s, want 03:30:45 PM", got)
		}
		if got := FormatDate(date, "h:m:s a"); got != "3:30:45 pm" {
			t.Errorf("FormatDate h:m:s a = %s, want 3:30:45 pm", got)
		}
	})

	t.Run("day of week format", func(t *testing.T) {
		// June 10, 2023 was a Saturday (6)
		date := time.Date(2023, 6, 10, 0, 0, 0, 0, time.UTC)
		if got := FormatDate(date, "d"); got != "6" {
			t.Errorf("FormatDate d = %s, want 6", got)
		}
		if got := FormatDate(date, "YYYY-MM-DD (d)"); got != "2023-06-10 (6)" {
			t.Errorf("FormatDate YYYY-MM-DD (d) = %s, want 2023-06-10 (6)", got)
		}
	})

	t.Run("morning hours", func(t *testing.T) {
		date := time.Date(2023, 6, 10, 9, 5, 8, 4000000, time.UTC)
		if got := FormatDate(date, "HH:mm:ss"); got != "09:05:08" {
			t.Errorf("FormatDate HH:mm:ss = %s, want 09:05:08", got)
		}
		if got := FormatDate(date, "H:m:s"); got != "9:5:8" {
			t.Errorf("FormatDate H:m:s = %s, want 9:5:8", got)
		}
		if got := FormatDate(date, "hh:mm A"); got != "09:05 AM" {
			t.Errorf("FormatDate hh:mm A = %s, want 09:05 AM", got)
		}
		if got := FormatDate(date, "h:mm a"); got != "9:05 am" {
			t.Errorf("FormatDate h:mm a = %s, want 9:05 am", got)
		}
	})
}

// --- NewDate tests ---

func TestNewDate(t *testing.T) {
	t.Run("create date with year month day", func(t *testing.T) {
		date := NewDate(2025, 1, 1)
		if date.Year() != 2025 {
			t.Errorf("year = %d, want 2025", date.Year())
		}
		if date.Month() != time.January {
			t.Errorf("month = %v, want January", date.Month())
		}
		if date.Day() != 1 {
			t.Errorf("day = %d, want 1", date.Day())
		}
	})
}

// --- Now tests ---

func TestNow(t *testing.T) {
	t.Run("returns a time close to now", func(t *testing.T) {
		before := time.Now()
		result := Now()
		after := time.Now()

		if result.Before(before) || result.After(after) {
			t.Error("Now() should return current time")
		}
	})
}
