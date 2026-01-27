package array_test

import (
	"math"
	"math/rand"
	"reflect"
	"sort"
	"testing"

	"github.com/riya-amemiya/umt-go/src/array"
)

// Helper comparison functions for tests

func intCmp(a, b int) int {
	if a < b {
		return -1
	}
	if a > b {
		return 1
	}
	return 0
}

func intCmpDesc(a, b int) int {
	return intCmp(b, a)
}

func stringCmp(a, b string) int {
	if a < b {
		return -1
	}
	if a > b {
		return 1
	}
	return 0
}

// =============================================================================
// Tests from sort_test.go
// =============================================================================

func TestQuickSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := array.QuickSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := array.QuickSort([]int{1, 2, 3, 4, 5}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := array.QuickSort([]int{5, 4, 3, 2, 1}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		result := array.QuickSort([]int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := array.QuickSort([]int{3, 3, 3, 2, 1, 1, 4, 4, 5}, intCmp)
		expected := []int{1, 1, 2, 3, 3, 3, 4, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := array.QuickSort([]int{5, -1, 3, 2, 4, -5, 1, -2, 0}, intCmp)
		expected := []int{-5, -2, -1, 0, 1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		result := array.QuickSort([]int{1, 2, 3, 4, 5}, intCmpDesc)
		expected := []int{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := array.QuickSort([]int{1}, intCmp)
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all equal", func(t *testing.T) {
		result := array.QuickSort([]int{5, 5, 5, 5, 5}, intCmp)
		expected := []int{5, 5, 5, 5, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large array", func(t *testing.T) {
		large := make([]int, 10000)
		for i := range large {
			large[i] = rand.Intn(10000)
		}
		result := array.QuickSort(large, intCmp)
		sorted := make([]int, len(large))
		copy(sorted, large)
		sort.Ints(sorted)
		if !reflect.DeepEqual(result, sorted) {
			t.Error("Large array sort failed")
		}
	})

	t.Run("two element arrays", func(t *testing.T) {
		result := array.QuickSort([]int{2, 1}, intCmp)
		expected := []int{1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}

		result = array.QuickSort([]int{1, 2}, intCmp)
		expected = []int{1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		original := []int{3, 1, 4, 1, 5}
		originalCopy := make([]int, len(original))
		copy(originalCopy, original)
		array.QuickSort(original, intCmp)
		if !reflect.DeepEqual(original, originalCopy) {
			t.Error("Original array was mutated")
		}
	})
}

func TestMergeSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := array.MergeSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := array.MergeSort([]int{1, 2, 3}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := array.MergeSort([]int{3, 2, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		result := array.MergeSort([]int{2, 3, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := array.MergeSort([]int{2, 3, 3, 1, 2}, intCmp)
		expected := []int{1, 2, 2, 3, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := array.MergeSort([]int{1}, intCmp)
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := array.MergeSort([]int{3, -1, 4, -5, 2, -3}, intCmp)
		expected := []int{-5, -3, -1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings alphabetically", func(t *testing.T) {
		result := array.MergeSort([]string{"banana", "apple", "cherry"}, stringCmp)
		expected := []string{"apple", "banana", "cherry"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("custom comparison descending", func(t *testing.T) {
		result := array.MergeSort([]int{1, 3, 2}, intCmpDesc)
		expected := []int{3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings by length", func(t *testing.T) {
		lenCmp := func(a, b string) int {
			return len(a) - len(b)
		}
		result := array.MergeSort([]string{"aaa", "a", "aa"}, lenCmp)
		expected := []string{"a", "aa", "aaa"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestTimSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := array.TimSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := array.TimSort([]int{1, 2, 3}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := array.TimSort([]int{3, 2, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
		result := array.TimSort(arr, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := array.TimSort([]int{2, 3, 3, 1, 2}, intCmp)
		expected := []int{1, 2, 2, 3, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large array", func(t *testing.T) {
		large := make([]int, 10000)
		for i := range large {
			large[i] = rand.Intn(10000)
		}
		result := array.TimSort(large, intCmp)
		sorted := make([]int, len(large))
		copy(sorted, large)
		sort.Ints(sorted)
		if !reflect.DeepEqual(result, sorted) {
			t.Error("Large array sort failed")
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := array.TimSort([]int{5, -1, 3, 2, 4, -5, 1, -2, 0}, intCmp)
		expected := []int{-5, -2, -1, 0, 1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		result := array.TimSort([]int{1, 2, 3}, intCmpDesc)
		expected := []int{3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("partial sort", func(t *testing.T) {
		result := array.TimSort([]int{1, 3, 2, 5, 4}, intCmp, 1, 3)
		expected := []int{1, 2, 3, 5, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("array just larger than MIN_RUN", func(t *testing.T) {
		arr := make([]int, 33)
		for i := range arr {
			arr[i] = 33 - i
		}
		result := array.TimSort(arr, intCmp)
		expected := make([]int, 33)
		for i := range expected {
			expected[i] = i + 1
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all identical elements", func(t *testing.T) {
		arr := make([]int, 100)
		for i := range arr {
			arr[i] = 1
		}
		result := array.TimSort(arr, intCmp)
		if !reflect.DeepEqual(result, arr) {
			t.Error("All identical elements should remain unchanged")
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := array.TimSort([]string{"b", "a", "d", "c"}, stringCmp)
		expected := []string{"a", "b", "c", "d"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestDualPivotQuickSort(t *testing.T) {
	t.Run("ascending numbers", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3}
		result := array.DualPivotQuickSort(arr, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings lexicographically", func(t *testing.T) {
		result := array.DualPivotQuickSort([]string{"banana", "apple", "orange", "grape"}, stringCmp)
		expected := []string{"apple", "banana", "grape", "orange"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3}
		result := array.DualPivotQuickSort(arr, intCmpDesc)
		expected := []int{9, 6, 5, 5, 4, 3, 3, 2, 1, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.DualPivotQuickSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := array.DualPivotQuickSort([]int{1}, intCmp)
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all identical", func(t *testing.T) {
		result := array.DualPivotQuickSort([]int{2, 2, 2, 2, 2}, intCmp)
		expected := []int{2, 2, 2, 2, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := array.DualPivotQuickSort([]int{1, 2, 3, 4, 5}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := array.DualPivotQuickSort([]int{5, 4, 3, 2, 1}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		arr := []int{-3, 1, -4, 1, 5, -9, 2, 6, 5, 3}
		result := array.DualPivotQuickSort(arr, intCmp)
		expected := []int{-9, -4, -3, 1, 1, 2, 3, 5, 5, 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large array", func(t *testing.T) {
		size := 1000
		arr := make([]int, size)
		for i := range arr {
			arr[i] = rand.Intn(size)
		}
		result := array.DualPivotQuickSort(arr, intCmp)
		for i := 1; i < len(result); i++ {
			if result[i] < result[i-1] {
				t.Errorf("Element at index %d (%d) is less than element at index %d (%d)", i, result[i], i-1, result[i-1])
			}
		}
	})

	t.Run("sort objects by field", func(t *testing.T) {
		type item struct {
			id    int
			value string
		}
		arr := []item{{3, "c"}, {1, "a"}, {2, "b"}}
		cmp := func(a, b item) int { return a.id - b.id }
		result := array.DualPivotQuickSort(arr, cmp)
		expected := []item{{1, "a"}, {2, "b"}, {3, "c"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestInsertionSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := array.InsertionSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := array.InsertionSort([]int{1, 2, 3}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := array.InsertionSort([]int{3, 2, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
		result := array.InsertionSort(arr, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := array.InsertionSort([]int{2, 3, 3, 1, 2}, intCmp)
		expected := []int{1, 2, 2, 3, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := array.InsertionSort([]int{5, -1, 3, 2, 4, -5, 1, -2, 0}, intCmp)
		expected := []int{-5, -2, -1, 0, 1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		result := array.InsertionSort([]int{1, 2, 3}, intCmpDesc)
		expected := []int{3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("partial sort", func(t *testing.T) {
		result := array.InsertionSort([]int{1, 3, 2, 5, 4}, intCmp, 1, 3)
		expected := []int{1, 2, 3, 5, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// =============================================================================
// Tests from operations_test.go
// =============================================================================

func TestChunk(t *testing.T) {
	t.Run("splits array into chunks", func(t *testing.T) {
		result := array.Chunk([]int{0, 1, 2, 3, 4, 5, 6, 7}, 3)
		expected := [][]int{{0, 1, 2}, {3, 4, 5}, {6, 7}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.Chunk([]int{}, 3)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("chunk size larger than array", func(t *testing.T) {
		result := array.Chunk([]int{0, 1, 2}, 5)
		expected := [][]int{{0, 1, 2}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("chunk size of 1", func(t *testing.T) {
		result := array.Chunk([]int{1, 2, 3, 4, 5}, 1)
		expected := [][]int{{1}, {2}, {3}, {4}, {5}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("perfect division", func(t *testing.T) {
		result := array.Chunk([]int{1, 2, 3, 4, 5, 6}, 3)
		expected := [][]int{{1, 2, 3}, {4, 5, 6}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("array equal to chunk size", func(t *testing.T) {
		result := array.Chunk([]int{1, 2, 3}, 3)
		expected := [][]int{{1, 2, 3}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := array.Chunk([]int{42}, 1)
		expected := [][]int{{42}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("string elements", func(t *testing.T) {
		result := array.Chunk([]string{"a", "b", "c", "d", "e", "f", "g"}, 3)
		expected := [][]string{{"a", "b", "c"}, {"d", "e", "f"}, {"g"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("does not mutate input", func(t *testing.T) {
		input := []int{0, 1, 2, 3, 4, 5, 6, 7}
		original := make([]int, len(input))
		copy(original, input)
		array.Chunk(input, 3)
		if !reflect.DeepEqual(input, original) {
			t.Error("Input was mutated")
		}
	})

	t.Run("large array", func(t *testing.T) {
		input := make([]int, 10000)
		for i := range input {
			input[i] = i
		}
		result := array.Chunk(input, 1000)
		if len(result) != 10 {
			t.Errorf("Expected 10 chunks, got %d", len(result))
		}
		if result[0][0] != 0 {
			t.Errorf("Expected first element 0, got %d", result[0][0])
		}
		if result[9][999] != 9999 {
			t.Errorf("Expected last element 9999, got %d", result[9][999])
		}
	})
}

func TestDrop(t *testing.T) {
	t.Run("drop from start", func(t *testing.T) {
		result := array.Drop([]int{1, 2, 3, 4, 5}, 2)
		expected := []int{3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("drop 1 by default logic", func(t *testing.T) {
		result := array.Drop([]int{1, 2, 3, 4, 5}, 1)
		expected := []int{2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("drop more than length", func(t *testing.T) {
		result := array.Drop([]int{1, 2, 3}, 4)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("drop equal to length", func(t *testing.T) {
		result := array.Drop([]int{1, 2, 3}, 3)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("drop zero", func(t *testing.T) {
		result := array.Drop([]int{1, 2, 3}, 0)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("drop negative", func(t *testing.T) {
		result := array.Drop([]int{1, 2, 3}, -1)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.Drop([]int{}, 1)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})
}

func TestFirst(t *testing.T) {
	t.Run("non-empty int array", func(t *testing.T) {
		val, ok := array.First([]int{1, 2, 3})
		if !ok || val != 1 {
			t.Errorf("Expected (1, true), got (%d, %v)", val, ok)
		}
	})

	t.Run("non-empty string array", func(t *testing.T) {
		val, ok := array.First([]string{"a", "b", "c"})
		if !ok || val != "a" {
			t.Errorf("Expected (a, true), got (%s, %v)", val, ok)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		_, ok := array.First([]int{})
		if ok {
			t.Error("Expected false for empty array")
		}
	})

	t.Run("single element", func(t *testing.T) {
		val, ok := array.First([]int{42})
		if !ok || val != 42 {
			t.Errorf("Expected (42, true), got (%d, %v)", val, ok)
		}
	})

	t.Run("bool array", func(t *testing.T) {
		val, ok := array.First([]bool{true, false, true})
		if !ok || val != true {
			t.Errorf("Expected (true, true), got (%v, %v)", val, ok)
		}
	})

	t.Run("zero value first element", func(t *testing.T) {
		val, ok := array.First([]int{0, 1, 2})
		if !ok || val != 0 {
			t.Errorf("Expected (0, true), got (%d, %v)", val, ok)
		}
	})

	t.Run("false first element", func(t *testing.T) {
		val, ok := array.First([]bool{false, true})
		if !ok || val != false {
			t.Errorf("Expected (false, true), got (%v, %v)", val, ok)
		}
	})

	t.Run("empty string first element", func(t *testing.T) {
		val, ok := array.First([]string{"", "hello"})
		if !ok || val != "" {
			t.Errorf("Expected ('', true), got (%s, %v)", val, ok)
		}
	})
}

func TestPop(t *testing.T) {
	t.Run("non-empty array", func(t *testing.T) {
		rest, val, ok := array.Pop([]int{1, 2, 3})
		if !ok || val != 3 {
			t.Errorf("Expected (3, true), got (%d, %v)", val, ok)
		}
		expected := []int{1, 2}
		if !reflect.DeepEqual(rest, expected) {
			t.Errorf("Expected rest %v, got %v", expected, rest)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		rest, _, ok := array.Pop([]int{})
		if ok {
			t.Error("Expected false for empty array")
		}
		if len(rest) != 0 {
			t.Errorf("Expected empty rest, got %v", rest)
		}
	})

	t.Run("single element", func(t *testing.T) {
		rest, val, ok := array.Pop([]int{42})
		if !ok || val != 42 {
			t.Errorf("Expected (42, true), got (%d, %v)", val, ok)
		}
		if len(rest) != 0 {
			t.Errorf("Expected empty rest, got %v", rest)
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		original := []int{1, 2, 3}
		originalCopy := make([]int, len(original))
		copy(originalCopy, original)
		array.Pop(original)
		if !reflect.DeepEqual(original, originalCopy) {
			t.Error("Original was mutated")
		}
	})

	t.Run("string array", func(t *testing.T) {
		rest, val, ok := array.Pop([]string{"a", "b", "c"})
		if !ok || val != "c" {
			t.Errorf("Expected (c, true), got (%s, %v)", val, ok)
		}
		expected := []string{"a", "b"}
		if !reflect.DeepEqual(rest, expected) {
			t.Errorf("Expected rest %v, got %v", expected, rest)
		}
	})
}

func TestRange(t *testing.T) {
	t.Run("positive range", func(t *testing.T) {
		result := array.Range(1, 10)
		expected := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative to positive", func(t *testing.T) {
		result := array.Range(-5, 5)
		expected := []int{-5, -4, -3, -2, -1, 0, 1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("zero to N", func(t *testing.T) {
		result := array.Range(0, 5)
		expected := []int{0, 1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("same start and end", func(t *testing.T) {
		result := array.Range(0, 0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
		result = array.Range(5, 5)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("start greater than end", func(t *testing.T) {
		result := array.Range(10, 5)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})
}

func TestRangeFloat(t *testing.T) {
	t.Run("positive step", func(t *testing.T) {
		result := array.RangeFloat(0, 10, 2)
		expected := []float64{0, 2, 4, 6, 8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("fractional step", func(t *testing.T) {
		result := array.RangeFloat(0, 5, 0.5)
		expected := []float64{0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative step", func(t *testing.T) {
		result := array.RangeFloat(5, 0, -1)
		expected := []float64{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("zero step", func(t *testing.T) {
		result := array.RangeFloat(0, 5, 0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("start greater than end with positive step", func(t *testing.T) {
		result := array.RangeFloat(10, 5, 1)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("floating point precision", func(t *testing.T) {
		result := array.RangeFloat(0, 1, 0.2)
		expected := []float64{0, 0.2, 0.4, 0.6, 0.8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative range with negative step", func(t *testing.T) {
		result := array.RangeFloat(0, -5, -1)
		expected := []float64{0, -1, -2, -3, -4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestZip(t *testing.T) {
	t.Run("equal length arrays", func(t *testing.T) {
		result := array.Zip([]int{1, 2, 3}, []int{4, 5, 6}, []int{7, 8, 9})
		expected := [][]int{{1, 4, 7}, {2, 5, 8}, {3, 6, 9}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("different length arrays", func(t *testing.T) {
		result := array.Zip([]int{1, 2}, []int{3, 4, 5})
		expected := [][]int{{1, 3}, {2, 4}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with empty array", func(t *testing.T) {
		result := array.Zip([]int{1, 2, 3}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := array.Zip[int]()
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string arrays", func(t *testing.T) {
		result := array.Zip([]string{"a", "b"}, []string{"x", "y"})
		expected := [][]string{{"a", "x"}, {"b", "y"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestZipLongest(t *testing.T) {
	t.Run("different length arrays with default", func(t *testing.T) {
		result := array.ZipLongest(0, []int{1, 2}, []int{3, 4, 5})
		expected := [][]int{{1, 3}, {2, 4}, {0, 5}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("equal length arrays", func(t *testing.T) {
		result := array.ZipLongest(0, []int{1, 2, 3}, []int{4, 5, 6})
		expected := [][]int{{1, 4}, {2, 5}, {3, 6}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with empty array", func(t *testing.T) {
		result := array.ZipLongest(0, []int{1, 2, 3}, []int{})
		expected := [][]int{{1, 0}, {2, 0}, {3, 0}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := array.ZipLongest[int](0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string default value", func(t *testing.T) {
		result := array.ZipLongest("N/A", []string{"a", "b"}, []string{"x", "y", "z"})
		expected := [][]string{{"a", "x"}, {"b", "y"}, {"N/A", "z"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("three arrays with different lengths", func(t *testing.T) {
		result := array.ZipLongest(0, []int{1, 2}, []int{3, 4, 5}, []int{6})
		expected := [][]int{{1, 3, 6}, {2, 4, 0}, {0, 5, 0}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestUnique(t *testing.T) {
	t.Run("remove duplicates", func(t *testing.T) {
		result := array.Unique([]int{1, 2, 2, 3, 3, 3, 4})
		expected := []int{1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.Unique([]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("no duplicates", func(t *testing.T) {
		result := array.Unique([]int{1, 2, 3, 4})
		expected := []int{1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := array.Unique([]string{"a", "b", "b", "c", "a"})
		expected := []string{"a", "b", "c"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all same values", func(t *testing.T) {
		result := array.Unique([]int{1, 1, 1, 1})
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("preserves first occurrence order", func(t *testing.T) {
		result := array.Unique([]int{3, 1, 2, 1, 3, 2})
		expected := []int{3, 1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("booleans", func(t *testing.T) {
		result := array.Unique([]bool{true, false, true, false, true})
		expected := []bool{true, false}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestUniqBy(t *testing.T) {
	t.Run("by struct field", func(t *testing.T) {
		type person struct {
			ID   int
			Name string
		}
		data := []person{
			{1, "Alice"},
			{2, "Bob"},
			{1, "Alice Duplicate"},
			{3, "Charlie"},
		}
		result := array.UniqBy(data, func(p person) int { return p.ID })
		expected := []person{{1, "Alice"}, {2, "Bob"}, {3, "Charlie"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.UniqBy([]int{}, func(x int) int { return x })
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string first char", func(t *testing.T) {
		data := []string{"apple", "banana", "apricot", "blueberry"}
		result := array.UniqBy(data, func(s string) byte { return s[0] })
		expected := []string{"apple", "banana"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("by floor value", func(t *testing.T) {
		data := []float64{1.1, 1.2, 2.1, 2.2, 3.1}
		result := array.UniqBy(data, func(n float64) int { return int(n) })
		expected := []float64{1.1, 2.1, 3.1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("by modulo", func(t *testing.T) {
		data := []int{1, 2, 3, 11, 12, 13}
		result := array.UniqBy(data, func(n int) int { return n % 10 })
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("constant key returns first", func(t *testing.T) {
		data := []int{1, 2, 3, 4}
		result := array.UniqBy(data, func(_ int) string { return "same" })
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestCompact(t *testing.T) {
	t.Run("remove zero int values", func(t *testing.T) {
		result := array.Compact([]int{0, 1, 0, 2, 0, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("remove empty strings", func(t *testing.T) {
		result := array.Compact([]string{"", "hello", "", "world"})
		expected := []string{"hello", "world"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("remove false booleans", func(t *testing.T) {
		result := array.Compact([]bool{false, true, false, true})
		expected := []bool{true, true}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all truthy", func(t *testing.T) {
		result := array.Compact([]int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all zero values", func(t *testing.T) {
		result := array.Compact([]int{0, 0, 0})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.Compact([]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("pointer nil removal", func(t *testing.T) {
		a := 1
		b := 2
		result := array.Compact([]*int{nil, &a, nil, &b})
		if len(result) != 2 {
			t.Errorf("Expected 2 elements, got %d", len(result))
		}
		if *result[0] != 1 || *result[1] != 2 {
			t.Error("Wrong pointer values")
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		original := []int{0, 1, 0, 2}
		originalCopy := make([]int, len(original))
		copy(originalCopy, original)
		array.Compact(original)
		if !reflect.DeepEqual(original, originalCopy) {
			t.Error("Original was mutated")
		}
	})

	t.Run("float64 zero removal", func(t *testing.T) {
		result := array.Compact([]float64{0.0, 1.5, 0.0, 2.5})
		expected := []float64{1.5, 2.5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestShuffle(t *testing.T) {
	t.Run("same length", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		result := array.Shuffle(arr)
		if len(result) != len(arr) {
			t.Errorf("Expected length %d, got %d", len(arr), len(result))
		}
	})

	t.Run("same elements", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		result := array.Shuffle(arr)
		sortedResult := make([]int, len(result))
		copy(sortedResult, result)
		sort.Ints(sortedResult)
		sortedArr := make([]int, len(arr))
		copy(sortedArr, arr)
		sort.Ints(sortedArr)
		if !reflect.DeepEqual(sortedResult, sortedArr) {
			t.Error("Elements differ after shuffle")
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.Shuffle([]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := array.Shuffle([]int{1})
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		original := make([]int, len(arr))
		copy(original, arr)
		array.Shuffle(arr)
		if !reflect.DeepEqual(arr, original) {
			t.Error("Original was mutated")
		}
	})

	t.Run("maintains frequency of duplicates", func(t *testing.T) {
		arr := []int{1, 1, 2, 2, 3, 3}
		result := array.Shuffle(arr)
		sortedResult := make([]int, len(result))
		copy(sortedResult, result)
		sort.Ints(sortedResult)
		sortedArr := make([]int, len(arr))
		copy(sortedArr, arr)
		sort.Ints(sortedArr)
		if !reflect.DeepEqual(sortedResult, sortedArr) {
			t.Error("Element frequencies differ")
		}
	})
}

func TestShuffle2DArray(t *testing.T) {
	t.Run("single element", func(t *testing.T) {
		arr := [][]int{{1}}
		result := array.Shuffle2DArray(arr)
		expected := [][]int{{1}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.Shuffle2DArray([][]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("maintains dimensions", func(t *testing.T) {
		arr := [][]int{{1, 2}, {3, 4}, {5, 6}}
		result := array.Shuffle2DArray(arr)
		if len(result) != len(arr) {
			t.Errorf("Expected %d rows, got %d", len(arr), len(result))
		}
		for i, sub := range result {
			if len(sub) != len(arr[i]) {
				t.Errorf("Row %d: expected length %d, got %d", i, len(arr[i]), len(sub))
			}
		}
	})

	t.Run("same total elements", func(t *testing.T) {
		arr := [][]int{{1, 2}, {3, 4}, {5, 6}}
		result := array.Shuffle2DArray(arr)

		var flatOrig, flatResult []int
		for _, sub := range arr {
			flatOrig = append(flatOrig, sub...)
		}
		for _, sub := range result {
			flatResult = append(flatResult, sub...)
		}
		sort.Ints(flatOrig)
		sort.Ints(flatResult)
		if !reflect.DeepEqual(flatOrig, flatResult) {
			t.Error("Total elements differ")
		}
	})

	t.Run("different sub-array lengths", func(t *testing.T) {
		arr := [][]int{{1}, {2, 3}, {4, 5, 6}}
		result := array.Shuffle2DArray(arr)
		if len(result) != 3 {
			t.Errorf("Expected 3 rows, got %d", len(result))
		}
		if len(result[0]) != 1 || len(result[1]) != 2 || len(result[2]) != 3 {
			t.Error("Row lengths changed")
		}

		var flatOrig, flatResult []int
		for _, sub := range arr {
			flatOrig = append(flatOrig, sub...)
		}
		for _, sub := range result {
			flatResult = append(flatResult, sub...)
		}
		sort.Ints(flatOrig)
		sort.Ints(flatResult)
		if !reflect.DeepEqual(flatOrig, flatResult) {
			t.Error("Total elements differ")
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		arr := [][]int{{1, 2}, {3, 4}, {5, 6}}
		original := make([][]int, len(arr))
		for i, sub := range arr {
			original[i] = make([]int, len(sub))
			copy(original[i], sub)
		}
		array.Shuffle2DArray(arr)
		if !reflect.DeepEqual(arr, original) {
			t.Error("Original was mutated")
		}
	})
}

// =============================================================================
// Tests from search_test.go
// =============================================================================

func TestBinarySearch(t *testing.T) {
	cmp := func(a, b int) int {
		if a < b {
			return -1
		}
		if a > b {
			return 1
		}
		return 0
	}

	t.Run("target present", func(t *testing.T) {
		arr := []int{1, 3, 5, 7, 9, 11, 13, 15, 17, 19}
		if idx := array.BinarySearch(arr, 7, cmp); idx != 3 {
			t.Errorf("Expected 3, got %d", idx)
		}
		if idx := array.BinarySearch(arr, 1, cmp); idx != 0 {
			t.Errorf("Expected 0, got %d", idx)
		}
		if idx := array.BinarySearch(arr, 19, cmp); idx != 9 {
			t.Errorf("Expected 9, got %d", idx)
		}
	})

	t.Run("target not present", func(t *testing.T) {
		arr := []int{1, 3, 5, 7, 9, 11, 13, 15, 17, 19}
		if idx := array.BinarySearch(arr, 2, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
		if idx := array.BinarySearch(arr, 20, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
		if idx := array.BinarySearch(arr, 0, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		if idx := array.BinarySearch([]int{}, 1, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})

	t.Run("single element found", func(t *testing.T) {
		if idx := array.BinarySearch([]int{5}, 5, cmp); idx != 0 {
			t.Errorf("Expected 0, got %d", idx)
		}
	})

	t.Run("single element not found", func(t *testing.T) {
		if idx := array.BinarySearch([]int{5}, 1, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})

	t.Run("two element array", func(t *testing.T) {
		arr := []int{3, 6}
		if idx := array.BinarySearch(arr, 3, cmp); idx != 0 {
			t.Errorf("Expected 0, got %d", idx)
		}
		if idx := array.BinarySearch(arr, 6, cmp); idx != 1 {
			t.Errorf("Expected 1, got %d", idx)
		}
		if idx := array.BinarySearch(arr, 4, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})

	t.Run("string binary search", func(t *testing.T) {
		strCmp := func(a, b string) int {
			if a < b {
				return -1
			}
			if a > b {
				return 1
			}
			return 0
		}
		arr := []string{"apple", "banana", "cherry", "date"}
		if idx := array.BinarySearch(arr, "cherry", strCmp); idx != 2 {
			t.Errorf("Expected 2, got %d", idx)
		}
		if idx := array.BinarySearch(arr, "fig", strCmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})
}

func TestGetArraysCommon(t *testing.T) {
	t.Run("two arrays", func(t *testing.T) {
		result := array.GetArraysCommon([]int{1, 2, 3}, []int{2, 3, 4})
		expected := []int{2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("three arrays", func(t *testing.T) {
		result := array.GetArraysCommon([]int{1, 2, 3}, []int{2, 3, 9}, []int{3, 4, 5})
		expected := []int{3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty arrays", func(t *testing.T) {
		result := array.GetArraysCommon([]int{}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
		result = array.GetArraysCommon([]int{1, 2, 3}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("no common elements", func(t *testing.T) {
		result := array.GetArraysCommon([]int{1, 2, 3}, []int{4, 5, 6})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string arrays", func(t *testing.T) {
		result := array.GetArraysCommon([]string{"a", "b", "c"}, []string{"b", "c", "d"})
		expected := []string{"b", "c"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single array", func(t *testing.T) {
		result := array.GetArraysCommon([]int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("preserves order from first array", func(t *testing.T) {
		result := array.GetArraysCommon([]int{3, 1, 2}, []int{2, 3, 1}, []int{1, 3, 2})
		expected := []int{3, 1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("handles duplicates", func(t *testing.T) {
		result := array.GetArraysCommon([]int{1, 1, 2, 2, 3}, []int{2, 2, 3, 3, 4}, []int{3, 3, 4, 4, 5})
		expected := []int{3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("booleans", func(t *testing.T) {
		result := array.GetArraysCommon([]bool{true, false}, []bool{false, true}, []bool{true, false})
		expected := []bool{true, false}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := array.GetArraysCommon[int]()
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("large arrays", func(t *testing.T) {
		arr1 := make([]int, 10000)
		for i := range arr1 {
			arr1[i] = i
		}
		arr2 := make([]int, 10000)
		for i := range arr2 {
			arr2[i] = i + 5000
		}
		arr3 := make([]int, 10000)
		for i := range arr3 {
			arr3[i] = i + 7500
		}
		result := array.GetArraysCommon(arr1, arr2, arr3)
		if len(result) != 2500 {
			t.Errorf("Expected 2500 elements, got %d", len(result))
		}
		if result[0] != 7500 {
			t.Errorf("Expected first element 7500, got %d", result[0])
		}
		if result[len(result)-1] != 9999 {
			t.Errorf("Expected last element 9999, got %d", result[len(result)-1])
		}
	})
}

func TestGetArraysDiff(t *testing.T) {
	t.Run("standard arrays", func(t *testing.T) {
		result := array.GetArraysDiff([]int{1, 2, 3}, []int{2, 3, 4})
		expected := []int{1, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty arrays", func(t *testing.T) {
		result := array.GetArraysDiff([]int{}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("first empty", func(t *testing.T) {
		result := array.GetArraysDiff([]int{}, []int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("second empty", func(t *testing.T) {
		result := array.GetArraysDiff([]int{1, 2, 3}, []int{})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no duplicates between arrays", func(t *testing.T) {
		result := array.GetArraysDiff([]int{1, 2}, []int{3, 4})
		expected := []int{1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("identical arrays", func(t *testing.T) {
		result := array.GetArraysDiff([]int{1, 2, 3}, []int{1, 2, 3})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("identical arrays different order", func(t *testing.T) {
		result := array.GetArraysDiff([]int{1, 2, 3}, []int{3, 1, 2})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := array.GetArraysDiff([]string{"a", "b", "c"}, []string{"b", "c", "d"})
		expected := []string{"a", "d"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := array.GetArraysDiff([]int{-1, 0, 1}, []int{0, 1, 2})
		expected := []int{-1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("floats", func(t *testing.T) {
		result := array.GetArraysDiff([]float64{0.1, 0.2, 0.3}, []float64{0.2, 0.3, 0.4})
		expected := []float64{0.1, 0.4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestArraysJoin(t *testing.T) {
	t.Run("basic join", func(t *testing.T) {
		result := array.ArraysJoin([]int{1, 2, 3}, []int{4, 5, 6})
		expected := []int{1, 2, 3, 4, 5, 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("multiple arrays", func(t *testing.T) {
		result := array.ArraysJoin([]int{1, 2}, []int{3, 4}, []int{5, 6})
		expected := []int{1, 2, 3, 4, 5, 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with empty arrays", func(t *testing.T) {
		result := array.ArraysJoin([]int{}, []int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single array", func(t *testing.T) {
		result := array.ArraysJoin([]int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := array.ArraysJoin[int]()
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := array.ArraysJoin([]string{"a", "b"}, []string{"c", "d"})
		expected := []string{"a", "b", "c", "d"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large arrays", func(t *testing.T) {
		arr1 := make([]int, 10000)
		for i := range arr1 {
			arr1[i] = i
		}
		arr2 := make([]int, 10000)
		for i := range arr2 {
			arr2[i] = i + 10000
		}
		result := array.ArraysJoin(arr1, arr2)
		if len(result) != 20000 {
			t.Errorf("Expected 20000 elements, got %d", len(result))
		}
		if result[0] != 0 {
			t.Errorf("Expected first element 0, got %d", result[0])
		}
		if result[19999] != 19999 {
			t.Errorf("Expected last element 19999, got %d", result[19999])
		}
	})
}

func TestGroupBy(t *testing.T) {
	t.Run("odd and even", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		result := array.GroupBy(arr, func(n int) string {
			if n%2 == 0 {
				return "even"
			}
			return "odd"
		})
		expected := map[string][]int{"odd": {1, 3, 5}, "even": {2, 4}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings by length", func(t *testing.T) {
		arr := []string{"one", "two", "three", "four", "five"}
		result := array.GroupBy(arr, func(s string) int { return len(s) })
		expected := map[int][]string{3: {"one", "two"}, 4: {"four", "five"}, 5: {"three"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("objects by property", func(t *testing.T) {
		type item struct {
			Type string
			Name string
		}
		arr := []item{
			{"fruit", "apple"},
			{"vegetable", "carrot"},
			{"fruit", "banana"},
			{"vegetable", "lettuce"},
		}
		result := array.GroupBy(arr, func(i item) string { return i.Type })
		if len(result["fruit"]) != 2 || len(result["vegetable"]) != 2 {
			t.Errorf("Expected 2 fruits and 2 vegetables, got %v", result)
		}
		if result["fruit"][0].Name != "apple" || result["fruit"][1].Name != "banana" {
			t.Errorf("Wrong fruit grouping: %v", result["fruit"])
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.GroupBy([]int{}, func(n int) string { return "x" })
		if len(result) != 0 {
			t.Errorf("Expected empty map, got %v", result)
		}
	})

	t.Run("booleans", func(t *testing.T) {
		arr := []bool{true, false, true, false, true}
		result := array.GroupBy(arr, func(b bool) bool { return b })
		if len(result[true]) != 3 || len(result[false]) != 2 {
			t.Errorf("Expected 3 true, 2 false, got %v", result)
		}
	})
}

func TestRandomSelect(t *testing.T) {
	t.Run("selects from array", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		val, err := array.RandomSelect(arr)
		if err != nil {
			t.Errorf("Expected no error, got %v", err)
		}
		found := false
		for _, v := range arr {
			if v == val {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("Selected value %d not in array %v", val, arr)
		}
	})

	t.Run("empty array returns error", func(t *testing.T) {
		_, err := array.RandomSelect([]int{})
		if err == nil {
			t.Error("Expected error for empty array")
		}
	})

	t.Run("single element", func(t *testing.T) {
		val, err := array.RandomSelect([]int{42})
		if err != nil {
			t.Errorf("Expected no error, got %v", err)
		}
		if val != 42 {
			t.Errorf("Expected 42, got %d", val)
		}
	})

	t.Run("string array", func(t *testing.T) {
		arr := []string{"a", "b", "c"}
		val, err := array.RandomSelect(arr)
		if err != nil {
			t.Errorf("Expected no error, got %v", err)
		}
		found := false
		for _, v := range arr {
			if v == val {
				found = true
				break
			}
		}
		if !found {
			t.Errorf("Selected value %s not in array %v", val, arr)
		}
	})
}

func TestSum(t *testing.T) {
	t.Run("integer values", func(t *testing.T) {
		if result := array.Sum([]float64{1, 2, 3}); result != 6 {
			t.Errorf("Expected 6, got %f", result)
		}
	})

	t.Run("negative values", func(t *testing.T) {
		if result := array.Sum([]float64{-1, -2, -3}); result != -6 {
			t.Errorf("Expected -6, got %f", result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		if result := array.Sum([]float64{}); result != 0 {
			t.Errorf("Expected 0, got %f", result)
		}
	})

	t.Run("decimal values", func(t *testing.T) {
		result := array.Sum([]float64{0.1, 0.2, 0.3})
		if math.Abs(result-0.6) > 1e-10 {
			t.Errorf("Expected ~0.6, got %f", result)
		}
	})

	t.Run("mixed values", func(t *testing.T) {
		result := array.Sum([]float64{1, 2.5, 3.7})
		if math.Abs(result-7.2) > 1e-10 {
			t.Errorf("Expected ~7.2, got %f", result)
		}
	})

	t.Run("large numbers", func(t *testing.T) {
		largeNum := 1_000_000_000.0
		result := array.Sum([]float64{largeNum, largeNum})
		if result != 2*largeNum {
			t.Errorf("Expected %f, got %f", 2*largeNum, result)
		}
	})

	t.Run("cancellation", func(t *testing.T) {
		largeNum := 1_000_000_000.0
		result := array.Sum([]float64{largeNum, -largeNum})
		if result != 0 {
			t.Errorf("Expected 0, got %f", result)
		}
	})
}

func TestGenerateNumberArray(t *testing.T) {
	t.Run("positive step", func(t *testing.T) {
		result := array.GenerateNumberArray(0, 10, 2)
		expected := []float64{0, 2, 4, 6, 8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("fractional step", func(t *testing.T) {
		result := array.GenerateNumberArray(0, 1, 0.2)
		expected := []float64{0, 0.2, 0.4, 0.6, 0.8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative step", func(t *testing.T) {
		result := array.GenerateNumberArray(5, 0, -1)
		expected := []float64{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("zero step", func(t *testing.T) {
		result := array.GenerateNumberArray(0, 5, 0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("start greater than end with positive step", func(t *testing.T) {
		result := array.GenerateNumberArray(10, 5, 1)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("negative range", func(t *testing.T) {
		result := array.GenerateNumberArray(0, -5, -1)
		expected := []float64{0, -1, -2, -3, -4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large negative step", func(t *testing.T) {
		result := array.GenerateNumberArray(10, 5, -2)
		expected := []float64{10, 8, 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("half step", func(t *testing.T) {
		result := array.GenerateNumberArray(0, 5, 0.5)
		expected := []float64{0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

func TestCheckFlagAlignment(t *testing.T) {
	t.Run("flag is set", func(t *testing.T) {
		if !array.CheckFlagAlignment(0b1111, 0b0101) {
			t.Error("Expected true for 0b1111 & 0b0101")
		}
	})

	t.Run("flag is not set", func(t *testing.T) {
		if array.CheckFlagAlignment(0b1010, 0b0101) {
			t.Error("Expected false for 0b1010 & 0b0101")
		}
	})

	t.Run("exact match", func(t *testing.T) {
		if !array.CheckFlagAlignment(0b1010, 0b1010) {
			t.Error("Expected true for exact match")
		}
	})

	t.Run("zero flag", func(t *testing.T) {
		if !array.CheckFlagAlignment(42, 0) {
			t.Error("Expected true for zero flag")
		}
	})

	t.Run("zero value", func(t *testing.T) {
		if array.CheckFlagAlignment(0, 1) {
			t.Error("Expected false for zero value with non-zero flag")
		}
	})

	t.Run("both zero", func(t *testing.T) {
		if !array.CheckFlagAlignment(0, 0) {
			t.Error("Expected true for both zero")
		}
	})

	t.Run("partial flag set", func(t *testing.T) {
		if array.CheckFlagAlignment(0b1010, 0b1110) {
			t.Error("Expected false for partial flag set")
		}
	})

	t.Run("practical example: permissions", func(t *testing.T) {
		const (
			read    = 1 << 0
			write   = 1 << 1
			execute = 1 << 2
		)
		perms := read | write
		if !array.CheckFlagAlignment(perms, read) {
			t.Error("Expected read permission to be set")
		}
		if !array.CheckFlagAlignment(perms, write) {
			t.Error("Expected write permission to be set")
		}
		if array.CheckFlagAlignment(perms, execute) {
			t.Error("Expected execute permission to not be set")
		}
		if !array.CheckFlagAlignment(perms, read|write) {
			t.Error("Expected read|write to be set")
		}
	})
}

// =============================================================================
// Tests from levenshtein_test.go
// =============================================================================

func TestLevenshteinDistance(t *testing.T) {
	t.Run("identical strings", func(t *testing.T) {
		if d := array.LevenshteinDistance("hello", "hello"); d != 0 {
			t.Errorf("Expected 0, got %d", d)
		}
	})

	t.Run("empty strings", func(t *testing.T) {
		if d := array.LevenshteinDistance("", ""); d != 0 {
			t.Errorf("Expected 0, got %d", d)
		}
	})

	t.Run("one empty string", func(t *testing.T) {
		if d := array.LevenshteinDistance("", "hello"); d != 5 {
			t.Errorf("Expected 5, got %d", d)
		}
		if d := array.LevenshteinDistance("hello", ""); d != 5 {
			t.Errorf("Expected 5, got %d", d)
		}
	})

	t.Run("single character substitution", func(t *testing.T) {
		if d := array.LevenshteinDistance("cat", "bat"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("single character insertion", func(t *testing.T) {
		if d := array.LevenshteinDistance("cat", "cats"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("single character deletion", func(t *testing.T) {
		if d := array.LevenshteinDistance("cats", "cat"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("multiple differences - kitten/sitting", func(t *testing.T) {
		if d := array.LevenshteinDistance("kitten", "sitting"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("multiple differences - saturday/sunday", func(t *testing.T) {
		if d := array.LevenshteinDistance("saturday", "sunday"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("completely different strings", func(t *testing.T) {
		if d := array.LevenshteinDistance("abc", "xyz"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
		if d := array.LevenshteinDistance("hello", "world"); d != 4 {
			t.Errorf("Expected 4, got %d", d)
		}
	})

	t.Run("case sensitive", func(t *testing.T) {
		if d := array.LevenshteinDistance("Hello", "hello"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
		if d := array.LevenshteinDistance("ABC", "abc"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("unicode characters", func(t *testing.T) {
		if d := array.LevenshteinDistance("cafe\u0301", "cafe"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
	})

	t.Run("unicode Japanese", func(t *testing.T) {
		if d := array.LevenshteinDistance("\u3053\u3093\u306b\u3061\u306f", "\u3053\u3093\u3070\u3093\u306f"); d != 2 {
			t.Errorf("Expected 2, got %d", d)
		}
	})

	t.Run("symmetric", func(t *testing.T) {
		d1 := array.LevenshteinDistance("hello", "world")
		d2 := array.LevenshteinDistance("world", "hello")
		if d1 != d2 {
			t.Errorf("Expected symmetric: %d != %d", d1, d2)
		}
	})

	t.Run("single character strings", func(t *testing.T) {
		if d := array.LevenshteinDistance("a", "b"); d != 1 {
			t.Errorf("Expected 1, got %d", d)
		}
		if d := array.LevenshteinDistance("a", "a"); d != 0 {
			t.Errorf("Expected 0, got %d", d)
		}
	})

	t.Run("prefix string", func(t *testing.T) {
		if d := array.LevenshteinDistance("abc", "abcdef"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("suffix string", func(t *testing.T) {
		if d := array.LevenshteinDistance("def", "abcdef"); d != 3 {
			t.Errorf("Expected 3, got %d", d)
		}
	})

	t.Run("longer strings", func(t *testing.T) {
		s1 := "the quick brown fox"
		s2 := "the quick brown dog"
		d := array.LevenshteinDistance(s1, s2)
		if d != 2 {
			t.Errorf("Expected 2, got %d", d)
		}
	})

	t.Run("transposition is two edits", func(t *testing.T) {
		if d := array.LevenshteinDistance("ab", "ba"); d != 2 {
			t.Errorf("Expected 2, got %d", d)
		}
	})
}

// =============================================================================
// NEW: Tests for CompareFunctionDefault
// =============================================================================

func TestCompareFunctionDefault(t *testing.T) {
	t.Run("int greater than", func(t *testing.T) {
		result := array.CompareFunctionDefault(5, 3)
		if result != 1 {
			t.Errorf("Expected 1 for 5 > 3, got %d", result)
		}
	})

	t.Run("int less than", func(t *testing.T) {
		result := array.CompareFunctionDefault(3, 5)
		if result != -1 {
			t.Errorf("Expected -1 for 3 < 5, got %d", result)
		}
	})

	t.Run("int equal", func(t *testing.T) {
		result := array.CompareFunctionDefault(5, 5)
		if result != 0 {
			t.Errorf("Expected 0 for 5 == 5, got %d", result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		if r := array.CompareFunctionDefault(-1, -5); r != 1 {
			t.Errorf("Expected 1 for -1 > -5, got %d", r)
		}
		if r := array.CompareFunctionDefault(-5, -1); r != -1 {
			t.Errorf("Expected -1 for -5 < -1, got %d", r)
		}
		if r := array.CompareFunctionDefault(-3, -3); r != 0 {
			t.Errorf("Expected 0 for -3 == -3, got %d", r)
		}
	})

	t.Run("zero values", func(t *testing.T) {
		if r := array.CompareFunctionDefault(0, 0); r != 0 {
			t.Errorf("Expected 0 for 0 == 0, got %d", r)
		}
		if r := array.CompareFunctionDefault(0, 1); r != -1 {
			t.Errorf("Expected -1 for 0 < 1, got %d", r)
		}
		if r := array.CompareFunctionDefault(1, 0); r != 1 {
			t.Errorf("Expected 1 for 1 > 0, got %d", r)
		}
	})

	t.Run("float64 comparison", func(t *testing.T) {
		if r := array.CompareFunctionDefault(3.14, 2.71); r != 1 {
			t.Errorf("Expected 1 for 3.14 > 2.71, got %d", r)
		}
		if r := array.CompareFunctionDefault(2.71, 3.14); r != -1 {
			t.Errorf("Expected -1 for 2.71 < 3.14, got %d", r)
		}
		if r := array.CompareFunctionDefault(1.5, 1.5); r != 0 {
			t.Errorf("Expected 0 for 1.5 == 1.5, got %d", r)
		}
	})

	t.Run("string comparison", func(t *testing.T) {
		if r := array.CompareFunctionDefault("banana", "apple"); r != 1 {
			t.Errorf("Expected 1 for banana > apple, got %d", r)
		}
		if r := array.CompareFunctionDefault("apple", "banana"); r != -1 {
			t.Errorf("Expected -1 for apple < banana, got %d", r)
		}
		if r := array.CompareFunctionDefault("hello", "hello"); r != 0 {
			t.Errorf("Expected 0 for hello == hello, got %d", r)
		}
	})

	t.Run("use as sort comparator", func(t *testing.T) {
		result := array.QuickSort([]int{5, 3, 1, 4, 2}, array.CompareFunctionDefault[int])
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large int values", func(t *testing.T) {
		if r := array.CompareFunctionDefault(math.MaxInt64, math.MinInt64); r != 1 {
			t.Errorf("Expected 1 for MaxInt64 > MinInt64, got %d", r)
		}
		if r := array.CompareFunctionDefault(math.MinInt64, math.MaxInt64); r != -1 {
			t.Errorf("Expected -1 for MinInt64 < MaxInt64, got %d", r)
		}
	})
}

// =============================================================================
// NEW: Tests for UltraNumberSort
// =============================================================================

func TestUltraNumberSort(t *testing.T) {
	t.Run("ascending basic", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{5, 3, 1, 4, 2}, true)
		expected := []float64{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending basic", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{5, 3, 1, 4, 2}, false)
		expected := []float64{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{}, true)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{42}, true)
		expected := []float64{42}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("already sorted ascending", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{1, 2, 3, 4, 5}, true)
		expected := []float64{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("already sorted descending", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{5, 4, 3, 2, 1}, false)
		expected := []float64{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted ascending", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{5, 4, 3, 2, 1}, true)
		expected := []float64{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted descending", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{1, 2, 3, 4, 5}, false)
		expected := []float64{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with negative numbers ascending", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{3, -1, 4, -5, 2, 0, -3}, true)
		expected := []float64{-5, -3, -1, 0, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with negative numbers descending", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{3, -1, 4, -5, 2, 0, -3}, false)
		expected := []float64{4, 3, 2, 0, -1, -3, -5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with duplicates", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{3, 1, 3, 2, 1, 2}, true)
		expected := []float64{1, 1, 2, 2, 3, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("two elements", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{2, 1}, true)
		expected := []float64{1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}

		result = array.UltraNumberSort([]float64{1, 2}, false)
		expected = []float64{2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("three elements", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{3, 1, 2}, true)
		expected := []float64{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}

		result = array.UltraNumberSort([]float64{1, 3, 2}, false)
		expected = []float64{3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with floating point values", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{3.14, 1.41, 2.72, 0.58}, true)
		expected := []float64{0.58, 1.41, 2.72, 3.14}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		original := []float64{5, 3, 1, 4, 2}
		originalCopy := make([]float64, len(original))
		copy(originalCopy, original)
		array.UltraNumberSort(original, true)
		if !reflect.DeepEqual(original, originalCopy) {
			t.Error("Original array was mutated")
		}
	})

	t.Run("all identical elements", func(t *testing.T) {
		result := array.UltraNumberSort([]float64{7, 7, 7, 7, 7}, true)
		expected := []float64{7, 7, 7, 7, 7}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large array ascending", func(t *testing.T) {
		large := make([]float64, 1000)
		for i := range large {
			large[i] = float64(rand.Intn(10000))
		}
		result := array.UltraNumberSort(large, true)
		for i := 1; i < len(result); i++ {
			if result[i] < result[i-1] {
				t.Errorf("Not sorted ascending at index %d: %f < %f", i, result[i], result[i-1])
				break
			}
		}
	})

	t.Run("large array descending", func(t *testing.T) {
		large := make([]float64, 1000)
		for i := range large {
			large[i] = float64(rand.Intn(10000))
		}
		result := array.UltraNumberSort(large, false)
		for i := 1; i < len(result); i++ {
			if result[i] > result[i-1] {
				t.Errorf("Not sorted descending at index %d: %f > %f", i, result[i], result[i-1])
				break
			}
		}
	})
}
