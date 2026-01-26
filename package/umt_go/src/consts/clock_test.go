package consts

import "testing"

// ---------------------------------------------------------------------------
// Basic time unit values
// ---------------------------------------------------------------------------

func TestOneSecondMs(t *testing.T) {
	if OneSecondMs != 1000 {
		t.Errorf("expected 1000, got %d", OneSecondMs)
	}
}

func TestOneMinuteMs(t *testing.T) {
	if OneMinuteMs != 60_000 {
		t.Errorf("expected 60000, got %d", OneMinuteMs)
	}
}

func TestOneHourMs(t *testing.T) {
	if OneHourMs != 3_600_000 {
		t.Errorf("expected 3600000, got %d", OneHourMs)
	}
}

func TestOneDayMs(t *testing.T) {
	if OneDayMs != 86_400_000 {
		t.Errorf("expected 86400000, got %d", OneDayMs)
	}
}

func TestOneWeekMs(t *testing.T) {
	if OneWeekMs != 604_800_000 {
		t.Errorf("expected 604800000, got %d", OneWeekMs)
	}
}

// ---------------------------------------------------------------------------
// Time unit relationships
// ---------------------------------------------------------------------------

func TestMinuteIsSecondTimes60(t *testing.T) {
	if OneMinuteMs != OneSecondMs*60 {
		t.Errorf("expected OneMinuteMs == OneSecondMs*60, got %d", OneMinuteMs)
	}
}

func TestHourIsMinuteTimes60(t *testing.T) {
	if OneHourMs != OneMinuteMs*60 {
		t.Errorf("expected OneHourMs == OneMinuteMs*60, got %d", OneHourMs)
	}
}

func TestDayIsHourTimes24(t *testing.T) {
	if OneDayMs != OneHourMs*24 {
		t.Errorf("expected OneDayMs == OneHourMs*24, got %d", OneDayMs)
	}
}

func TestWeekIsDayTimes7(t *testing.T) {
	if OneWeekMs != OneDayMs*7 {
		t.Errorf("expected OneWeekMs == OneDayMs*7, got %d", OneWeekMs)
	}
}

// ---------------------------------------------------------------------------
// Month variations
// ---------------------------------------------------------------------------

func TestOneMonthMs28(t *testing.T) {
	if OneMonthMs28 != 2_419_200_000 {
		t.Errorf("expected 2419200000, got %d", OneMonthMs28)
	}
	if OneMonthMs28 != OneDayMs*28 {
		t.Errorf("expected OneMonthMs28 == OneDayMs*28, got %d", OneMonthMs28)
	}
}

func TestOneMonthMs29(t *testing.T) {
	if OneMonthMs29 != 2_505_600_000 {
		t.Errorf("expected 2505600000, got %d", OneMonthMs29)
	}
	if OneMonthMs29 != OneDayMs*29 {
		t.Errorf("expected OneMonthMs29 == OneDayMs*29, got %d", OneMonthMs29)
	}
}

func TestOneMonthMs(t *testing.T) {
	if OneMonthMs != 2_592_000_000 {
		t.Errorf("expected 2592000000, got %d", OneMonthMs)
	}
	if OneMonthMs != OneDayMs*30 {
		t.Errorf("expected OneMonthMs == OneDayMs*30, got %d", OneMonthMs)
	}
}

func TestOneMonthMs31(t *testing.T) {
	if OneMonthMs31 != 2_678_400_000 {
		t.Errorf("expected 2678400000, got %d", OneMonthMs31)
	}
	if OneMonthMs31 != OneDayMs*31 {
		t.Errorf("expected OneMonthMs31 == OneDayMs*31, got %d", OneMonthMs31)
	}
}

func TestMonthLengthOrdering(t *testing.T) {
	if OneMonthMs28 >= OneMonthMs29 {
		t.Errorf("expected OneMonthMs28 < OneMonthMs29")
	}
	if OneMonthMs29 >= OneMonthMs {
		t.Errorf("expected OneMonthMs29 < OneMonthMs")
	}
	if OneMonthMs >= OneMonthMs31 {
		t.Errorf("expected OneMonthMs < OneMonthMs31")
	}
}

// ---------------------------------------------------------------------------
// Year variations
// ---------------------------------------------------------------------------

func TestOneYearMs(t *testing.T) {
	if OneYearMs != 31_536_000_000 {
		t.Errorf("expected 31536000000, got %d", OneYearMs)
	}
	if OneYearMs != OneDayMs*365 {
		t.Errorf("expected OneYearMs == OneDayMs*365, got %d", OneYearMs)
	}
}

func TestOneYearMs366(t *testing.T) {
	if OneYearMs366 != 31_622_400_000 {
		t.Errorf("expected 31622400000, got %d", OneYearMs366)
	}
	if OneYearMs366 != OneDayMs*366 {
		t.Errorf("expected OneYearMs366 == OneDayMs*366, got %d", OneYearMs366)
	}
}

func TestLeapYearIsOneDayLonger(t *testing.T) {
	if OneYearMs >= OneYearMs366 {
		t.Errorf("expected OneYearMs < OneYearMs366")
	}
	if OneYearMs366-OneYearMs != OneDayMs {
		t.Errorf("expected leap year difference to be one day (%d), got %d", OneDayMs, OneYearMs366-OneYearMs)
	}
}
