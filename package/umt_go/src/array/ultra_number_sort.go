package array

import "math"

// UltraNumberSort performs an ultra-fast sort specifically optimized for
// float64 slices. It automatically selects between counting sort, radix sort,
// and quicksort depending on the data characteristics.
// The ascending parameter controls sort order (true = ascending, false = descending).
// Returns a new sorted slice; the original is not modified.
func UltraNumberSort(array []float64, ascending bool) []float64 {
	result := make([]float64, len(array))
	copy(result, array)
	length := len(result)

	if length <= 1 {
		return result
	}

	// For tiny arrays, use optimized inline sort
	if length == 2 {
		if (result[0] > result[1]) == ascending {
			result[0], result[1] = result[1], result[0]
		}
		return result
	}

	if length == 3 {
		ultraInlineSort3(result, ascending)
		return result
	}

	// Check if all numbers are integers and find range
	allIntegers := true
	min := result[0]
	max := result[0]
	hasNaN := false

	for i := 0; i < length; i++ {
		value := result[i]
		if math.IsNaN(value) {
			hasNaN = true
			break
		}
		if value < min {
			min = value
		}
		if value > max {
			max = value
		}
		if allIntegers && value != math.Floor(value) {
			allIntegers = false
		}
	}

	// Handle NaN values
	if hasNaN {
		return ultraHandleNaNSort(result, ascending)
	}

	// For small integer ranges, use counting sort
	rangeSize := max - min
	if allIntegers && rangeSize < float64(length*2) && rangeSize < 1_000_000 {
		return ultraCountingSort(result, min, max, ascending)
	}

	// For larger arrays, use radix sort if applicable
	if allIntegers && length > 100 {
		return ultraRadixSort(result, ascending)
	}

	// Fall back to optimized quicksort for floating point
	return ultraNumericQuickSort(result, 0, length-1, ascending)
}

// ultraInlineSort3 sorts 3 elements in place.
func ultraInlineSort3(array []float64, ascending bool) {
	a, b, c := array[0], array[1], array[2]

	if ascending {
		if a > b {
			a, b = b, a
		}
		if b > c {
			b, c = c, b
			if a > b {
				a, b = b, a
			}
		}
	} else {
		if a < b {
			a, b = b, a
		}
		if b < c {
			b, c = c, b
			if a < b {
				a, b = b, a
			}
		}
	}

	array[0] = a
	array[1] = b
	array[2] = c
}

// ultraHandleNaNSort sorts an array that contains NaN values.
// NaN values are placed at the end.
func ultraHandleNaNSort(array []float64, ascending bool) []float64 {
	var valid []float64
	nanCount := 0

	for _, elem := range array {
		if !math.IsNaN(elem) {
			valid = append(valid, elem)
		} else {
			nanCount++
		}
	}

	if len(valid) > 0 {
		ultraNumericQuickSort(valid, 0, len(valid)-1, ascending)
	}

	// NaN values go to the end
	for i := 0; i < nanCount; i++ {
		valid = append(valid, math.NaN())
	}

	// Copy back
	for i := 0; i < len(array); i++ {
		array[i] = valid[i]
	}

	return array
}

// ultraCountingSort is a counting sort for small integer ranges.
func ultraCountingSort(array []float64, min, max float64, ascending bool) []float64 {
	rangeSize := int(max-min) + 1
	count := make([]uint32, rangeSize)

	// Count occurrences
	for _, elem := range array {
		count[int(elem-min)]++
	}

	// Reconstruct array
	idx := 0
	if ascending {
		for i := 0; i < rangeSize; i++ {
			cnt := count[i]
			value := float64(i) + min
			for j := uint32(0); j < cnt; j++ {
				array[idx] = value
				idx++
			}
		}
	} else {
		for i := rangeSize - 1; i >= 0; i-- {
			cnt := count[i]
			value := float64(i) + min
			for j := uint32(0); j < cnt; j++ {
				array[idx] = value
				idx++
			}
		}
	}

	return array
}

