package advance

// RangeAdvance returns a slice of integers from start (inclusive) to end (exclusive).
// If a condition function is provided, only numbers satisfying the condition are included.
// If no condition is provided, all numbers in the range are returned.
//
// Returns an empty slice if start >= end.
//
// Example:
//
//	RangeAdvance(0, 5, nil)                              // [0, 1, 2, 3, 4]
//	RangeAdvance(2, 5, nil)                              // [2, 3, 4]
//	RangeAdvance(0, 10, func(n int) bool { return n%2 == 0 }) // [0, 2, 4, 6, 8]
//	RangeAdvance(5, 2, nil)                              // []
func RangeAdvance(start, end int, condition func(int) bool) []int {
	result := []int{}

	if condition != nil {
		for i := start; i < end; i++ {
			if condition(i) {
				result = append(result, i)
			}
		}
		return result
	}

	for i := start; i < end; i++ {
		result = append(result, i)
	}
	return result
}
