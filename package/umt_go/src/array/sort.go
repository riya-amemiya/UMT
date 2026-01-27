package array

// InsertionSort sorts a slice using the insertion sort algorithm.
// It accepts an optional range [start, end] (inclusive) to sort only a portion.
// Returns a new sorted slice without mutating the input.
func InsertionSort[T any](arr []T, cmp func(T, T) int, startEnd ...int) []T {
	result := make([]T, len(arr))
	copy(result, arr)

	start := 0
	end := len(result) - 1
	if len(startEnd) >= 1 {
		start = startEnd[0]
	}
	if len(startEnd) >= 2 {
		end = startEnd[1]
	}

	if len(result) == 0 {
		return result
	}

	insertionSortRange(result, cmp, start, end)
	return result
}

// insertionSortRange sorts a range [start, end] (inclusive) of the slice in-place.
func insertionSortRange[T any](arr []T, cmp func(T, T) int, start, end int) {
	for i := start + 1; i <= end; i++ {
		j := i
		target := arr[i]
		for j > start && cmp(arr[j-1], target) > 0 {
			arr[j] = arr[j-1]
			j--
		}
		arr[j] = target
	}
}

// MergeSort sorts a slice using the merge sort algorithm.
// Returns a new sorted slice.
func MergeSort[T any](arr []T, cmp func(T, T) int) []T {
	if len(arr) <= 1 {
		result := make([]T, len(arr))
		copy(result, arr)
		return result
	}

	mid := len(arr) / 2
	left := MergeSort(arr[:mid], cmp)
	right := MergeSort(arr[mid:], cmp)

	return mergeSorted(left, right, cmp)
}

func mergeSorted[T any](left, right []T, cmp func(T, T) int) []T {
	result := make([]T, 0, len(left)+len(right))
	li, ri := 0, 0

	for li < len(left) && ri < len(right) {
		if cmp(left[li], right[ri]) <= 0 {
			result = append(result, left[li])
			li++
		} else {
			result = append(result, right[ri])
			ri++
		}
	}

	result = append(result, left[li:]...)
	result = append(result, right[ri:]...)
	return result
}

// QuickSort sorts a slice using a hybrid quicksort algorithm with insertion sort
// for small partitions. Accepts optional startIndex, endIndex, and insertionSortThreshold.
// Returns a new sorted slice.
func QuickSort[T any](arr []T, cmp func(T, T) int, opts ...int) []T {
	result := make([]T, len(arr))
	copy(result, arr)

	if len(result) == 0 {
		return result
	}

	startIndex := 0
	endIndex := len(result) - 1
	insertionSortThreshold := 10

	if len(opts) >= 1 {
		startIndex = opts[0]
	}
	if len(opts) >= 2 {
		endIndex = opts[1]
	}
	if len(opts) >= 3 {
		insertionSortThreshold = opts[2]
	}

	vs, ve, shouldSort := validateRange(result, startIndex, endIndex)
	if shouldSort {
		quickSortImpl(result, vs, ve, cmp, insertionSortThreshold)
	}
	return result
}

func validateRange[T any](arr []T, startIndex, endIndex int) (int, int, bool) {
	length := len(arr)
	if length == 0 {
		return 0, -1, false
	}

	vs := startIndex
	if vs < 0 {
		vs = 0
	}
	if vs > length-1 {
		vs = length - 1
	}

	ve := endIndex
	if ve < vs {
		ve = vs
	}
	if ve > length-1 {
		ve = length - 1
	}

	return vs, ve, ve >= vs
}

func quickSortMedianOfThree[T any](arr []T, a, b, c int, cmp func(T, T) int) T {
	ab := cmp(arr[a], arr[b])
	if ab < 0 {
		bc := cmp(arr[b], arr[c])
		if bc < 0 {
			return arr[b]
		}
		if cmp(arr[a], arr[c]) < 0 {
			return arr[c]
		}
		return arr[a]
	}
	ac := cmp(arr[a], arr[c])
	if ac < 0 {
		return arr[a]
	}
	if cmp(arr[b], arr[c]) < 0 {
		return arr[c]
	}
	return arr[b]
}

