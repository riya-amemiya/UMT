package math

import (
	stdmath "math"
	"regexp"
	"strconv"
	"strings"

	"github.com/riya-amemiya/umt-go/src/tool"
)

// jsNumberToString formats a float64 the way JavaScript's String(number)
// (Number.prototype.toString) does. This matters for the calculator port so
// that observable string output matches the TypeScript reference, including
// "Infinity"/"-Infinity"/"NaN" and the scientific-notation thresholds.
//
// JavaScript switches to exponential notation when the exponent is >= 21
// (large magnitudes) or <= -7 (small magnitudes); otherwise it uses plain
// decimal notation with the shortest round-tripping representation.
func jsNumberToString(n float64) string {
	if stdmath.IsNaN(n) {
		return "NaN"
	}
	if stdmath.IsInf(n, 1) {
		return "Infinity"
	}
	if stdmath.IsInf(n, -1) {
		return "-Infinity"
	}
	if n == 0 {
		return "0"
	}

	abs := stdmath.Abs(n)
	if abs >= 1e21 || abs < 1e-6 {
		s := strconv.FormatFloat(n, 'e', -1, 64)
		return normalizeJSExponent(s)
	}
	return strconv.FormatFloat(n, 'f', -1, 64)
}

// normalizeJSExponent converts Go's exponential form (e.g. "1e+21", "1.5e-07")
// into JavaScript's form (e.g. "1e+21", "1.5e-7"): no leading zeros in the
// exponent and a mandatory sign.
func normalizeJSExponent(s string) string {
	idx := strings.IndexByte(s, 'e')
	if idx < 0 {
		return s
	}
	mantissa := s[:idx]
	exp := s[idx+1:]
	sign := "+"
	if len(exp) > 0 && (exp[0] == '+' || exp[0] == '-') {
		if exp[0] == '-' {
			sign = "-"
		}
		exp = exp[1:]
	}
	exp = strings.TrimLeft(exp, "0")
	if exp == "" {
		exp = "0"
	}
	return mantissa + "e" + sign + exp
}

var (
	calcParenRe  = regexp.MustCompile(`\((-?\d+(?:\.\d+)?)([*+/-])(-?\d+(?:\.\d+)?)\)`)
	calcMulExpRe = regexp.MustCompile(`(.*?)(-?\d+(?:\.\d+)?)([*^])(-?\d+(?:\.\d+)?)$`)
	calcDivRe    = regexp.MustCompile(`(-?\d+(?:\.\d+)?)/(-?\d+(?:\.\d+)?)`)
	calcAddSubRe = regexp.MustCompile(`(-?\d+(?:\.\d+)?)(\+|-)(-?\d+(?:\.\d+)?)`)
)

// Calculator evaluates a mathematical expression or a simple single-variable
// equation. Whitespace is stripped first. Expressions containing "=" are
// treated as equations and solved via literalExpression; otherwise the
// expression is reduced via the calculatorCore evaluation loop. Currency
// symbols can be substituted using the optional exchange map.
//
// When the input is not a fully reducible expression, the partially-reduced
// string is returned unchanged (e.g. "1+" stays "1+").
//
// Example:
//
//	Calculator("1+2", nil)            // "3"
//	Calculator("(2+3)*4", nil)        // "20"
//	Calculator("x=5", nil)            // "x=5"
//	Calculator("$10*2", map[string]any{"$": 100}) // "2000"
func Calculator(expression string, exchange map[string]any) string {
	cleanExpression := regexp.MustCompile(`\s+`).ReplaceAllString(expression, "")
	if strings.Contains(cleanExpression, "=") {
		return literalExpression(cleanExpression)
	}
	return calculatorCore(cleanExpression, exchange)
}

// CalculatorInitialization returns a Calculator function pre-configured with the
// given currency exchange rates.
//
// Example:
//
//	calc := CalculatorInitialization(map[string]any{"$": 100})
//	calc("$1")      // "100"
//	calc("100 + $1") // "200"
func CalculatorInitialization(exchange map[string]any) func(string) string {
	return func(x string) string {
		return Calculator(x, exchange)
	}
}

