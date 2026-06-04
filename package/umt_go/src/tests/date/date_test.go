package date_test

import (
	"testing"
	"time"

	"github.com/riya-amemiya/umt-go/src/date"
)

// --- IsLeapYear tests ---

func TestIsLeapYear(t *testing.T) {
	t.Run("divisible by 4 but not 100", func(t *testing.T) {
		leapYears := []int{2020, 2024, 1996, 2004, 2008}
		for _, y := range leapYears {
			if !date.IsLeapYear(y) {
				t.Errorf("%d should be a leap year", y)
			}
		}
	})

	t.Run("divisible by 100 but not 400", func(t *testing.T) {
		nonLeapYears := []int{1900, 2100, 1700, 1800, 2200}
		for _, y := range nonLeapYears {
			if date.IsLeapYear(y) {
				t.Errorf("%d should not be a leap year", y)
			}
		}
	})

	t.Run("divisible by 400", func(t *testing.T) {
		leapYears := []int{2000, 1600, 2400, 800, 1200}
		for _, y := range leapYears {
			if !date.IsLeapYear(y) {
				t.Errorf("%d should be a leap year", y)
			}
		}
	})

	t.Run("non-leap years", func(t *testing.T) {
		nonLeapYears := []int{2023, 2025, 1997, 2001, 2003}
		for _, y := range nonLeapYears {
			if date.IsLeapYear(y) {
				t.Errorf("%d should not be a leap year", y)
			}
		}
	})

	t.Run("early years", func(t *testing.T) {
		if !date.IsLeapYear(4) {
			t.Error("4 should be a leap year")
		}
		if date.IsLeapYear(100) {
			t.Error("100 should not be a leap year")
		}
		if !date.IsLeapYear(400) {
			t.Error("400 should be a leap year")
		}
		if !date.IsLeapYear(8) {
			t.Error("8 should be a leap year")
		}
		if date.IsLeapYear(1) {
			t.Error("1 should not be a leap year")
		}
	})

	t.Run("edge cases", func(t *testing.T) {
		if !date.IsLeapYear(0) {
			t.Error("0 should be a leap year")
		}
		if !date.IsLeapYear(-4) {
			t.Error("-4 should be a leap year")
		}
		if date.IsLeapYear(-100) {
			t.Error("-100 should not be a leap year")
		}
		if !date.IsLeapYear(-400) {
			t.Error("-400 should be a leap year")
		}
	})

	t.Run("very large years", func(t *testing.T) {
		if !date.IsLeapYear(4000) {
			t.Error("4000 should be a leap year")
		}
		if !date.IsLeapYear(8000) {
			t.Error("8000 should be a leap year")
		}
		if date.IsLeapYear(9999) {
			t.Error("9999 should not be a leap year")
		}
		if !date.IsLeapYear(10000) {
			t.Error("10000 should be a leap year")
		}
	})

	t.Run("verify leap year pattern", func(t *testing.T) {
		recentLeapYears := []int{1996, 2000, 2004, 2008, 2012, 2016, 2020, 2024}
		for _, y := range recentLeapYears {
			if !date.IsLeapYear(y) {
				t.Errorf("%d should be a leap year", y)
			}
		}

		recentNonLeapYears := []int{
			1997, 1998, 1999, 2001, 2002, 2003, 2005, 2006, 2007,
			2009, 2010, 2011, 2013, 2014, 2015, 2017, 2018, 2019,
			2021, 2022, 2023,
		}
		for _, y := range recentNonLeapYears {
			if date.IsLeapYear(y) {
				t.Errorf("%d should not be a leap year", y)
			}
		}
	})
}

// --- DayOfWeek tests ---

func TestDayOfWeek(t *testing.T) {
	t.Run("known dates", func(t *testing.T) {
		// January 1, 2020 was a Wednesday
		if got := date.DayOfWeek(2020, 1, 1); got != "Wednesday" {
			t.Errorf("DayOfWeek(2020, 1, 1) = %s, want Wednesday", got)
		}
		// January 2, 2020 was a Thursday
		if got := date.DayOfWeek(2020, 1, 2); got != "Thursday" {
			t.Errorf("DayOfWeek(2020, 1, 2) = %s, want Thursday", got)
		}
		// June 10, 2023 was a Saturday
		if got := date.DayOfWeek(2023, 6, 10); got != "Saturday" {
			t.Errorf("DayOfWeek(2023, 6, 10) = %s, want Saturday", got)
		}
	})
}

// --- Birthday tests ---

func TestBirthday(t *testing.T) {
	currentYear := time.Now().Year()

	t.Run("past birthday this year", func(t *testing.T) {
		// Someone born on January 1 of a past year
		age := date.Birthday(currentYear-25, 1, 1)
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
		age := date.Birthday(currentYear-25, 12, 31)
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
		age := date.Birthday(currentYear+10, 1, 1)
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
		dates := date.DateRange(start, end)

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
		d := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)
		dates := date.DateRange(d, d)

		if len(dates) != 1 {
			t.Errorf("expected 1 date, got %d", len(dates))
		}
		if !dates[0].Equal(d) {
			t.Errorf("date should be 2025-01-01, got %v", dates[0])
		}
	})

	t.Run("should handle month and year transitions", func(t *testing.T) {
		start := time.Date(2024, 12, 30, 0, 0, 0, 0, time.UTC)
		end := time.Date(2025, 1, 2, 0, 0, 0, 0, time.UTC)
		dates := date.DateRange(start, end)

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
		if got := date.GetDay(2020, 1, 1); got != 3 {
			t.Errorf("GetDay(2020, 1, 1) = %d, want 3", got)
		}
		// January 2, 2020 was a Thursday = 4
		if got := date.GetDay(2020, 1, 2); got != 4 {
			t.Errorf("GetDay(2020, 1, 2) = %d, want 4", got)
		}
	})
}

