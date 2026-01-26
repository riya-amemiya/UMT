package math

import (
	stdmath "math"
)

// SolveEquation solves the quadratic equation ax^2 + bx + c = 0.
// Returns a slice of real roots:
//   - Empty slice if no real roots (negative discriminant)
//   - Single element if discriminant is zero
//   - Two elements if discriminant is positive
//
// If a == 0, solves the linear equation bx + c = 0.
//
// Example:
//
//	SolveEquation(1, -3, 2)  // [2, 1] (x^2 - 3x + 2 = 0)
//	SolveEquation(1, -2, 1)  // [1] (x^2 - 2x + 1 = 0)
//	SolveEquation(1, 0, 1)   // [] (x^2 + 1 = 0, no real roots)
//	SolveEquation(0, 2, -4)  // [2] (2x - 4 = 0)
func SolveEquation(a, b, c float64) []float64 {
	if a == 0 {
		if b == 0 {
			return []float64{}
		}
		return []float64{-c / b}
	}

	discriminant := b*b - 4*a*c

	if discriminant < 0 {
		return []float64{}
	}

	if discriminant == 0 {
		return []float64{-b / (2 * a)}
	}

	sqrtD := stdmath.Sqrt(discriminant)
	x1 := (-b + sqrtD) / (2 * a)
	x2 := (-b - sqrtD) / (2 * a)
	return []float64{x1, x2}
}
