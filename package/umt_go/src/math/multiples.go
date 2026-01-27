package math

// Multiples generates an array of multiples of a number.
//
// Example:
//
//	Multiples(2, 5) // [2, 4, 6, 8, 10]
//	Multiples(1, 3) // [1, 2, 3]
//	Multiples(-2, 3) // [-2, -4, -6]
//	Multiples(0.5, 3) // [0.5, 1.0, 1.5]
func Multiples(x float64, n int) []float64 {
	result := make([]float64, 0, n)
	for i := 1; i <= n; i++ {
		result = append(result, x*float64(i))
	}
	return result
}