// --- GetDayName tests ---

func TestGetDayName(t *testing.T) {
	t.Run("Sunday in different languages", func(t *testing.T) {
		if got := date.GetDayName(0, "en"); got != "Sun" {
			t.Errorf("GetDayName(0, 'en') = %s, want Sun", got)
		}
		if got := date.GetDayName(0, "ja"); got != "日" {
			t.Errorf("GetDayName(0, 'ja') = %s, want 日", got)
		}
		if got := date.GetDayName(0, "ko"); got != "일" {
			t.Errorf("GetDayName(0, 'ko') = %s, want 일", got)
		}
		if got := date.GetDayName(0, "de"); got != "So" {
			t.Errorf("GetDayName(0, 'de') = %s, want So", got)
		}
		if got := date.GetDayName(0, "fr"); got != "Dim" {
			t.Errorf("GetDayName(0, 'fr') = %s, want Dim", got)
		}
	})

	t.Run("Wednesday in different languages", func(t *testing.T) {
		if got := date.GetDayName(3, "en"); got != "Wed" {
			t.Errorf("GetDayName(3, 'en') = %s, want Wed", got)
		}
		if got := date.GetDayName(3, "ja"); got != "水" {
			t.Errorf("GetDayName(3, 'ja') = %s, want 水", got)
		}
		if got := date.GetDayName(3, "ko"); got != "수" {
			t.Errorf("GetDayName(3, 'ko') = %s, want 수", got)
		}
		if got := date.GetDayName(3, "de"); got != "Mi" {
			t.Errorf("GetDayName(3, 'de') = %s, want Mi", got)
		}
		if got := date.GetDayName(3, "fr"); got != "Mer" {
			t.Errorf("GetDayName(3, 'fr') = %s, want Mer", got)
		}
	})

	t.Run("default is Japanese", func(t *testing.T) {
		japaneseDays := []string{"日", "月", "火", "水", "木", "金", "土"}
		for i, expected := range japaneseDays {
			if got := date.GetDayName(i, ""); got != expected {
				t.Errorf("GetDayName(%d, '') = %s, want %s", i, got, expected)
			}
		}
	})

	t.Run("invalid day numbers default to Sunday", func(t *testing.T) {
		if got := date.GetDayName(-1, ""); got != "日" {
			t.Errorf("GetDayName(-1, '') = %s, want 日", got)
		}
		if got := date.GetDayName(7, ""); got != "日" {
			t.Errorf("GetDayName(7, '') = %s, want 日", got)
		}
		if got := date.GetDayName(100, ""); got != "日" {
			t.Errorf("GetDayName(100, '') = %s, want 日", got)
		}
	})

	t.Run("all days in English", func(t *testing.T) {
		days := []string{"Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"}
		for i, expected := range days {
			if got := date.GetDayName(i, "en"); got != expected {
				t.Errorf("GetDayName(%d, 'en') = %s, want %s", i, got, expected)
			}
		}
	})
}

// --- FormatDate tests ---

