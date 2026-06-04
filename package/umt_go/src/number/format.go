// Package number provides numeric formatting helpers that mirror the TypeScript
// Number module (formatNumber, toOrdinal, toPercentage).
package number

import (
	"math"
	"strconv"
	"strings"
)

// FormatNumberOptions configures FormatNumber. The zero value formats a number
// as a plain decimal in the default ("en-US") locale.
//
// Each field mirrors the corresponding Intl.NumberFormat option. The pointer
// fields distinguish "not provided" (nil) from an explicit value, matching the
// TypeScript options object where an undefined key falls back to the locale and
// style defaults.
type FormatNumberOptions struct {
	// Locale selects the formatting locale (e.g. "en-US", "de-DE", "ja-JP").
	// An empty string uses the default locale ("en-US").
	Locale string
	// MinimumFractionDigits is the minimum number of fraction digits. nil keeps
	// the style/currency default.
	MinimumFractionDigits *int
	// MaximumFractionDigits is the maximum number of fraction digits. nil keeps
	// the style/currency default.
	MaximumFractionDigits *int
	// Style is the formatting style: "decimal" (default), "currency", or
	// "percent". An empty string is treated as "decimal".
	Style string
	// Currency is the ISO 4217 currency code, required when Style is "currency".
	Currency string
}

// localeData holds the separators and percent spacing for a single locale.
type localeData struct {
	group   string
	decimal string
	// percentSpace is inserted between the number and the "%" sign.
	percentSpace string
	// currencyAfter places the currency symbol after the number (with
	// currencySpace between them) instead of before it.
	currencyAfter bool
	currencySpace string
}

// nbsp is the non-breaking space (U+00A0) used by several locales between a
// number and a unit such as "%".
const nbsp = " "

var localeTable = map[string]localeData{
	"en-US": {group: ",", decimal: ".", percentSpace: "", currencyAfter: false, currencySpace: ""},
	"de-DE": {group: ".", decimal: ",", percentSpace: nbsp, currencyAfter: true, currencySpace: nbsp},
	"ja-JP": {group: ",", decimal: ".", percentSpace: "", currencyAfter: false, currencySpace: ""},
}

// currencyData holds the symbol and default fraction-digit count for a currency.
type currencyData struct {
	symbol   string
	fraction int
}

var currencyTable = map[string]currencyData{
	"USD": {symbol: "$", fraction: 2},
	// JPY uses the fullwidth yen sign (U+FFE5) and no fraction digits.
	"JPY": {symbol: "￥", fraction: 0},
	"EUR": {symbol: "€", fraction: 2},
	"GBP": {symbol: "£", fraction: 2},
}

// resolveLocale returns the localeData for name, falling back to "en-US".
func resolveLocale(name string) localeData {
	if name == "" {
		return localeTable["en-US"]
	}
	if data, ok := localeTable[name]; ok {
		return data
	}
	return localeTable["en-US"]
}

// FormatNumber formats a number, mirroring JavaScript's Intl.NumberFormat.
//
// With no options the value is formatted as a grouped decimal in the default
// ("en-US") locale, using up to 3 fraction digits. The Style field selects
// "currency" or "percent" formatting; for "percent" the value is multiplied by
// 100 before formatting. Rounding follows Intl's default "halfExpand" rule
// (round half away from zero). Locales "en-US", "de-DE", and "ja-JP" are
// supported; any other locale falls back to "en-US".
//
// Example:
//
//	FormatNumber(1234567.89, FormatNumberOptions{Locale: "en-US"})
//	// "1,234,567.89"
//
//	FormatNumber(0.75, FormatNumberOptions{Style: "percent", Locale: "en-US"})
//	// "75%"
//
//	FormatNumber(1234.5, FormatNumberOptions{
//	    Style: "currency", Currency: "USD", Locale: "en-US",
//	})
//	// "$1,234.50"
func FormatNumber(value float64, options ...FormatNumberOptions) string {
	var opts FormatNumberOptions
	if len(options) > 0 {
		opts = options[0]
	}

	locale := resolveLocale(opts.Locale)
	style := opts.Style
	if style == "" {
		style = "decimal"
	}

	scaled := value
	if style == "percent" {
		scaled = value * 100
	}

	minFrac, maxFrac := defaultFractionDigits(style, opts.Currency)
	if opts.MinimumFractionDigits != nil {
		minFrac = *opts.MinimumFractionDigits
		if maxFrac < minFrac {
			maxFrac = minFrac
		}
	}
	if opts.MaximumFractionDigits != nil {
		maxFrac = *opts.MaximumFractionDigits
		if maxFrac < minFrac {
			minFrac = maxFrac
		}
	}

	body := formatDecimalString(scaled, minFrac, maxFrac, locale)

	switch style {
	case "percent":
		return body + locale.percentSpace + "%"
	case "currency":
		symbol := opts.Currency
		if data, ok := currencyTable[opts.Currency]; ok {
			symbol = data.symbol
		}
		if locale.currencyAfter {
			return body + locale.currencySpace + symbol
		}
		return symbol + body
	default:
		return body
	}
}

// defaultFractionDigits returns the (min, max) fraction-digit defaults for a
// style/currency combination, matching Intl.NumberFormat's defaults.
func defaultFractionDigits(style, currency string) (int, int) {
	switch style {
	case "currency":
		if data, ok := currencyTable[currency]; ok {
			return data.fraction, data.fraction
		}
		return 2, 2
	case "percent":
		return 0, 0
	default:
		return 0, 3
	}
}

// formatDecimalString rounds value to maxFrac fraction digits (half away from
// zero), trims trailing zeros down to minFrac, and applies the locale's group
// and decimal separators.
func formatDecimalString(value float64, minFrac, maxFrac int, locale localeData) string {
	negative := math.Signbit(value) && value != 0
	abs := math.Abs(value)

	// Round to maxFrac digits, rounding halves away from zero.
	factor := math.Pow(10, float64(maxFrac))
	rounded := math.Floor(abs*factor+0.5) / factor

	// Render with maxFrac digits, then trim trailing zeros down to minFrac.
	rendered := strconv.FormatFloat(rounded, 'f', maxFrac, 64)

	intPart := rendered
	fracPart := ""
	if dot := strings.IndexByte(rendered, '.'); dot >= 0 {
		intPart = rendered[:dot]
		fracPart = rendered[dot+1:]
	}

	for len(fracPart) > minFrac && strings.HasSuffix(fracPart, "0") {
		fracPart = fracPart[:len(fracPart)-1]
	}

	grouped := groupInteger(intPart, locale.group)

	result := grouped
	if len(fracPart) > 0 {
		result += locale.decimal + fracPart
	}

	if negative && result != zeroBody(minFrac, locale) {
		return "-" + result
	}
	return result
}

// zeroBody returns the formatted representation of zero with minFrac digits, so
// a rounded-to-zero negative value (e.g. -0.0001 -> "0") does not gain a sign.
func zeroBody(minFrac int, locale localeData) string {
	if minFrac <= 0 {
		return "0"
	}
	return "0" + locale.decimal + strings.Repeat("0", minFrac)
}

// groupInteger inserts the group separator into an integer-part string.
func groupInteger(intPart, group string) string {
	if group == "" || len(intPart) <= 3 {
		return intPart
	}
	var builder strings.Builder
	offset := len(intPart) % 3
	if offset > 0 {
		builder.WriteString(intPart[:offset])
	}
	for i := offset; i < len(intPart); i += 3 {
		if builder.Len() > 0 {
			builder.WriteString(group)
		}
		builder.WriteString(intPart[i : i+3])
	}
	return builder.String()
}
