package math

import (
	"fmt"
	stdmath "math"
	"strconv"
	"strings"
)

// DegToRad converts an angle from degrees to radians.
//
// Example:
//
//	DegToRad(180) // 3.141592653589793
//	DegToRad(90)  // 1.5707963267948966
func DegToRad(deg float64) float64 {
	return deg * stdmath.Pi / 180
}

// RadToDeg converts an angle from radians to degrees.
//
// Example:
//
//	RadToDeg(math.Pi)   // 180
//	RadToDeg(math.Pi/2) // 90
func RadToDeg(rad float64) float64 {
	return rad * 180 / stdmath.Pi
}

// ToBaseN converts a decimal integer to a string representation in the specified base.
// The base must be between 2 and 36.
//
// Example:
//
//	ToBaseN(10, 2)  // "1010"
//	ToBaseN(15, 16) // "f"
//	ToBaseN(255, 16) // "ff"
func ToBaseN(n int, base int) string {
	return strconv.FormatInt(int64(n), base)
}

// FlexibleNumberConversion converts a string representation of a number
// from one base to another.
//
// Example:
//
//	FlexibleNumberConversion("ff", 16, 10)   // ("255", nil)
//	FlexibleNumberConversion("1010", 2, 10)  // ("10", nil)
//	FlexibleNumberConversion("255", 10, 16)  // ("ff", nil)
func FlexibleNumberConversion(value string, fromBase, toBase int) (string, error) {
	n, err := strconv.ParseInt(strings.TrimSpace(value), fromBase, 64)
	if err != nil {
		return "", fmt.Errorf("failed to parse %q in base %d: %w", value, fromBase, err)
	}
	return strconv.FormatInt(n, toBase), nil
}

// GetDecimalLength returns the number of decimal places in a float64 number.
//
// Example:
//
//	GetDecimalLength(1.23) // 2
//	GetDecimalLength(100)  // 0
//	GetDecimalLength(1.0)  // 0
func GetDecimalLength(n float64) int {
	s := strconv.FormatFloat(n, 'f', -1, 64)
	parts := strings.Split(s, ".")
	if len(parts) == 2 && len(parts[1]) > 0 {
		return len(parts[1])
	}
	return 0
}

// RoundOf rounds a number to the specified number of decimal places.
//
// Example:
//
//	RoundOf(1.234, 2)  // 1.23
//	RoundOf(1.235, 2)  // 1.24
//	RoundOf(-1.234, 2) // -1.23
func RoundOf(n float64, precision int) float64 {
	p := stdmath.Pow(10, float64(precision))
	return stdmath.Round(n*p) / p
}

// MathSeparator formats a number with the specified separator for thousands grouping.
//
// Example:
//
//	MathSeparator(1234567.89, ",") // "1,234,567.89"
//	MathSeparator(1000, ",")       // "1,000"
func MathSeparator(n float64, sep string) string {
	s := strconv.FormatFloat(n, 'f', -1, 64)
	parts := strings.Split(s, ".")
	intPart := parts[0]

	negative := false
	if strings.HasPrefix(intPart, "-") {
		negative = true
		intPart = intPart[1:]
	}

	// Insert separators every 3 digits from right
	length := len(intPart)
	if length <= 3 {
		formatted := intPart
		if negative {
			formatted = "-" + formatted
		}
		if len(parts) == 2 {
			formatted += "." + parts[1]
		}
		return formatted
	}

	var result strings.Builder
	firstGroupLen := length % 3
	if firstGroupLen == 0 {
		firstGroupLen = 3
	}

	result.WriteString(intPart[:firstGroupLen])
	for i := firstGroupLen; i < length; i += 3 {
		result.WriteString(sep)
		result.WriteString(intPart[i : i+3])
	}

	formatted := result.String()
	if negative {
		formatted = "-" + formatted
	}
	if len(parts) == 2 {
		formatted += "." + parts[1]
	}
	return formatted
}