func calculatorCore(expression string, currencyExchange map[string]any) string {
	if expression == "" {
		return ""
	}

	sanitizedExpression := sanitizeSigns(expression)

	for {
		if currencyExchange != nil {
			sanitizedExpression = applyCurrencyExchange(sanitizedExpression, currencyExchange)
		}

		switch {
		case containsParentheses(sanitizedExpression):
			temporary := resolveParentheses(sanitizedExpression)
			if temporary == "NaN" {
				return sanitizedExpression
			}
			sanitizedExpression = temporary

		case containsMulExp(sanitizedExpression):
			temporary := resolveMulExp(sanitizedExpression)
			if temporary == "NaN" {
				return sanitizedExpression
			}
			sanitizedExpression = temporary

		case containsDiv(sanitizedExpression):
			temporary := resolveDiv(sanitizedExpression)
			if temporary == "NaN" {
				return sanitizedExpression
			}
			sanitizedExpression = temporary

		case containsAddSub(sanitizedExpression) && !isNumberString(sanitizedExpression):
			temporary := resolveAddSub(sanitizedExpression)
			if temporary == "NaN" {
				return sanitizedExpression
			}
			sanitizedExpression = temporary

		default:
			number, err := strconv.ParseFloat(sanitizedExpression, 64)
			if err == nil && !stdmath.IsNaN(number) {
				rounded := stdmath.Round(number*1e10) / 1e10
				return jsNumberToString(rounded)
			}
			return sanitizedExpression
		}
	}
}

func sanitizeSigns(expr string) string {
	expr = strings.ReplaceAll(expr, "--", "+")
	expr = strings.ReplaceAll(expr, "++", "+")
	expr = strings.ReplaceAll(expr, "+-", "+0-")
	expr = strings.ReplaceAll(expr, "-+", "+0-")
	return expr
}

func applyCurrencyExchange(expr string, rates map[string]any) string {
	returnExpr := expr
	for index := range rates {
		if strings.Contains(returnExpr, index) {
			re := getCurrencyRegex(index)
			loc := re.FindString(returnExpr)
			if loc != "" {
				returnExpr = strings.Replace(returnExpr, loc, ConvertCurrency(loc, rates), 1)
			}
		}
	}
	return returnExpr
}

func getCurrencyRegex(currencySymbol string) *regexp.Regexp {
	return regexp.MustCompile(tool.EscapeRegExp(currencySymbol) + `([0-9]+)`)
}

func containsParentheses(expr string) bool {
	return strings.Contains(expr, "(") || strings.Contains(expr, ")")
}

func resolveParentheses(expr string) string {
	match := calcParenRe.FindString(expr)
	if match != "" {
		inner := strings.NewReplacer("(", "", ")", "").Replace(match)
		return strings.Replace(expr, match, calculatorCore(inner, nil), 1)
	}
	return "NaN"
}

func containsMulExp(expr string) bool {
	return strings.Contains(expr, "^") || strings.Contains(expr, "*")
}

func resolveMulExp(expr string) string {
	match := calcMulExpRe.FindStringSubmatch(expr)
	if match != nil {
		left, _ := strconv.ParseFloat(match[2], 64)
		right, _ := strconv.ParseFloat(match[4], 64)
		var result float64
		if match[3] == "^" {
			result = stdmath.Pow(left, right)
		} else {
			result = Multiplication(left, right)
		}
		return match[1] + jsNumberToString(result)
	}
	return "NaN"
}

func containsDiv(expr string) bool {
	return strings.Contains(expr, "/")
}

func resolveDiv(expr string) string {
	match := calcDivRe.FindStringSubmatch(expr)
	if match != nil {
		left, _ := strconv.ParseFloat(match[1], 64)
		right, _ := strconv.ParseFloat(match[2], 64)
		result := Division(left, right)
		return strings.Replace(expr, match[0], jsNumberToString(result), 1)
	}
	return "NaN"
}

func containsAddSub(expr string) bool {
	return strings.Contains(expr, "+") || strings.Contains(expr, "-")
}

func resolveAddSub(expr string) string {
	match := calcAddSubRe.FindStringSubmatch(expr)
	if match != nil {
		left, _ := strconv.ParseFloat(match[1], 64)
		right, _ := strconv.ParseFloat(match[3], 64)
		var result float64
		if match[2] == "+" {
			result = Addition(left, right)
		} else {
			result = Subtraction(left, right)
		}
		return strings.Replace(expr, match[0], jsNumberToString(result), 1)
	}
	return "NaN"
}

// isNumberString reports whether s parses as a finite number, mirroring the
// loose isNumber check used by the TypeScript calculatorCore on the running
// expression string.
func isNumberString(s string) bool {
	if s == "" {
		return false
	}
	n, err := strconv.ParseFloat(s, 64)
	if err != nil {
		return false
	}
	return !stdmath.IsNaN(n) && !stdmath.IsInf(n, 0)
}