func quickSortPartition[T any](arr []T, low, high int, cmp func(T, T) int) int {
	pivot := quickSortMedianOfThree(arr, low, (low+high)/2, high, cmp)
	left := low - 1
	right := high + 1

	for {
		left++
		for cmp(arr[left], pivot) < 0 {
			left++
		}
		right--
		for cmp(arr[right], pivot) > 0 {
			right--
		}

		if left >= right {
			return right
		}

		arr[left], arr[right] = arr[right], arr[left]
	}
}

func quickSortImpl[T any](arr []T, low, high int, cmp func(T, T) int, threshold int) {
	for low < high {
		if high-low+1 <= threshold {
			insertionSortRange(arr, cmp, low, high)
			return
		}
		pivotIndex := quickSortPartition(arr, low, high, cmp)
		if pivotIndex-low < high-pivotIndex {
			quickSortImpl(arr, low, pivotIndex, cmp, threshold)
			low = pivotIndex + 1
		} else {
			quickSortImpl(arr, pivotIndex+1, high, cmp, threshold)
			high = pivotIndex
		}
	}
}

// DualPivotQuickSort sorts a slice using a dual-pivot quicksort algorithm.
// Accepts optional startIndex, endIndex, and insertionSortThreshold.
// Returns a new sorted slice.
func DualPivotQuickSort[T any](arr []T, cmp func(T, T) int, opts ...int) []T {
	result := make([]T, len(arr))
	copy(result, arr)

	if len(result) == 0 {
		return result
	}

	startIndex := 0
	endIndex := len(result) - 1
	insertionSortThreshold := 10

	if len(opts) >= 1 {
		startIndex = opts[0]
	}
	if len(opts) >= 2 {
		endIndex = opts[1]
	}
	if len(opts) >= 3 {
		insertionSortThreshold = opts[2]
	}

	vs, ve, shouldSort := validateRange(result, startIndex, endIndex)
	if shouldSort {
		dualPivotSortRange(result, vs, ve, cmp, insertionSortThreshold)
	}
	return result
}

func dualPivotMedianOfThree[T any](arr []T, a, b, c int, cmp func(T, T) int) int {
	type indexedVal struct {
		index int
	}
	vals := [3]indexedVal{{a}, {b}, {c}}

	// Simple sort of 3 elements
	if cmp(arr[vals[0].index], arr[vals[1].index]) > 0 {
		vals[0], vals[1] = vals[1], vals[0]
	}
	if cmp(arr[vals[1].index], arr[vals[2].index]) > 0 {
		vals[1], vals[2] = vals[2], vals[1]
	}
	if cmp(arr[vals[0].index], arr[vals[1].index]) > 0 {
		vals[0], vals[1] = vals[1], vals[0]
	}
	return vals[1].index
}

type dualPivotPartitionResult struct {
	leftPivotIndex  int
	rightPivotIndex int
}

