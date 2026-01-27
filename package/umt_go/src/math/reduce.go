package math

import "errors"

// ReduceResult represents a reduced fraction with its GCD.
type ReduceResult struct {
	X   int
	Y   int
	GCD int
}

// ErrZeroDenominator is returned when the denominator is zero.
var ErrZeroDenominator = errors.New("denominator cannot be zero")

// Reduce reduces a fraction to its lowest terms.
// Returns an error if the denominator y is zero.
//
// Example:
//
//	Reduce(2, 4)   // ReduceResult{X: 1, Y: 2, GCD: 2}, nil
//	Reduce(3, 6)   // ReduceResult{X: 1, Y: 2, GCD: 3}, nil
//	Reduce(0, 5)   // ReduceResult{X: 0, Y: 1, GCD: 5}, nil
//	Reduce(5, 0)   // ReduceResult{}, ErrZeroDenominator
//	Reduce(-6, 8)  // ReduceResult{X: -3, Y: 4, GCD: 2}, nil
//	Reduce(6, -8)  // ReduceResult{X: -3, Y: 4, GCD: 2}, nil
//	Reduce(-6, -8) // ReduceResult{X: 3, Y: 4, GCD: 2}, nil
func Reduce(x, y int) (ReduceResult, error) {
	if y == 0 {
		return ReduceResult{}, ErrZeroDenominator
	}

	if x == 0 {
		absY := y
		if absY < 0 {
			absY = -absY
		}
		return ReduceResult{X: 0, Y: 1, GCD: absY}, nil
	}

	absX := x
	if absX < 0 {
		absX = -absX
	}
	absY := y
	if absY < 0 {
		absY = -absY
	}

	gcdValue := GCD(absX, absY)

	sign := 1
	if y < 0 {
		sign = -1
	}

	return ReduceResult{
		X:   (x / gcdValue) * sign,
		Y:   absY / gcdValue,
		GCD: gcdValue,
	}, nil
}
