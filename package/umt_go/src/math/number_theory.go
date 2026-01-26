package math

import "sort"

// IsPrimeNumber checks whether n is a prime number.
//
// Example:
//
//	IsPrimeNumber(2)  // true
//	IsPrimeNumber(17) // true
//	IsPrimeNumber(4)  // false
//	IsPrimeNumber(1)  // false
func IsPrimeNumber(n int) bool {
	if n < 2 {
		return false
	}
	if n == 2 {
		return true
	}
	if n%2 == 0 {
		return false
	}
	for i := 3; i*i <= n; i += 2 {
		if n%i == 0 {
			return false
		}
	}
	return true
}

// PrimeFactorization returns the prime factors of n as a flat list
// (with repeated factors for multiplicity).
//
// Example:
//
//	PrimeFactorization(12)   // [2, 2, 3]
//	PrimeFactorization(17)   // [17]
//	PrimeFactorization(1000) // [2, 2, 2, 5, 5, 5]
func PrimeFactorization(n int) []int {
	result := []int{}
	remaining := n
	if remaining < 0 {
		remaining = -remaining
	}
	for factor := 2; factor*factor <= remaining; factor++ {
		for remaining%factor == 0 {
			result = append(result, factor)
			remaining /= factor
		}
	}
	if remaining > 1 {
		result = append(result, remaining)
	}
	return result
}

// Factorize returns all divisors of n, sorted in ascending order.
//
// Example:
//
//	Factorize(12) // [1, 2, 3, 4, 6, 12]
//	Factorize(7)  // [1, 7]
//	Factorize(1)  // [1]
func Factorize(n int) []int {
	if n == 0 {
		return []int{}
	}
	absN := n
	if absN < 0 {
		absN = -absN
	}
	var divisors []int
	for i := 1; i*i <= absN; i++ {
		if absN%i == 0 {
			divisors = append(divisors, i)
			if i != absN/i {
				divisors = append(divisors, absN/i)
			}
		}
	}
	sort.Ints(divisors)
	return divisors
}

// GCD calculates the Greatest Common Divisor of a and b
// using the Euclidean algorithm.
//
// Example:
//
//	GCD(12, 18) // 6
//	GCD(56, 48) // 8
func GCD(a, b int) int {
	if a < 0 {
		a = -a
	}
	if b < 0 {
		b = -b
	}
	for b != 0 {
		a, b = b, a%b
	}
	return a
}

// LCM calculates the Least Common Multiple of a and b.
// Returns 0 if either a or b is 0.
//
// Example:
//
//	LCM(4, 5)  // 20
//	LCM(6, 8)  // 24
//	LCM(0, 5)  // 0
func LCM(a, b int) int {
	if a == 0 || b == 0 {
		return 0
	}
	absA := a
	if absA < 0 {
		absA = -absA
	}
	absB := b
	if absB < 0 {
		absB = -absB
	}
	return absA / GCD(absA, absB) * absB
}