func TestFormatDate(t *testing.T) {
	t.Run("basic date formats", func(t *testing.T) {
		d := time.Date(2023, 6, 10, 15, 30, 45, 123000000, time.UTC)
		if got := date.FormatDate(d, "YYYY-MM-DD"); got != "2023-06-10" {
			t.Errorf("FormatDate YYYY-MM-DD = %s, want 2023-06-10", got)
		}
		if got := date.FormatDate(d, "YYYY/MM/DD HH:mm:ss"); got != "2023/06/10 15:30:45" {
			t.Errorf("FormatDate = %s, want 2023/06/10 15:30:45", got)
		}
		if got := date.FormatDate(d, "YYYY-MM-DD HH:mm:ss.SSS"); got != "2023-06-10 15:30:45.123" {
			t.Errorf("FormatDate = %s, want 2023-06-10 15:30:45.123", got)
		}
	})

	t.Run("escaped characters", func(t *testing.T) {
		d := time.Date(2023, 6, 10, 15, 30, 45, 123000000, time.UTC)
		if got := date.FormatDate(d, "[Year:] YYYY [Month:] MM [Day:] DD"); got != "Year: 2023 Month: 06 Day: 10" {
			t.Errorf("FormatDate with escapes = %s, want 'Year: 2023 Month: 06 Day: 10'", got)
		}
	})

	t.Run("different formats", func(t *testing.T) {
		d := time.Date(2023, 6, 10, 15, 30, 45, 123000000, time.UTC)
		if got := date.FormatDate(d, "YY-M-D"); got != "23-6-10" {
			t.Errorf("FormatDate YY-M-D = %s, want 23-6-10", got)
		}
		if got := date.FormatDate(d, "hh:mm:ss A"); got != "03:30:45 PM" {
			t.Errorf("FormatDate hh:mm:ss A = %s, want 03:30:45 PM", got)
		}
		if got := date.FormatDate(d, "h:m:s a"); got != "3:30:45 pm" {
			t.Errorf("FormatDate h:m:s a = %s, want 3:30:45 pm", got)
		}
	})

	t.Run("day of week format", func(t *testing.T) {
		// June 10, 2023 was a Saturday (6)
		d := time.Date(2023, 6, 10, 0, 0, 0, 0, time.UTC)
		if got := date.FormatDate(d, "d"); got != "6" {
			t.Errorf("FormatDate d = %s, want 6", got)
		}
		if got := date.FormatDate(d, "YYYY-MM-DD (d)"); got != "2023-06-10 (6)" {
			t.Errorf("FormatDate YYYY-MM-DD (d) = %s, want 2023-06-10 (6)", got)
		}
	})

	t.Run("morning hours", func(t *testing.T) {
		d := time.Date(2023, 6, 10, 9, 5, 8, 4000000, time.UTC)
		if got := date.FormatDate(d, "HH:mm:ss"); got != "09:05:08" {
			t.Errorf("FormatDate HH:mm:ss = %s, want 09:05:08", got)
		}
		if got := date.FormatDate(d, "H:m:s"); got != "9:5:8" {
			t.Errorf("FormatDate H:m:s = %s, want 9:5:8", got)
		}
		if got := date.FormatDate(d, "hh:mm A"); got != "09:05 AM" {
			t.Errorf("FormatDate hh:mm A = %s, want 09:05 AM", got)
		}
		if got := date.FormatDate(d, "h:mm a"); got != "9:05 am" {
			t.Errorf("FormatDate h:mm a = %s, want 9:05 am", got)
		}
	})
}

// --- NewDate tests ---

func TestNewDate(t *testing.T) {
	t.Run("create date with year month day", func(t *testing.T) {
		d := date.NewDate(2025, 1, 1)
		if d.Year() != 2025 {
			t.Errorf("year = %d, want 2025", d.Year())
		}
		if d.Month() != time.January {
			t.Errorf("month = %v, want January", d.Month())
		}
		if d.Day() != 1 {
			t.Errorf("day = %d, want 1", d.Day())
		}
	})
}

// --- Now tests ---

func TestNow(t *testing.T) {
	t.Run("returns a time close to now", func(t *testing.T) {
		before := time.Now()
		result := date.Now()
		after := time.Now()

		if result.Before(before) || result.After(after) {
			t.Error("Now() should return current time")
		}
	})
}

// --- GetTimezoneOffsetString tests ---

func TestGetTimezoneOffsetString(t *testing.T) {
	t.Run("UTC timezone", func(t *testing.T) {
		utcTime := time.Date(2025, 1, 1, 12, 0, 0, 0, time.UTC)
		got := date.GetTimezoneOffsetString(utcTime)
		if got != "+00:00" {
			t.Errorf("GetTimezoneOffsetString(UTC) = %s, want +00:00", got)
		}
	})

	t.Run("positive offset (JST UTC+9)", func(t *testing.T) {
		jst := time.FixedZone("JST", 9*3600)
		jstTime := time.Date(2025, 1, 1, 12, 0, 0, 0, jst)
		got := date.GetTimezoneOffsetString(jstTime)
		if got != "+09:00" {
			t.Errorf("GetTimezoneOffsetString(JST) = %s, want +09:00", got)
		}
	})

	t.Run("negative offset (EST UTC-5)", func(t *testing.T) {
		est := time.FixedZone("EST", -5*3600)
		estTime := time.Date(2025, 1, 1, 12, 0, 0, 0, est)
		got := date.GetTimezoneOffsetString(estTime)
		if got != "-05:00" {
			t.Errorf("GetTimezoneOffsetString(EST) = %s, want -05:00", got)
		}
	})

	t.Run("offset with minutes (IST UTC+5:30)", func(t *testing.T) {
		ist := time.FixedZone("IST", 5*3600+30*60)
		istTime := time.Date(2025, 1, 1, 12, 0, 0, 0, ist)
		got := date.GetTimezoneOffsetString(istTime)
		if got != "+05:30" {
			t.Errorf("GetTimezoneOffsetString(IST) = %s, want +05:30", got)
		}
	})

	t.Run("negative offset with minutes (UTC-9:30)", func(t *testing.T) {
		zone := time.FixedZone("TEST", -9*3600-30*60)
		testTime := time.Date(2025, 1, 1, 12, 0, 0, 0, zone)
		got := date.GetTimezoneOffsetString(testTime)
		if got != "-09:30" {
			t.Errorf("GetTimezoneOffsetString(UTC-9:30) = %s, want -09:30", got)
		}
	})

	t.Run("large positive offset (UTC+12)", func(t *testing.T) {
		zone := time.FixedZone("TEST", 12*3600)
		testTime := time.Date(2025, 1, 1, 12, 0, 0, 0, zone)
		got := date.GetTimezoneOffsetString(testTime)
		if got != "+12:00" {
			t.Errorf("GetTimezoneOffsetString(UTC+12) = %s, want +12:00", got)
		}
	})
}

