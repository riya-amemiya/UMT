package math

import (
	stdmath "math"
	"strconv"
	"strings"
)

// ConvertCurrency converts a currency amount embedded at the start of a string
// using a map of currency symbols to conversion rates. If conversionRates is
// nil, the symbol is not present, or the conversion yields NaN, the original
// string is returned unchanged.
//
// Rate values may be numeric (int/float) or a numeric string; non-numeric
// strings cause the original input to be returned.
//
// Example:
//
//	ConvertCurrency("¥100", map[string]any{"¥": 0.01}) // "1"
//	ConvertCurrency("$50", map[string]any{"$": 1.2})   // "60"
//	ConvertCurrency("€200", map[string]any{"€": 1.1})  // "220"
func ConvertCurrency(inputString string, conversionRates map[string]any) string {
	if conversionRates == nil {
		return inputString
	}

	for currencySymbol, rate := range conversionRates {
		if strings.HasPrefix(inputString, currencySymbol) {
			amountString := inputString[len(currencySymbol):]
			rateValue, ok := looseNumber(rate)
			if !ok {
				continue
			}
			amount := jsParseNumber(amountString)
			convertedAmount := Multiplication(amount, rateValue)
			if stdmath.IsNaN(convertedAmount) {
				return inputString
			}
			return jsNumberToString(convertedAmount)
		}
	}
	return inputString
}

// jsParseNumber mirrors JavaScript's Number(string) coercion for the cases
// relevant to currency conversion: an empty or whitespace-only string becomes
// 0, and any non-numeric string becomes NaN.
func jsParseNumber(s string) float64 {
	trimmed := strings.TrimSpace(s)
	if trimmed == "" {
		return 0
	}
	f, err := strconv.ParseFloat(trimmed, 64)
	if err != nil {
		return stdmath.NaN()
	}
	return f
}

// looseNumber mirrors the loose isNumber check used by the TypeScript
// convertCurrency: numeric values and finite numeric strings are accepted,
// while booleans, non-numeric strings, and other types are rejected.
func looseNumber(value any) (float64, bool) {
	switch v := value.(type) {
	case int:
		return float64(v), true
	case int8:
		return float64(v), true
	case int16:
		return float64(v), true
	case int32:
		return float64(v), true
	case int64:
		return float64(v), true
	case uint:
		return float64(v), true
	case uint8:
		return float64(v), true
	case uint16:
		return float64(v), true
	case uint32:
		return float64(v), true
	case uint64:
		return float64(v), true
	case float32:
		f := float64(v)
		if stdmath.IsNaN(f) || stdmath.IsInf(f, 0) {
			return 0, false
		}
		return f, true
	case float64:
		if stdmath.IsNaN(v) || stdmath.IsInf(v, 0) {
			return 0, false
		}
		return v, true
	case string:
		f, err := strconv.ParseFloat(strings.TrimSpace(v), 64)
		if err != nil || stdmath.IsNaN(f) || stdmath.IsInf(f, 0) {
			return 0, false
		}
		return f, true
	default:
		return 0, false
	}
}
