package math

import (
	"fmt"
	stdmath "math"
	"regexp"
	"strconv"
	"strings"
)

var mulOrExpRe = regexp.MustCompile(`\d+\.?(\d+)?(\*|\^)\d+\.?(\d+)?`)

// MathConverter expands the square of n into a sum of simpler multiplications
// using the distributive property.
//
// For example, 1250*1250 is equivalent to (1000+250)^2, which expands to:
//
//	1500*1000 + 400*100 + 200*100 + 50*50
//
// The function handles both multiplication (n*n) and exponentiation (n^2) notation.
//
// Example:
//
//	MathConverter("1250*1250") // "1500*1000+400*100+200*100+50*50"
//	MathConverter("1250^2")    // "1500*1000+400*100+200*100+50*50"
//	MathConverter("1250")      // "1250" (no conversion needed)
//	MathConverter("abc")       // "abc" (invalid input returned as-is)
func MathConverter(equation string) string {
	convertedEquation := equation

	for {
		match := mulOrExpRe.FindString(convertedEquation)
		if match == "" {
			return convertedEquation
		}

		// Find the operator (* or ^) position in the match
		opIdx := -1
		var operator string
		for i, ch := range match {
			if ch == '*' || ch == '^' {
				opIdx = i
				operator = string(ch)
				break
			}
		}

		operand1 := match[:opIdx]
		operand2 := match[opIdx+1:]

		if operand1 == operand2 || (operand2 != "" && operator == "^") {
			primary, remainder := splitAtHighestPlace(operand1)

			if primary == 0 {
				return convertedEquation
			}

			op1Val, _ := strconv.ParseFloat(operand1, 64)
			newLeft := op1Val + remainder

			convertedEquation = fmt.Sprintf("%s*%s+", formatNumClean(newLeft), formatNumClean(primary))

			if remainder <= 100 {
				convertedEquation += fmt.Sprintf("%s*%s", formatNumClean(remainder), formatNumClean(remainder))
			} else {
				convertedEquation += MathConverter(fmt.Sprintf("%s*%s", formatNumClean(remainder), formatNumClean(remainder)))
			}
		} else {
			return convertedEquation
		}
	}
}

// splitAtHighestPlace separates a number string at its highest place value.
// For example, "1250" -> (1000, 250), "350" -> (100, 250).
// Returns (0, 0) if the input is not a valid number.
func splitAtHighestPlace(input string) (float64, float64) {
	_, err := strconv.ParseFloat(input, 64)
	if err != nil {
		return 0, 0
	}

	parts := strings.Split(input, ".")
	integerPart := parts[0]
	decimalPart := 0.0
	if len(parts) > 1 && parts[1] != "" {
		decimalPart, _ = strconv.ParseFloat("0."+parts[1], 64)
	}

	numberOfDigits := len(integerPart) - 1
	numericalValue, _ := strconv.ParseFloat(integerPart, 64)

	if numberOfDigits == 0 {
		return numericalValue, decimalPart
	}

	primary := stdmath.Pow(10, float64(numberOfDigits))
	remainder := numericalValue - primary + decimalPart

	return primary, remainder
}

// formatNumClean formats a float64 as a clean string, removing unnecessary
// decimal points for whole numbers.
func formatNumClean(n float64) string {
	if n == float64(int64(n)) {
		return strconv.FormatInt(int64(n), 10)
	}
	return strconv.FormatFloat(n, 'f', -1, 64)
}
