package date

import (
	"fmt"
	"math"
	"strings"
	"time"

	"github.com/riya-amemiya/umt-go/src/consts"
)

// relativeThreshold pairs a relative-time unit name with its length in
// milliseconds, ordered from largest to smallest.
type relativeThreshold struct {
	unit string
	ms   int64
}

var relativeThresholds = []relativeThreshold{
	{"year", consts.OneYearMs},
	{"month", consts.OneMonthMs},
	{"week", consts.OneWeekMs},
	{"day", consts.OneDayMs},
	{"hour", consts.OneHourMs},
	{"minute", consts.OneMinuteMs},
	{"second", consts.OneSecondMs},
}

// FormatRelative formats a date relative to a base date and returns a localized
// relative-time string. It picks the largest unit that fits the absolute delta,
// mirroring Intl.RelativeTimeFormat with numeric "auto".
//
// Only the "en" and "ja" locales are supported; any other locale (including the
// empty string) falls back to "en". The variadic options accept an optional
// base date followed by an optional locale; when the base date is omitted the
// current time is used.
//
// Example:
//
//	FormatRelative(time.Now().Add(-time.Hour), time.Now(), "en") // "1 hour ago"
//	FormatRelative(time.Now().Add(24*time.Hour), time.Now(), "ja") // "明日"
func FormatRelative(date time.Time, baseDate time.Time, locale string) string {
	deltaMs := date.UnixMilli() - baseDate.UnixMilli()
	absDelta := deltaMs
	if absDelta < 0 {
		absDelta = -absDelta
	}

	for _, threshold := range relativeThresholds {
		if absDelta >= threshold.ms {
			value := int(math.Round(float64(deltaMs) / float64(threshold.ms)))
			return formatRelativeUnit(value, threshold.unit, locale)
		}
	}
	return formatRelativeUnit(0, "second", locale)
}

// formatRelativeUnit renders a single value/unit pair using numeric "auto"
// semantics, where -1, 0, and +1 of certain units produce idiomatic words.
func formatRelativeUnit(value int, unit, locale string) string {
	if strings.HasPrefix(locale, "ja") {
		return formatRelativeJa(value, unit)
	}
	return formatRelativeEn(value, unit)
}

// formatRelativeEn renders English relative-time strings matching CLDR's
// "auto" numbering, including special words such as "now", "tomorrow", and
// "last week".
func formatRelativeEn(value int, unit string) string {
	if word, ok := relativeAutoEn[unitKey{unit, value}]; ok {
		return word
	}

	plural := unit
	if value != 1 && value != -1 {
		plural = unit + "s"
	}
	if value < 0 {
		return fmt.Sprintf("%d %s ago", -value, plural)
	}
	return fmt.Sprintf("in %d %s", value, plural)
}

// formatRelativeJa renders Japanese relative-time strings matching CLDR's
// "auto" numbering, including special words such as "今" and "明日".
func formatRelativeJa(value int, unit string) string {
	if word, ok := relativeAutoJa[unitKey{unit, value}]; ok {
		return word
	}

	counter := relativeUnitJa[unit]
	if value < 0 {
		return fmt.Sprintf("%d %s前", -value, counter)
	}
	return fmt.Sprintf("%d %s後", value, counter)
}

// unitKey identifies a special-cased value/unit combination.
type unitKey struct {
	unit  string
	value int
}

// relativeAutoEn holds the idiomatic English words used by numeric "auto".
var relativeAutoEn = map[unitKey]string{
	{"second", 0}: "now",
	{"minute", 0}: "this minute",
	{"hour", 0}:   "this hour",
	{"day", -1}:   "yesterday",
	{"day", 0}:    "today",
	{"day", 1}:    "tomorrow",
	{"week", -1}:  "last week",
	{"week", 0}:   "this week",
	{"week", 1}:   "next week",
	{"month", -1}: "last month",
	{"month", 0}:  "this month",
	{"month", 1}:  "next month",
	{"year", -1}:  "last year",
	{"year", 0}:   "this year",
	{"year", 1}:   "next year",
}

// relativeAutoJa holds the idiomatic Japanese words used by numeric "auto".
var relativeAutoJa = map[unitKey]string{
	{"second", 0}: "今",
	{"minute", 0}: "1 分以内",
	{"hour", 0}:   "1 時間以内",
	{"day", -2}:   "一昨日",
	{"day", -1}:   "昨日",
	{"day", 0}:    "今日",
	{"day", 1}:    "明日",
	{"day", 2}:    "明後日",
	{"week", -1}:  "先週",
	{"week", 0}:   "今週",
	{"week", 1}:   "来週",
	{"month", -1}: "先月",
	{"month", 0}:  "今月",
	{"month", 1}:  "来月",
	{"year", -1}:  "昨年",
	{"year", 0}:   "今年",
	{"year", 1}:   "来年",
}

// relativeUnitJa maps a unit name to its Japanese counter for non-special
// values.
var relativeUnitJa = map[string]string{
	"second": "秒",
	"minute": "分",
	"hour":   "時間",
	"day":    "日",
	"week":   "週間",
	"month":  "か月",
	"year":   "年",
}