// --- MsByUnit tests ---

func TestMsByUnit(t *testing.T) {
	t.Run("fixed-length units return their millisecond length", func(t *testing.T) {
		cases := []struct {
			unit date.DurationUnit
			want int64
		}{
			{date.UnitMillisecond, 1},
			{date.UnitSecond, 1000},
			{date.UnitMinute, 60_000},
			{date.UnitHour, 3_600_000},
			{date.UnitDay, 86_400_000},
			{date.UnitWeek, 604_800_000},
		}
		for _, c := range cases {
			got, ok := date.MsByUnit(c.unit)
			if !ok || got != c.want {
				t.Errorf("MsByUnit(%q) = (%d, %v), want (%d, true)", c.unit, got, ok, c.want)
			}
		}
	})

	t.Run("calendar-aware and unknown units are absent", func(t *testing.T) {
		for _, u := range []date.DurationUnit{date.UnitMonth, date.UnitYear, date.DurationUnit("unknown")} {
			if got, ok := date.MsByUnit(u); ok {
				t.Errorf("MsByUnit(%q) = (%d, true), want (_, false)", u, got)
			}
		}
	})
}

// --- AddDuration tests ---

func TestAddDuration(t *testing.T) {
	base := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)

	t.Run("adds milliseconds", func(t *testing.T) {
		got := date.AddDuration(base, 500, date.UnitMillisecond)
		if got.UnixMilli() != base.UnixMilli()+500 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()+500)
		}
	})

	t.Run("adds seconds", func(t *testing.T) {
		got := date.AddDuration(base, 30, date.UnitSecond)
		if got.UnixMilli() != base.UnixMilli()+30_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()+30_000)
		}
	})

	t.Run("adds minutes", func(t *testing.T) {
		got := date.AddDuration(base, 5, date.UnitMinute)
		if got.UnixMilli() != base.UnixMilli()+5*60_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()+5*60_000)
		}
	})

	t.Run("adds hours", func(t *testing.T) {
		got := date.AddDuration(base, 2, date.UnitHour)
		if got.UnixMilli() != base.UnixMilli()+2*3_600_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()+2*3_600_000)
		}
	})

	t.Run("adds days", func(t *testing.T) {
		got := date.AddDuration(base, 7, date.UnitDay)
		if got.UnixMilli() != base.UnixMilli()+7*86_400_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()+7*86_400_000)
		}
	})

	t.Run("adds weeks", func(t *testing.T) {
		got := date.AddDuration(base, 2, date.UnitWeek)
		if got.UnixMilli() != base.UnixMilli()+14*86_400_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()+14*86_400_000)
		}
	})

	t.Run("adds months and clamps to last day of target month", func(t *testing.T) {
		got := date.AddDuration(time.Date(2025, 1, 31, 0, 0, 0, 0, time.UTC), 1, date.UnitMonth)
		if got.Year() != 2025 || got.Month() != time.February || got.Day() != 28 {
			t.Errorf("got %v, want 2025-02-28", got)
		}
	})

	t.Run("adds months across year boundary", func(t *testing.T) {
		got := date.AddDuration(time.Date(2025, 12, 15, 0, 0, 0, 0, time.UTC), 2, date.UnitMonth)
		if got.Year() != 2026 || got.Month() != time.February || got.Day() != 15 {
			t.Errorf("got %v, want 2026-02-15", got)
		}
	})

	t.Run("subtracts via negative amount", func(t *testing.T) {
		got := date.AddDuration(time.Date(2025, 3, 15, 0, 0, 0, 0, time.UTC), -1, date.UnitMonth)
		if got.Month() != time.February {
			t.Errorf("got month %v, want February", got.Month())
		}
	})

	t.Run("adds years preserving month and day with clamp", func(t *testing.T) {
		got := date.AddDuration(time.Date(2024, 2, 29, 0, 0, 0, 0, time.UTC), 1, date.UnitYear)
		if got.Year() != 2025 || got.Month() != time.February || got.Day() != 28 {
			t.Errorf("got %v, want 2025-02-28", got)
		}
	})

	t.Run("adds years across leap year boundary", func(t *testing.T) {
		got := date.AddDuration(time.Date(2024, 2, 29, 0, 0, 0, 0, time.UTC), 4, date.UnitYear)
		if got.Year() != 2028 || got.Month() != time.February || got.Day() != 29 {
			t.Errorf("got %v, want 2028-02-29", got)
		}
	})

	t.Run("does not mutate the input date", func(t *testing.T) {
		input := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)
		snapshot := input.UnixMilli()
		date.AddDuration(input, 5, date.UnitMonth)
		if input.UnixMilli() != snapshot {
			t.Errorf("input mutated: got %d, want %d", input.UnixMilli(), snapshot)
		}
	})
}

// --- SubDuration tests ---

