package math

import (
	stdmath "math"
)

// Factorial calculates the factorial of n.
// Returns 1 for n <= 1 (including negative numbers).
// For non-integer values, computes factorial of floor(n) or 1 if n < 2.
//
// Example:
//
//	Factorial(5)  // 120
//	Factorial(0)  // 1
//	Factorial(-1) // 1
func Factorial(n int) int {
	limit := n
	if limit < 1 {
		limit = 1
	}
	result := 1
	for i := 2; i <= limit; i++ {
		result *= i
	}
	return result
}

// NCr calculates the number of combinations (n choose r).
// Returns -1 for invalid inputs (n < r, n < 0, or r < 0).
//
// Example:
//
//	NCr(5, 2)  // 10
//	NCr(10, 4) // 210
//	NCr(0, 0)  // 1
func NCr(n, r int) int {
	if n < r || n < 0 || r < 0 {
		return -1
	}
	if r == 0 || n == r {
		return 1
	}

	numerator := NPr(n, r)
	if numerator < 0 {
		return -1
	}

	denominator := 1
	for i := 2; i <= r; i++ {
		denominator *= i
	}

	return numerator / denominator
}

// NPr calculates the number of permutations.
// Returns -1 for invalid inputs (n < r, n < 0, or r < 0).
//
// Example:
//
//	NPr(5, 2) // 20
//	NPr(3, 3) // 6
func NPr(n, r int) int {
	if n < r || n < 0 || r < 0 {
		return -1
	}
	if r == 0 {
		return 1
	}
	result := 1
	for i := 0; i < r; i++ {
		result *= n - i
	}
	return result
}

// NHr calculates combinations with repetition (n multichoose r).
// Returns -1 for invalid inputs (n <= 0 or r <= 0).
// Uses the formula nHr = C(n+r-1, r).
//
// Example:
//
//	NHr(5, 2)  // 15
//	NHr(3, 3)  // 10
func NHr(n, r int) int {
	if n <= 0 || r <= 0 {
		return -1
	}
	return NCr(n+r-1, r)
}

// RepeatedTrial calculates the probability of exactly x successes in n
// independent Bernoulli trials, each with probability p.
// Uses the binomial probability formula: P(X=x) = C(n,x) * p^x * (1-p)^(n-x)
//
// Example:
//
//	RepeatedTrial(5, 0.5, 2) // probability of exactly 2 successes in 5 trials with p=0.5
func RepeatedTrial(n int, p float64, x int) float64 {
	c := NCr(n, x)
	if c < 0 {
		return 0
	}
	return float64(c) * stdmath.Pow(p, float64(x)) * stdmath.Pow(1-p, float64(n-x))
}
