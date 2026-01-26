package timeutil

import (
	"testing"
)

// --- NormalizeTimeUnit tests ---

func TestNormalizeTimeUnit(t *testing.T) {
	t.Run("converts long format to long format", func(t *testing.T) {
		cases := map[string]string{
			"milliseconds": "milliseconds",
			"seconds":      "seconds",
			"minutes":      "minutes",
			"hours":        "hours",
		}
		for input, expected := range cases {
			if got := NormalizeTimeUnit(input, "long"); got != expected {
				t.Errorf("NormalizeTimeUnit(%q, 'long') = %q, want %q", input, got, expected)
			}
		}
	})

	t.Run("converts long format to short format", func(t *testing.T) {
		cases := map[string]string{
			"milliseconds": "ms",
			"seconds":      "s",
			"minutes":      "m",
			"hours":        "h",
		}
		for input, expected := range cases {
			if got := NormalizeTimeUnit(input, "short"); got != expected {
				t.Errorf("NormalizeTimeUnit(%q, 'short') = %q, want %q", input, got, expected)
			}
		}
	})

	t.Run("converts short format to long format", func(t *testing.T) {
		cases := map[string]string{
			"ms": "milliseconds",
			"s":  "seconds",
			"m":  "minutes",
			"h":  "hours",
		}
		for input, expected := range cases {
			if got := NormalizeTimeUnit(input, "long"); got != expected {
				t.Errorf("NormalizeTimeUnit(%q, 'long') = %q, want %q", input, got, expected)
			}
		}
	})

	t.Run("converts short format to short format", func(t *testing.T) {
		cases := map[string]string{
			"ms": "ms",
			"s":  "s",
			"m":  "m",
			"h":  "h",
		}
		for input, expected := range cases {
			if got := NormalizeTimeUnit(input, "short"); got != expected {
				t.Errorf("NormalizeTimeUnit(%q, 'short') = %q, want %q", input, got, expected)
			}
		}
	})

	t.Run("handles all supported units comprehensively", func(t *testing.T) {
		longUnits := []string{"milliseconds", "seconds", "minutes", "hours"}
		shortUnits := []string{"ms", "s", "m", "h"}

		for i, unit := range longUnits {
			if got := NormalizeTimeUnit(unit, "short"); got != shortUnits[i] {
				t.Errorf("NormalizeTimeUnit(%q, 'short') = %q, want %q", unit, got, shortUnits[i])
			}
			if got := NormalizeTimeUnit(unit, "long"); got != unit {
				t.Errorf("NormalizeTimeUnit(%q, 'long') = %q, want %q", unit, got, unit)
			}
		}

		for i, unit := range shortUnits {
			if got := NormalizeTimeUnit(unit, "long"); got != longUnits[i] {
				t.Errorf("NormalizeTimeUnit(%q, 'long') = %q, want %q", unit, got, longUnits[i])
			}
			if got := NormalizeTimeUnit(unit, "short"); got != unit {
				t.Errorf("NormalizeTimeUnit(%q, 'short') = %q, want %q", unit, got, unit)
			}
		}
	})
}

// --- ConvertTime tests ---

