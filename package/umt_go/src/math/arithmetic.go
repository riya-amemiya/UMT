package math

import (
	"math/big"
	stdmath "math"
	"strconv"
)

// Numeric is a type constraint for numeric types.
type Numeric interface {
	~int | ~int8 | ~int16 | ~int32 | ~int64 |
		~uint | ~uint8 | ~uint16 | ~uint32 | ~uint64 | ~uintptr |
		~float32 | ~float64
}

// parseBigFloat converts a float64 to a high-precision big.Float
// by going through string representation to avoid float64 precision loss.
func parseBigFloat(n float64) *big.Float {
	const prec = 256
	bf := new(big.Float).SetPrec(prec)
	s := strconv.FormatFloat(n, 'f', -1, 64)
	bf.SetString(s)
	return bf
}

// Addition performs addition without floating point errors using big.Float precision.
//
// Example:
//
//	Addition(0.1, 0.2) // 0.3
//	Addition(1, 2, 3)  // 6
func Addition(nums ...float64) float64 {
	if len(nums) == 0 {
		return 0
	}
	for _, n := range nums {
		if stdmath.IsNaN(n) {
			return stdmath.NaN()
		}
	}
	hasPos, hasNeg := false, false
	for _, n := range nums {
		if stdmath.IsInf(n, 1) {
			hasPos = true
		}
		if stdmath.IsInf(n, -1) {
			hasNeg = true
		}
	}
	if hasPos && hasNeg {
		return stdmath.NaN()
	}
	if hasPos {
		return stdmath.Inf(1)
	}
	if hasNeg {
		return stdmath.Inf(-1)
	}

	result := new(big.Float).SetPrec(256)
	for _, n := range nums {
		result.Add(result, parseBigFloat(n))
	}
	r, _ := result.Float64()
	return r
}

// Subtraction performs subtraction without floating point errors.
// The first argument is the minuend, and all subsequent arguments are subtracted from it.
//
// Example:
//
//	Subtraction(0.3, 0.1) // 0.2
//	Subtraction(1, 0.1, 0.2) // 0.7
func Subtraction(nums ...float64) float64 {
	if len(nums) == 0 {
		return 0
	}
	for _, n := range nums {
		if stdmath.IsNaN(n) {
			return stdmath.NaN()
		}
	}
	if stdmath.IsInf(nums[0], 0) {
		return nums[0]
	}

	result := parseBigFloat(nums[0])
	for _, n := range nums[1:] {
		if stdmath.IsInf(n, 0) {
			// subtracting +Inf gives -Inf, subtracting -Inf gives +Inf
			if stdmath.IsInf(n, 1) {
				return stdmath.Inf(-1)
			}
			return stdmath.Inf(1)
		}
		result.Sub(result, parseBigFloat(n))
	}
	r, _ := result.Float64()
	return r
}

// Multiplication performs multiplication without floating point errors using big.Float precision.
//
// Example:
//
//	Multiplication(0.1, 0.2) // 0.02
//	Multiplication(0.1, 0.2, 0.3) // 0.006
func Multiplication(nums ...float64) float64 {
	if len(nums) == 0 {
		return 1
	}
	for _, n := range nums {
		if stdmath.IsNaN(n) {
			return stdmath.NaN()
		}
	}
	// Handle Inf cases
	for _, n := range nums {
		if stdmath.IsInf(n, 0) {
			// simplified: just use standard float64 multiplication
			result := nums[0]
			for _, m := range nums[1:] {
				result *= m
			}
			return result
		}
	}

	result := parseBigFloat(nums[0])
	for _, n := range nums[1:] {
		result.Mul(result, parseBigFloat(n))
	}
	r, _ := result.Float64()
	return r
}

// Division performs division without floating point errors using big.Float precision.
// Returns NaN if any divisor is zero.
//
// Example:
//
//	Division(0.1, 0.2) // 0.5
//	Division(10, 2, 5) // 1
func Division(nums ...float64) float64 {
	if len(nums) == 0 {
		return 0
	}
	if len(nums) == 1 {
		return nums[0]
	}
	for _, n := range nums {
		if stdmath.IsNaN(n) {
			return stdmath.NaN()
		}
	}
	for _, n := range nums[1:] {
		if n == 0 {
			return stdmath.NaN()
		}
	}
	// Handle Inf cases
	for _, n := range nums {
		if stdmath.IsInf(n, 0) {
			result := nums[0]
			for _, m := range nums[1:] {
				result /= m
			}
			return result
		}
	}

	result := parseBigFloat(nums[0])
	for _, n := range nums[1:] {
		result.Quo(result, parseBigFloat(n))
	}
	r, _ := result.Float64()
	return r
}

// Quotient returns the integer quotient of a divided by b.
//
// Example:
//
//	Quotient(5, 2) // 2
//	Quotient(10, 3) // 3
func Quotient(a, b int) int {
	return a / b
}

// Max returns the maximum value from the input values.
//
// Example:
//
//	Max(1, 2, 3) // 3
//	Max(-1, -2, -3) // -1
func Max[T Numeric](values ...T) T {
	if len(values) == 0 {
		var zero T
		return zero
	}
	m := values[0]
	for _, v := range values[1:] {
		if v > m {
			m = v
		}
	}
	return m
}

// Min returns the minimum value from the input values.
//
// Example:
//
//	Min(1, 2, 3) // 1
//	Min(-1, -2, -3) // -3
func Min[T Numeric](values ...T) T {
	if len(values) == 0 {
		var zero T
		return zero
	}
	m := values[0]
	for _, v := range values[1:] {
		if v < m {
			m = v
		}
	}
	return m
}

// ValueSwap swaps the values of two pointers.
//
// Example:
//
//	a, b := 1, 2
//	ValueSwap(&a, &b)
//	// a == 2, b == 1
func ValueSwap[T any](a, b *T) {
	*a, *b = *b, *a
}
