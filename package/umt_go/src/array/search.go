package array

import (
	"errors"
	"math"
	"math/rand"
)

// BinarySearch performs binary search on a sorted array using the provided compare function.
// Returns the index of the target, or -1 if not found.
func BinarySearch[T any](arr []T, target T, compare func(a, b T) int) int {
	left := 0
	right := len(arr) - 1
	for left <= right {
		mid := left + (right-left)/2
		cmp := compare(arr[mid], target)
		if cmp == 0 {
			return mid
		}
		if cmp < 0 {
			left = mid + 1
		} else {
			right = mid - 1
		}
	}
	return -1
}

// GetArraysCommon returns the intersection of multiple arrays.
// Elements are unique in the result and preserve the order from the first array.
func GetArraysCommon[T comparable](arrays ...[]T) []T {
	if len(arrays) == 0 {
		return []T{}
	}
	if len(arrays) == 1 {
		result := make([]T, len(arrays[0]))
		copy(result, arrays[0])
		return result
	}

	first := arrays[0]
	others := arrays[1:]

	// Build sets from all arrays except the first
	otherSets := make([]map[T]bool, len(others))
	for i, arr := range others {
		s := make(map[T]bool, len(arr))
		for _, v := range arr {
			s[v] = true
		}
		otherSets[i] = s
	}

	seen := make(map[T]bool)
	result := []T{}

	for _, item := range first {
		if seen[item] {
			continue
		}
		common := true
		for _, s := range otherSets {
			if !s[item] {
				common = false
				break
			}
		}
		if common {
			seen[item] = true
			result = append(result, item)
		}
	}

	return result
}

// GetArraysDiff returns elements that appear in only one of the two arrays (symmetric difference).
// Preserves order: elements from a first, then elements from b.
func GetArraysDiff[T comparable](a, b []T) []T {
	allValues := make(map[T]bool)
	duplicates := make(map[T]bool)
	order := []T{}

	for _, v := range a {
		if !allValues[v] {
			allValues[v] = true
			order = append(order, v)
		}
	}

	for _, v := range b {
		if allValues[v] {
			duplicates[v] = true
		} else {
			allValues[v] = true
			order = append(order, v)
		}
	}

	result := []T{}
	for _, v := range order {
		if !duplicates[v] {
			result = append(result, v)
		}
	}

	return result
}

// ArraysJoin concatenates multiple arrays into a single array.
func ArraysJoin[T any](arrays ...[]T) []T {
	total := 0
	for _, arr := range arrays {
		total += len(arr)
	}
	result := make([]T, 0, total)
	for _, arr := range arrays {
		result = append(result, arr...)
	}
	return result
}

// GroupBy groups elements of an array by a key function.
// Returns a map from keys to slices of elements with that key.
func GroupBy[T any, K comparable](arr []T, fn func(T) K) map[K][]T {
	result := make(map[K][]T)
	for _, v := range arr {
		key := fn(v)
		result[key] = append(result[key], v)
	}
	return result
}

// RandomSelect randomly selects a single element from the array.
// Returns an error if the array is empty.
func RandomSelect[T any](arr []T) (T, error) {
	if len(arr) == 0 {
		var zero T
		return zero, errors.New("array is empty")
	}
	return arr[rand.Intn(len(arr))], nil
}

// Sum returns the sum of all elements in a float64 slice.
func Sum(arr []float64) float64 {
	total := 0.0
	for _, v := range arr {
		total += v
	}
	return total
}

// GenerateNumberArray generates a sequence of float64 values from start (inclusive)
// to end (exclusive) with the given step. Handles floating point precision.
func GenerateNumberArray(start, end, step float64) []float64 {
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

// CheckFlagAlignment checks if a bitwise flag is set within a value.
// Returns true if value & flag == flag.
func CheckFlagAlignment(value, flag int) bool {
	return value&flag == flag
}
