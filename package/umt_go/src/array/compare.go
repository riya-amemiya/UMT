package array

import "cmp"

// CompareFunctionDefault is the default comparison function for sorting.
// It returns:
//
//	 1 if a > b
//	-1 if a < b
//	 0 if a == b
func CompareFunctionDefault[T cmp.Ordered](a, b T) int {
	if a > b {
		return 1
	}
	if a < b {
		return -1
	}
	return 0
}