func TestSubDuration(t *testing.T) {
	t.Run("subtracts milliseconds", func(t *testing.T) {
		base := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)
		got := date.SubDuration(base, 1000, date.UnitMillisecond)
		if got.UnixMilli() != base.UnixMilli()-1000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()-1000)
		}
	})

	t.Run("subtracts months and clamps to last day", func(t *testing.T) {
		got := date.SubDuration(time.Date(2025, 3, 31, 0, 0, 0, 0, time.UTC), 1, date.UnitMonth)
		if got.Month() != time.February || got.Day() != 28 {
			t.Errorf("got %v, want 2025-02-28", got)
		}
	})

	t.Run("subtracts years across leap year", func(t *testing.T) {
		got := date.SubDuration(time.Date(2025, 2, 28, 0, 0, 0, 0, time.UTC), 1, date.UnitYear)
		if got.Year() != 2024 || got.Month() != time.February || got.Day() != 28 {
			t.Errorf("got %v, want 2024-02-28", got)
		}
	})

	t.Run("subtracts seconds", func(t *testing.T) {
		base := time.Date(2025, 6, 1, 12, 0, 0, 0, time.UTC)
		got := date.SubDuration(base, 30, date.UnitSecond)
		if got.UnixMilli() != base.UnixMilli()-30_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()-30_000)
		}
	})

	t.Run("subtracts minutes", func(t *testing.T) {
		base := time.Date(2025, 6, 1, 12, 0, 0, 0, time.UTC)
		got := date.SubDuration(base, 5, date.UnitMinute)
		if got.UnixMilli() != base.UnixMilli()-300_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()-300_000)
		}
	})

	t.Run("subtracts hours", func(t *testing.T) {
		base := time.Date(2025, 6, 1, 12, 0, 0, 0, time.UTC)
		got := date.SubDuration(base, 1, date.UnitHour)
		if got.UnixMilli() != base.UnixMilli()-3_600_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()-3_600_000)
		}
	})

	t.Run("subtracts days", func(t *testing.T) {
		base := time.Date(2025, 6, 1, 12, 0, 0, 0, time.UTC)
		got := date.SubDuration(base, 1, date.UnitDay)
		if got.UnixMilli() != base.UnixMilli()-86_400_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()-86_400_000)
		}
	})

	t.Run("subtracts weeks", func(t *testing.T) {
		base := time.Date(2025, 6, 15, 12, 0, 0, 0, time.UTC)
		got := date.SubDuration(base, 1, date.UnitWeek)
		if got.UnixMilli() != base.UnixMilli()-7*86_400_000 {
			t.Errorf("got %d, want %d", got.UnixMilli(), base.UnixMilli()-7*86_400_000)
		}
	})
}

// --- Diff tests ---

func TestDiff(t *testing.T) {
	mustParse := func(s string) time.Time {
		parsed, err := time.Parse(time.RFC3339Nano, s)
		if err != nil {
			t.Fatalf("parse %q: %v", s, err)
		}
		return parsed
	}

	t.Run("computes day difference", func(t *testing.T) {
		got := date.Diff(time.Date(2025, 12, 31, 0, 0, 0, 0, time.UTC), time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), date.UnitDay)
		if got != 364 {
			t.Errorf("got %d, want 364", got)
		}
	})

	t.Run("computes hour difference", func(t *testing.T) {
		got := date.Diff(mustParse("2025-01-01T05:00:00Z"), mustParse("2025-01-01T00:00:00Z"), date.UnitHour)
		if got != 5 {
			t.Errorf("got %d, want 5", got)
		}
	})

	t.Run("computes minute difference", func(t *testing.T) {
		got := date.Diff(mustParse("2025-01-01T00:30:00Z"), mustParse("2025-01-01T00:00:00Z"), date.UnitMinute)
		if got != 30 {
			t.Errorf("got %d, want 30", got)
		}
	})

	t.Run("computes second difference", func(t *testing.T) {
		got := date.Diff(mustParse("2025-01-01T00:00:30Z"), mustParse("2025-01-01T00:00:00Z"), date.UnitSecond)
		if got != 30 {
			t.Errorf("got %d, want 30", got)
		}
	})

	t.Run("computes millisecond difference", func(t *testing.T) {
		got := date.Diff(mustParse("2025-01-01T00:00:00.500Z"), mustParse("2025-01-01T00:00:00Z"), date.UnitMillisecond)
		if got != 500 {
			t.Errorf("got %d, want 500", got)
		}
	})

	t.Run("computes week difference", func(t *testing.T) {
		got := date.Diff(time.Date(2025, 1, 15, 0, 0, 0, 0, time.UTC), time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), date.UnitWeek)
		if got != 2 {
			t.Errorf("got %d, want 2", got)
		}
	})

	t.Run("computes positive month difference adjusting for incomplete month", func(t *testing.T) {
		if got := date.Diff(time.Date(2025, 3, 14, 0, 0, 0, 0, time.UTC), time.Date(2025, 1, 15, 0, 0, 0, 0, time.UTC), date.UnitMonth); got != 1 {
			t.Errorf("got %d, want 1", got)
		}
		if got := date.Diff(time.Date(2025, 3, 16, 0, 0, 0, 0, time.UTC), time.Date(2025, 1, 15, 0, 0, 0, 0, time.UTC), date.UnitMonth); got != 2 {
			t.Errorf("got %d, want 2", got)
		}
	})

	t.Run("computes negative month difference adjusting for incomplete month", func(t *testing.T) {
		if got := date.Diff(time.Date(2025, 1, 15, 0, 0, 0, 0, time.UTC), time.Date(2025, 3, 14, 0, 0, 0, 0, time.UTC), date.UnitMonth); got != -1 {
			t.Errorf("got %d, want -1", got)
		}
		if got := date.Diff(time.Date(2025, 1, 15, 0, 0, 0, 0, time.UTC), time.Date(2025, 3, 16, 0, 0, 0, 0, time.UTC), date.UnitMonth); got != -2 {
			t.Errorf("got %d, want -2", got)
		}
	})

	t.Run("respects time-of-day for month boundary on positive diff", func(t *testing.T) {
		got := date.Diff(time.Date(2025, 2, 15, 10, 0, 0, 0, time.UTC), time.Date(2025, 1, 15, 11, 0, 0, 0, time.UTC), date.UnitMonth)
		if got != 0 {
			t.Errorf("got %d, want 0", got)
		}
	})

	t.Run("respects time-of-day for month boundary on negative diff", func(t *testing.T) {
		got := date.Diff(time.Date(2025, 1, 15, 11, 0, 0, 0, time.UTC), time.Date(2025, 2, 15, 10, 0, 0, 0, time.UTC), date.UnitMonth)
		if got != 0 {
			t.Errorf("got %d, want 0", got)
		}
	})

	t.Run("computes year difference", func(t *testing.T) {
		if got := date.Diff(time.Date(2026, 1, 1, 0, 0, 0, 0, time.UTC), time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), date.UnitYear); got != 1 {
			t.Errorf("got %d, want 1", got)
		}
		if got := date.Diff(time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), time.Date(2026, 1, 1, 0, 0, 0, 0, time.UTC), date.UnitYear); got != -1 {
			t.Errorf("got %d, want -1", got)
		}
	})

	t.Run("returns zero for identical dates", func(t *testing.T) {
		d := time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC)
		for _, u := range []date.DurationUnit{date.UnitDay, date.UnitMonth, date.UnitYear} {
			if got := date.Diff(d, d, u); got != 0 {
				t.Errorf("Diff(d, d, %q) = %d, want 0", u, got)
			}
		}
	})

	t.Run("truncates fractional fixed-unit differences toward zero", func(t *testing.T) {
		left := time.UnixMilli(1500)
		right := time.UnixMilli(0)
		if got := date.Diff(left, right, date.UnitSecond); got != 1 {
			t.Errorf("got %d, want 1", got)
		}
		if got := date.Diff(right, left, date.UnitSecond); got != -1 {
			t.Errorf("got %d, want -1", got)
		}
	})
}