// ultraRadixSort performs a radix sort for integer values stored as float64.
func ultraRadixSort(array []float64, ascending bool) []float64 {
	length := len(array)

	// Separate positive and negative numbers
	var positive []int
	var negative []int
	zeroCount := 0

	for i := 0; i < length; i++ {
		v := int(array[i])
		if v > 0 {
			positive = append(positive, v)
		} else if v < 0 {
			negative = append(negative, -v)
		} else {
			zeroCount++
		}
	}

	// Sort positive numbers
	if len(positive) > 0 {
		ultraRadixSortPositive(positive)
	}

	// Sort negative numbers
	if len(negative) > 0 {
		ultraRadixSortPositive(negative)
	}

	// Merge results
	idx := 0
	if ascending {
		// Negative numbers first (in reverse order)
		for i := len(negative) - 1; i >= 0; i-- {
			array[idx] = float64(-negative[i])
			idx++
		}
		// Zeros
		for i := 0; i < zeroCount; i++ {
			array[idx] = 0
			idx++
		}
		// Positive numbers
		for _, elem := range positive {
			array[idx] = float64(elem)
			idx++
		}
	} else {
		// Positive numbers first (in reverse order)
		for i := len(positive) - 1; i >= 0; i-- {
			array[idx] = float64(positive[i])
			idx++
		}
		// Zeros
		for i := 0; i < zeroCount; i++ {
			array[idx] = 0
			idx++
		}
		// Negative numbers
		for _, elem := range negative {
			array[idx] = float64(-elem)
			idx++
		}
	}

	return array
}

// ultraRadixSortPositive performs radix sort on positive integers (LSD, base 256).
func ultraRadixSortPositive(array []int) {
	length := len(array)
	if length <= 1 {
		return
	}

	// Find maximum to determine number of digits
	max := array[0]
	for i := 1; i < length; i++ {
		if array[i] > max {
			max = array[i]
		}
	}

	output := make([]int, length)
	var count [256]uint32

	// Process 8 bits at a time
	for shift := 0; max>>shift > 0; shift += 8 {
		// Reset count array
		for i := range count {
			count[i] = 0
		}

		// Count occurrences
		for i := 0; i < length; i++ {
			digit := (array[i] >> shift) & 0xff
			count[digit]++
		}

		// Change count[i] to actual position
		for i := 1; i < 256; i++ {
			count[i] += count[i-1]
		}

		// Build output array
		for i := length - 1; i >= 0; i-- {
			digit := (array[i] >> shift) & 0xff
			count[digit]--
			output[count[digit]] = array[i]
		}

		// Copy back
		copy(array, output)
	}
}

// ultraNumericQuickSort is an optimized iterative quicksort for numbers.
func ultraNumericQuickSort(array []float64, low, high int, ascending bool) []float64 {
	stack := []int{low, high}

	for len(stack) > 0 {
		h := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		l := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if h <= l {
			continue
		}

		// For small subarrays, use insertion sort
		if h-l < 16 {
			ultraNumericInsertionSort(array, l, h, ascending)
			continue
		}

		// Partition
		pivot := ultraNumericPartition(array, l, h, ascending)

		// Push larger partition first to limit stack depth
		if pivot-l > h-pivot {
			stack = append(stack, l, pivot-1, pivot+1, h)
		} else {
			stack = append(stack, pivot+1, h, l, pivot-1)
		}
	}

	return array
}

// ultraNumericInsertionSort sorts a small range of the array.
func ultraNumericInsertionSort(array []float64, low, high int, ascending bool) {
	if ascending {
		for i := low + 1; i <= high; i++ {
			key := array[i]
			j := i - 1
			for j >= low && array[j] > key {
				array[j+1] = array[j]
				j--
			}
			array[j+1] = key
		}
	} else {
		for i := low + 1; i <= high; i++ {
			key := array[i]
			j := i - 1
			for j >= low && array[j] < key {
				array[j+1] = array[j]
				j--
			}
			array[j+1] = key
		}
	}
}

// ultraNumericPartition partitions the array using median-of-three pivot selection.
func ultraNumericPartition(array []float64, low, high int, ascending bool) int {
	mid := low + ((high - low) >> 1)

	if ascending {
		if array[mid] < array[low] {
			array[low], array[mid] = array[mid], array[low]
		}
		if array[high] < array[low] {
			array[low], array[high] = array[high], array[low]
		}
		if array[high] < array[mid] {
			array[mid], array[high] = array[high], array[mid]
		}
	} else {
		if array[mid] > array[low] {
			array[low], array[mid] = array[mid], array[low]
		}
		if array[high] > array[low] {
			array[low], array[high] = array[high], array[low]
		}
		if array[high] > array[mid] {
			array[mid], array[high] = array[high], array[mid]
		}
	}

	// Move pivot to high-1
	array[mid], array[high-1] = array[high-1], array[mid]
	pivot := array[high-1]

	i := low
	j := high - 1

	if ascending {
		for {
			i++
			for array[i] < pivot {
				i++
			}
			j--
			for array[j] > pivot {
				j--
			}
			if i >= j {
				break
			}
			array[i], array[j] = array[j], array[i]
		}
	} else {
		for {
			i++
			for array[i] > pivot {
				i++
			}
			j--
			for array[j] < pivot {
				j--
			}
			if i >= j {
				break
			}
			array[i], array[j] = array[j], array[i]
		}
	}

	array[i], array[high-1] = array[high-1], array[i]
	return i
}
