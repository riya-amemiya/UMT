package timeutil

// conversionRates maps time unit names to their value in milliseconds.
var conversionRates = map[string]float64{
	"milliseconds": 1,
	"seconds":      1000,
	"minutes":      60000,
	"hours":        3600000,
}

// timeUnitMap maps both long and short time unit names to their canonical forms.
var timeUnitMap = map[string]struct {
	Long  string
	Short string
}{
	"milliseconds": {Long: "milliseconds", Short: "ms"},
	"seconds":      {Long: "seconds", Short: "s"},
	"minutes":      {Long: "minutes", Short: "m"},
	"hours":        {Long: "hours", Short: "h"},
	"ms":           {Long: "milliseconds", Short: "ms"},
	"s":            {Long: "seconds", Short: "s"},
	"m":            {Long: "minutes", Short: "m"},
	"h":            {Long: "hours", Short: "h"},
}

// NormalizeTimeUnit normalizes a time unit string to the specified form.
// The "to" parameter should be "long" or "short".
// Supported units: "milliseconds"/"ms", "seconds"/"s", "minutes"/"m", "hours"/"h".
func NormalizeTimeUnit(unit string, to string) string {
	entry, ok := timeUnitMap[unit]
	if !ok {
		return unit
	}
	if to == "short" {
		return entry.Short
	}
	return entry.Long
}

// ConvertTime converts a time value from one unit to another.
// Supported units (long): "milliseconds", "seconds", "minutes", "hours"
// Supported units (short): "ms", "s", "m", "h"
func ConvertTime(value float64, from, to string) float64 {
	normalizedFrom := NormalizeTimeUnit(from, "long")
	normalizedTo := NormalizeTimeUnit(to, "long")

	fromRate := conversionRates[normalizedFrom]
	toRate := conversionRates[normalizedTo]

	milliseconds := value * fromRate
	return milliseconds / toRate
}