// --- StartOf tests ---

func TestStartOf(t *testing.T) {
	t.Run("zeros milliseconds for second", func(t *testing.T) {
		got := date.StartOf(time.Date(2025, 4, 15, 10, 30, 45, 123*int(time.Millisecond), time.UTC), date.BoundarySecond)
		if got.Nanosecond() != 0 || got.Second() != 45 {
			t.Errorf("got %v, want second=45 ms=0", got)
		}
	})

	t.Run("zeros seconds for minute", func(t *testing.T) {
		got := date.StartOf(time.Date(2025, 4, 15, 10, 30, 45, 123*int(time.Millisecond), time.UTC), date.BoundaryMinute)
		if got.Second() != 0 || got.Nanosecond() != 0 {
			t.Errorf("got %v, want second=0 ms=0", got)
		}
	})

	t.Run("zeros minutes for hour", func(t *testing.T) {
		got := date.StartOf(time.Date(2025, 4, 15, 10, 30, 45, 123*int(time.Millisecond), time.UTC), date.BoundaryHour)
		if got.Minute() != 0 || got.Second() != 0 || got.Nanosecond() != 0 {
			t.Errorf("got %v, want minute=second=ms=0", got)
		}
	})

	t.Run("zeros hours for day", func(t *testing.T) {
		got := date.StartOf(time.Date(2025, 4, 15, 10, 30, 0, 0, time.UTC), date.BoundaryDay)
		if got.Hour() != 0 || got.Minute() != 0 || got.Second() != 0 || got.Nanosecond() != 0 {
			t.Errorf("got %v, want all zero time-of-day", got)
		}
	})

	t.Run("returns Sunday for week", func(t *testing.T) {
		got := date.StartOf(time.Date(2025, 4, 16, 0, 0, 0, 0, time.UTC), date.BoundaryWeek)
		if got.Weekday() != time.Sunday || got.Hour() != 0 {
			t.Errorf("got %v (weekday %v), want Sunday 00:00", got, got.Weekday())
		}
	})

	t.Run("returns first day for month", func(t *testing.T) {
		got := date.StartOf(time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC), date.BoundaryMonth)
		if got.Day() != 1 || got.Month() != time.April || got.Hour() != 0 {
			t.Errorf("got %v, want 2025-04-01 00:00", got)
		}
	})

	t.Run("returns first day of quarter", func(t *testing.T) {
		cases := []struct {
			in        time.Time
			wantMonth time.Month
		}{
			{time.Date(2025, 1, 31, 0, 0, 0, 0, time.UTC), time.January},
			{time.Date(2025, 5, 31, 0, 0, 0, 0, time.UTC), time.April},
			{time.Date(2025, 8, 31, 0, 0, 0, 0, time.UTC), time.July},
			{time.Date(2025, 11, 30, 0, 0, 0, 0, time.UTC), time.October},
		}
		for _, c := range cases {
			got := date.StartOf(c.in, date.BoundaryQuarter)
			if got.Month() != c.wantMonth || got.Day() != 1 {
				t.Errorf("StartOf(%v, quarter) = %v, want month %v day 1", c.in, got, c.wantMonth)
			}
		}
	})

	t.Run("returns Jan 1 for year", func(t *testing.T) {
		got := date.StartOf(time.Date(2025, 6, 15, 0, 0, 0, 0, time.UTC), date.BoundaryYear)
		if got.Month() != time.January || got.Day() != 1 || got.Hour() != 0 {
			t.Errorf("got %v, want 2025-01-01 00:00", got)
		}
	})

	t.Run("returns the date unchanged for an unknown unit", func(t *testing.T) {
		input := time.Date(2025, 4, 15, 10, 30, 0, 0, time.UTC)
		got := date.StartOf(input, date.DateBoundaryUnit("unknown"))
		if !got.Equal(input) {
			t.Errorf("got %v, want unchanged %v", got, input)
		}
	})
}