func TestConvertTime(t *testing.T) {
	// Long format to long format
	t.Run("long to long", func(t *testing.T) {
		cases := []struct {
			name     string
			value    float64
			from     string
			to       string
			expected float64
		}{
			{"1 hour to seconds", 1, "hours", "seconds", 3600},
			{"3600 seconds to hours", 3600, "seconds", "hours", 1},
			{"90 minutes to hours", 90, "minutes", "hours", 1.5},
			{"1 hour to milliseconds", 1, "hours", "milliseconds", 3600000},
			{"0.5 seconds to milliseconds", 0.5, "seconds", "milliseconds", 500},
			{"1000 milliseconds to seconds", 1000, "milliseconds", "seconds", 1},
			{"same units (10 seconds)", 10, "seconds", "seconds", 10},
			{"1.5 hours to minutes", 1.5, "hours", "minutes", 90},
			{"zero", 0, "hours", "seconds", 0},
			{"very large numbers", 1e9, "milliseconds", "hours", 1e9 / 3600000},
		}
		for _, tc := range cases {
			t.Run(tc.name, func(t *testing.T) {
				got := ConvertTime(tc.value, tc.from, tc.to)
				if got != tc.expected {
					t.Errorf("ConvertTime(%v, %q, %q) = %v, want %v", tc.value, tc.from, tc.to, got, tc.expected)
				}
			})
		}
	})

	// Short format to short format
	t.Run("short to short", func(t *testing.T) {
		cases := []struct {
			name     string
			value    float64
			from     string
			to       string
			expected float64
		}{
			{"1 h to s", 1, "h", "s", 3600},
			{"3600 s to h", 3600, "s", "h", 1},
			{"90 m to h", 90, "m", "h", 1.5},
			{"1 h to ms", 1, "h", "ms", 3600000},
			{"0.5 s to ms", 0.5, "s", "ms", 500},
			{"1000 ms to s", 1000, "ms", "s", 1},
			{"same units (10 s)", 10, "s", "s", 10},
			{"1.5 h to m", 1.5, "h", "m", 90},
			{"zero", 0, "h", "s", 0},
			{"very large", 1e9, "ms", "h", 1e9 / 3600000},
		}
		for _, tc := range cases {
			t.Run(tc.name, func(t *testing.T) {
				got := ConvertTime(tc.value, tc.from, tc.to)
				if got != tc.expected {
					t.Errorf("ConvertTime(%v, %q, %q) = %v, want %v", tc.value, tc.from, tc.to, got, tc.expected)
				}
			})
		}
	})

	// Long format to short format
	t.Run("long to short", func(t *testing.T) {
		cases := []struct {
			name     string
			value    float64
			from     string
			to       string
			expected float64
		}{
			{"1 hours to s", 1, "hours", "s", 3600},
			{"3600 seconds to h", 3600, "seconds", "h", 1},
			{"90 minutes to h", 90, "minutes", "h", 1.5},
			{"1 hours to ms", 1, "hours", "ms", 3600000},
			{"0.5 seconds to ms", 0.5, "seconds", "ms", 500},
			{"1000 milliseconds to s", 1000, "milliseconds", "s", 1},
			{"10 seconds to s", 10, "seconds", "s", 10},
			{"1.5 hours to m", 1.5, "hours", "m", 90},
			{"zero", 0, "hours", "s", 0},
			{"very large", 1e9, "milliseconds", "h", 1e9 / 3600000},
		}
		for _, tc := range cases {
			t.Run(tc.name, func(t *testing.T) {
				got := ConvertTime(tc.value, tc.from, tc.to)
				if got != tc.expected {
					t.Errorf("ConvertTime(%v, %q, %q) = %v, want %v", tc.value, tc.from, tc.to, got, tc.expected)
				}
			})
		}
	})

	// Short format to long format
	t.Run("short to long", func(t *testing.T) {
		cases := []struct {
			name     string
			value    float64
			from     string
			to       string
			expected float64
		}{
			{"1 h to seconds", 1, "h", "seconds", 3600},
			{"3600 s to hours", 3600, "s", "hours", 1},
			{"90 m to hours", 90, "m", "hours", 1.5},
			{"1 h to milliseconds", 1, "h", "milliseconds", 3600000},
			{"0.5 s to milliseconds", 0.5, "s", "milliseconds", 500},
			{"1000 ms to seconds", 1000, "ms", "seconds", 1},
			{"10 s to seconds", 10, "s", "seconds", 10},
			{"1.5 h to minutes", 1.5, "h", "minutes", 90},
			{"zero", 0, "h", "seconds", 0},
			{"very large", 1e9, "ms", "hours", 1e9 / 3600000},
		}
		for _, tc := range cases {
			t.Run(tc.name, func(t *testing.T) {
				got := ConvertTime(tc.value, tc.from, tc.to)
				if got != tc.expected {
					t.Errorf("ConvertTime(%v, %q, %q) = %v, want %v", tc.value, tc.from, tc.to, got, tc.expected)
				}
			})
		}
	})
}
