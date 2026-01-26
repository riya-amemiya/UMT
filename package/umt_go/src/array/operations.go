package array

import (
	"math"
	"math/rand"
)

// Chunk splits an array into smaller chunks of specified size.
// Returns a new 2D slice without mutating the input.
func Chunk[T any](arr []T, size int) [][]T {
	if size <= 0 || len(arr) == 0 {
		return [][]T{}
	}
	numChunks := (len(arr) + size - 1) / size
	result := make([][]T, 0, numChunks)
	for i := 0; i < len(arr); i += size {
		end := i + size
		if end > len(arr) {
			end = len(arr)
		}
		chunk := make([]T, end-i)
		copy(chunk, arr[i:end])
		result = append(result, chunk)
	}
	return result
}

// Drop returns a new array with n elements removed from the beginning.
// If n is negative, the original array is returned as-is.
// If n >= len(arr), an empty array is returned.
func Drop[T any](arr []T, n int) []T {
	if n < 0 {
		result := make([]T, len(arr))
		copy(result, arr)
		return result
	}
	if n >= len(arr) {
		return []T{}
	}
	result := make([]T, len(arr)-n)
	copy(result, arr[n:])
	return result
}

// First returns the first element of a slice and a boolean indicating success.
// Returns the zero value and false if the slice is empty.
func First[T any](arr []T) (T, bool) {
	if len(arr) == 0 {
		var zero T
		return zero, false
	}
	return arr[0], true
}

// Pop returns a new slice without the last element, the removed last element,
// and a boolean indicating success. Does not mutate the input.
func Pop[T any](arr []T) ([]T, T, bool) {
	if len(arr) == 0 {
		var zero T
		return []T{}, zero, false
	}
	last := arr[len(arr)-1]
	result := make([]T, len(arr)-1)
	copy(result, arr[:len(arr)-1])
	return result, last, true
}

// Range generates a sequence of integers from start (inclusive) to end (exclusive).
// Returns an empty slice if start >= end.
func Range(start, end int) []int {
	if start >= end {
		return []int{}
	}
	result := make([]int, 0, end-start)
	for i := start; i < end; i++ {
		result = append(result, i)
	}
	return result
}

// RangeFloat generates a sequence of float64 values from start to end (exclusive)
// with the given step. Handles floating point precision using rounding.
// Returns an empty slice if step is zero or the direction is invalid.
func RangeFloat(start, end, step float64) []float64 {
	if step == 0 {
		return []float64{}
	}
	if (step > 0 && start >= end) || (step < 0 && start <= end) {
		return []float64{}
	}
	result := []float64{}
	if step > 0 {
		for v := start; v < end; v += step {
			rounded := math.Round(v*1e10) / 1e10
			result = append(result, rounded)
		}
	} else {
		for v := start; v > end; v += step {
			rounded := math.Round(v*1e10) / 1e10
			result = append(result, rounded)
		}
	}
	return result
}

// Zip combines elements from multiple arrays at corresponding positions.
// The result length equals the shortest input array length.
func Zip[T any](arrays ...[]T) [][]T {
	if len(arrays) == 0 {
		return [][]T{}
	}
	minLen := len(arrays[0])
	for _, arr := range arrays[1:] {
		if len(arr) < minLen {
			minLen = len(arr)
		}
	}
	result := make([][]T, minLen)
	for i := 0; i < minLen; i++ {
		row := make([]T, len(arrays))
		for j, arr := range arrays {
			row[j] = arr[i]
		}
		result[i] = row
	}
	return result
}

// ZipLongest combines elements from multiple arrays, padding shorter arrays
// with the given default value to match the longest array length.
func ZipLongest[T any](defaultVal T, arrays ...[]T) [][]T {
	if len(arrays) == 0 {
		return [][]T{}
	}
	maxLen := 0
	for _, arr := range arrays {
		if len(arr) > maxLen {
			maxLen = len(arr)
		}
	}
	result := make([][]T, maxLen)
	for i := 0; i < maxLen; i++ {
		row := make([]T, len(arrays))
		for j, arr := range arrays {
			if i < len(arr) {
				row[j] = arr[i]
			} else {
				row[j] = defaultVal
			}
		}
		result[i] = row
	}
	return result
}

// Unique removes duplicate values from an array, preserving the order of first occurrence.
func Unique[T comparable](arr []T) []T {
	seen := make(map[T]bool, len(arr))
	result := make([]T, 0, len(arr))
	for _, v := range arr {
		if !seen[v] {
			seen[v] = true
			result = append(result, v)
		}
	}
	return result
}

// UniqBy removes duplicate values from an array based on a key function.
// Preserves the order of first occurrence.
func UniqBy[T any, K comparable](arr []T, fn func(T) K) []T {
	seen := make(map[K]bool)
	result := make([]T, 0, len(arr))
	for _, v := range arr {
		key := fn(v)
		if !seen[key] {
			seen[key] = true
			result = append(result, v)
		}
	}
	return result
}

// Compact removes zero values from a slice.
// Zero values are: 0 for numbers, "" for strings, false for bools, nil for pointers.
func Compact[T comparable](arr []T) []T {
	var zero T
	result := make([]T, 0, len(arr))
	for _, v := range arr {
		if v != zero {
			result = append(result, v)
		}
	}
	return result
}

// Shuffle randomly reorders the elements of a slice using the Fisher-Yates algorithm.
// Returns a new shuffled slice without mutating the input.
func Shuffle[T any](arr []T) []T {
	result := make([]T, len(arr))
	copy(result, arr)
	for i := len(result) - 1; i > 0; i-- {
		j := rand.Intn(i + 1)
		result[i], result[j] = result[j], result[i]
	}
	return result
}

// Shuffle2DArray shuffles all elements across a 2D array while maintaining row lengths.
// Flattens, shuffles, then reconstructs with original row dimensions.
// Returns a new 2D slice without mutating the input.
func Shuffle2DArray[T any](arr [][]T) [][]T {
	if len(arr) == 0 {
		return [][]T{}
	}
	// Flatten
	total := 0
	for _, sub := range arr {
		total += len(sub)
	}
	flat := make([]T, 0, total)
	for _, sub := range arr {
		flat = append(flat, sub...)
	}
	// Shuffle flat array
	for i := len(flat) - 1; i > 0; i-- {
		j := rand.Intn(i + 1)
		flat[i], flat[j] = flat[j], flat[i]
	}
	// Reconstruct with original dimensions
	result := make([][]T, len(arr))
	idx := 0
	for i, sub := range arr {
		row := make([]T, len(sub))
		copy(row, flat[idx:idx+len(sub)])
		result[i] = row
		idx += len(sub)
	}
	return result
}