// --- EndOf tests ---

func TestEndOf(t *testing.T) {
	ms := func(tm time.Time) int { return tm.Nanosecond() / int(time.Millisecond) }

	t.Run("sets milliseconds to 999 for second", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 4, 15, 10, 30, 45, 0, time.UTC), date.BoundarySecond)
		if ms(got) != 999 || got.Second() != 45 {
			t.Errorf("got %v, want second=45 ms=999", got)
		}
	})

	t.Run("sets to 59.999 for minute", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 4, 15, 10, 30, 0, 0, time.UTC), date.BoundaryMinute)
		if got.Second() != 59 || ms(got) != 999 {
			t.Errorf("got %v, want second=59 ms=999", got)
		}
	})

	t.Run("sets to xx:59:59.999 for hour", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 4, 15, 10, 0, 0, 0, time.UTC), date.BoundaryHour)
		if got.Minute() != 59 || got.Second() != 59 || ms(got) != 999 {
			t.Errorf("got %v, want minute=59 second=59 ms=999", got)
		}
	})

	t.Run("sets to 23:59:59.999 for day", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC), date.BoundaryDay)
		if got.Hour() != 23 || got.Minute() != 59 || got.Second() != 59 || ms(got) != 999 {
			t.Errorf("got %v, want 23:59:59.999", got)
		}
	})

	t.Run("returns Saturday end for week", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 4, 16, 0, 0, 0, 0, time.UTC), date.BoundaryWeek)
		if got.Weekday() != time.Saturday || got.Hour() != 23 {
			t.Errorf("got %v (weekday %v), want Saturday 23:xx", got, got.Weekday())
		}
	})

	t.Run("returns last day for month", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 4, 1, 0, 0, 0, 0, time.UTC), date.BoundaryMonth)
		if got.Day() != 30 || got.Month() != time.April {
			t.Errorf("got %v, want April 30", got)
		}
	})

	t.Run("returns Feb 28 for non-leap February", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 2, 1, 0, 0, 0, 0, time.UTC), date.BoundaryMonth)
		if got.Day() != 28 {
			t.Errorf("got day %d, want 28", got.Day())
		}
	})

	t.Run("returns Feb 29 for leap February", func(t *testing.T) {
		got := date.EndOf(time.Date(2024, 2, 1, 0, 0, 0, 0, time.UTC), date.BoundaryMonth)
		if got.Day() != 29 {
			t.Errorf("got day %d, want 29", got.Day())
		}
	})

	t.Run("returns last day of each quarter", func(t *testing.T) {
		cases := []struct {
			in        time.Time
			wantMonth time.Month
			wantDay   int
		}{
			{time.Date(2025, 1, 1, 0, 0, 0, 0, time.UTC), time.March, 31},
			{time.Date(2025, 4, 1, 0, 0, 0, 0, time.UTC), time.June, 30},
			{time.Date(2025, 7, 1, 0, 0, 0, 0, time.UTC), time.September, 30},
			{time.Date(2025, 10, 1, 0, 0, 0, 0, time.UTC), time.December, 31},
		}
		for _, c := range cases {
			got := date.EndOf(c.in, date.BoundaryQuarter)
			if got.Month() != c.wantMonth || got.Day() != c.wantDay {
				t.Errorf("EndOf(%v, quarter) = %v, want %v %d", c.in, got, c.wantMonth, c.wantDay)
			}
		}
	})

	t.Run("returns Dec 31 for year", func(t *testing.T) {
		got := date.EndOf(time.Date(2025, 6, 15, 0, 0, 0, 0, time.UTC), date.BoundaryYear)
		if got.Month() != time.December || got.Day() != 31 || got.Hour() != 23 {
			t.Errorf("got %v, want Dec 31 23:xx", got)
		}
	})

	t.Run("returns the date unchanged for an unknown unit", func(t *testing.T) {
		input := time.Date(2025, 4, 15, 10, 30, 0, 0, time.UTC)
		got := date.EndOf(input, date.DateBoundaryUnit("unknown"))
		if !got.Equal(input) {
			t.Errorf("got %v, want unchanged %v", got, input)
		}
	})
}