func dualPivotPartition[T any](arr []T, low, high int, cmp func(T, T) int) dualPivotPartitionResult {
	length := high - low
	gap := length / 3
	if gap < 1 {
		gap = 1
	}

	leftCand := low + 2*gap
	if leftCand > high {
		leftCand = high
	}
	leftPivotIdx := dualPivotMedianOfThree(arr, low, low+gap, leftCand, cmp)

	rightCandStart := high - 2*gap
	if rightCandStart < low {
		rightCandStart = low
	}
	rightPivotIdx := dualPivotMedianOfThree(arr, rightCandStart, high-gap, high, cmp)

	// Move pivots to ends
	if leftPivotIdx != low {
		arr[low], arr[leftPivotIdx] = arr[leftPivotIdx], arr[low]
	}
	if rightPivotIdx != high {
		arr[high], arr[rightPivotIdx] = arr[rightPivotIdx], arr[high]
	}

	// Swap if left pivot > right pivot
	if cmp(arr[low], arr[high]) > 0 {
		arr[low], arr[high] = arr[high], arr[low]
	}

	left := low + 1
	right := high - 1
	current := left

	for current <= right {
		if cmp(arr[current], arr[low]) < 0 {
			arr[current], arr[left] = arr[left], arr[current]
			left++
		} else if cmp(arr[current], arr[high]) > 0 {
			for current < right && cmp(arr[right], arr[high]) > 0 {
				right--
			}
			arr[current], arr[right] = arr[right], arr[current]
			right--
			if cmp(arr[current], arr[low]) < 0 {
				arr[current], arr[left] = arr[left], arr[current]
				left++
			}
		}
		current++
	}

	left--
	right++
	arr[low], arr[left] = arr[left], arr[low]
	arr[high], arr[right] = arr[right], arr[high]

	return dualPivotPartitionResult{leftPivotIndex: left, rightPivotIndex: right}
}

func dualPivotSortRange[T any](arr []T, start, end int, cmp func(T, T) int, threshold int) {
	if end-start+1 <= threshold {
		insertionSortRange(arr, cmp, start, end)
		return
	}
	if start >= end {
		return
	}

	pr := dualPivotPartition(arr, start, end, cmp)

	dualPivotSortRange(arr, start, pr.leftPivotIndex-1, cmp, threshold)
	if pr.rightPivotIndex-pr.leftPivotIndex > 1 {
		dualPivotSortRange(arr, pr.leftPivotIndex+1, pr.rightPivotIndex-1, cmp, threshold)
	}
	dualPivotSortRange(arr, pr.rightPivotIndex+1, end, cmp, threshold)
}

const minRun = 32

// TimSort sorts a slice using the TimSort algorithm, which combines insertion
// sort and merge sort. Returns a new sorted slice.
// Accepts optional start and end indices.
func TimSort[T any](arr []T, cmp func(T, T) int, startEnd ...int) []T {
	result := make([]T, len(arr))
	copy(result, arr)

	if len(result) == 0 {
		return result
	}

	start := 0
	end := len(result) - 1
	if len(startEnd) >= 1 {
		start = startEnd[0]
	}
	if len(startEnd) >= 2 {
		end = startEnd[1]
	}

	n := end - start + 1
	minRunLen := getMinRunLength(n)

	for runStart := start; runStart <= end; runStart += minRunLen {
		runEnd := runStart + minRunLen - 1
		if runEnd > end {
			runEnd = end
		}
		insertionSortRange(result, cmp, runStart, runEnd)
	}

	for size := minRunLen; size < n; size *= 2 {
		for left := start; left <= end; left += 2 * size {
			mid := left + size - 1
			right := left + 2*size - 1
			if right > end {
				right = end
			}
			if mid < right {
				timSortMerge(result, left, mid, right, cmp)
			}
		}
	}

	return result
}

func getMinRunLength(n int) int {
	r := 0
	for n >= minRun {
		r |= n & 1
		n >>= 1
	}
	return n + r
}

func timSortMerge[T any](arr []T, start, mid, end int, cmp func(T, T) int) {
	left := make([]T, mid-start+1)
	copy(left, arr[start:mid+1])
	right := make([]T, end-mid)
	copy(right, arr[mid+1:end+1])

	li, ri, ai := 0, 0, start

	for li < len(left) && ri < len(right) {
		if cmp(left[li], right[ri]) <= 0 {
			arr[ai] = left[li]
			li++
		} else {
			arr[ai] = right[ri]
			ri++
		}
		ai++
	}

	for li < len(left) {
		arr[ai] = left[li]
		li++
		ai++
	}

	for ri < len(right) {
		arr[ai] = right[ri]
		ri++
		ai++
	}
}