// --- IsWeekend tests ---

func TestIsWeekend(t *testing.T) {
	cases := []struct {
		in   time.Time
		want bool
		name string
	}{
		{time.Date(2025, 4, 19, 0, 0, 0, 0, time.UTC), true, "Saturday"},
		{time.Date(2025, 4, 20, 0, 0, 0, 0, time.UTC), true, "Sunday"},
		{time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC), false, "Monday"},
		{time.Date(2025, 4, 18, 0, 0, 0, 0, time.UTC), false, "Friday"},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := date.IsWeekend(c.in); got != c.want {
				t.Errorf("IsWeekend(%s) = %v, want %v", c.name, got, c.want)
			}
		})
	}
}

// --- IsSameDay tests ---

func TestIsSameDay(t *testing.T) {
	t.Run("returns true for same date with different times", func(t *testing.T) {
		if !date.IsSameDay(time.Date(2025, 4, 15, 1, 0, 0, 0, time.UTC), time.Date(2025, 4, 15, 23, 59, 0, 0, time.UTC)) {
			t.Error("want true for same day")
		}
	})

	t.Run("returns false for adjacent days", func(t *testing.T) {
		if date.IsSameDay(time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC), time.Date(2025, 4, 16, 0, 0, 0, 0, time.UTC)) {
			t.Error("want false for adjacent days")
		}
	})

	t.Run("returns false for different month", func(t *testing.T) {
		if date.IsSameDay(time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC), time.Date(2025, 5, 15, 0, 0, 0, 0, time.UTC)) {
			t.Error("want false for different month")
		}
	})

	t.Run("returns false for different year", func(t *testing.T) {
		if date.IsSameDay(time.Date(2024, 4, 15, 0, 0, 0, 0, time.UTC), time.Date(2025, 4, 15, 0, 0, 0, 0, time.UTC)) {
			t.Error("want false for different year")
		}
	})
}

// --- IsBusinessDay tests ---

func TestIsBusinessDay(t *testing.T) {
	t.Run("returns true for Monday with no holidays", func(t *testing.T) {
		if !date.IsBusinessDay(time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC)) {
			t.Error("want true for Monday")
		}
	})

	t.Run("returns false for Saturday", func(t *testing.T) {
		if date.IsBusinessDay(time.Date(2025, 4, 19, 0, 0, 0, 0, time.UTC)) {
			t.Error("want false for Saturday")
		}
	})

	t.Run("returns false for Sunday", func(t *testing.T) {
		if date.IsBusinessDay(time.Date(2025, 4, 20, 0, 0, 0, 0, time.UTC)) {
			t.Error("want false for Sunday")
		}
	})

	t.Run("returns false for weekday listed as holiday", func(t *testing.T) {
		d := time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC)
		if date.IsBusinessDay(d, time.Date(2025, 4, 21, 12, 0, 0, 0, time.UTC)) {
			t.Error("want false for holiday weekday")
		}
	})

	t.Run("returns true for weekday not in holiday list", func(t *testing.T) {
		d := time.Date(2025, 4, 22, 0, 0, 0, 0, time.UTC)
		if !date.IsBusinessDay(d, time.Date(2025, 4, 21, 0, 0, 0, 0, time.UTC)) {
			t.Error("want true for non-holiday weekday")
		}
	})
}

// --- FormatRelative tests ---

func TestFormatRelative(t *testing.T) {
	base := time.Date(2025, 4, 15, 12, 0, 0, 0, time.UTC)

	cases := []struct {
		name   string
		target time.Time
		locale string
		want   string
	}{
		{"now for delta below one second", base.Add(500 * time.Millisecond), "en", "now"},
		{"seconds in the past", base.Add(-5 * time.Second), "en", "5 seconds ago"},
		{"minutes in the future", base.Add(5 * time.Minute), "en", "in 5 minutes"},
		{"hours", base.Add(-3 * time.Hour), "en", "3 hours ago"},
		{"days", base.Add(24 * time.Hour), "en", "tomorrow"},
		{"weeks", base.Add(-14 * 24 * time.Hour), "en", "2 weeks ago"},
		{"months", base.Add(90 * 24 * time.Hour), "en", "in 3 months"},
		{"years", base.Add(-2 * 365 * 24 * time.Hour), "en", "2 years ago"},
		{"japanese tomorrow", base.Add(24 * time.Hour), "ja", "明日"},
		{"japanese seconds ago", base.Add(-5 * time.Second), "ja", "5 秒前"},
		{"japanese minutes after", base.Add(5 * time.Minute), "ja", "5 分後"},
		{"japanese now", base.Add(500 * time.Millisecond), "ja", "今"},
	}

	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			if got := date.FormatRelative(c.target, base, c.locale); got != c.want {
				t.Errorf("FormatRelative = %q, want %q", got, c.want)
			}
		})
	}

	t.Run("falls back to en for empty locale", func(t *testing.T) {
		if got := date.FormatRelative(base.Add(-5*time.Second), base, ""); got != "5 seconds ago" {
			t.Errorf("got %q, want \"5 seconds ago\"", got)
		}
	})
}
