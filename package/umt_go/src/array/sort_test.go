package array

import (
	"math/rand"
	"reflect"
	"sort"
	"testing"
)

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

// TestQuickSort tests the QuickSort algorithm
func TestQuickSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := QuickSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := QuickSort([]int{1, 2, 3, 4, 5}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := QuickSort([]int{5, 4, 3, 2, 1}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		result := QuickSort([]int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := QuickSort([]int{3, 3, 3, 2, 1, 1, 4, 4, 5}, intCmp)
		expected := []int{1, 1, 2, 3, 3, 3, 4, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := QuickSort([]int{5, -1, 3, 2, 4, -5, 1, -2, 0}, intCmp)
		expected := []int{-5, -2, -1, 0, 1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		result := QuickSort([]int{1, 2, 3, 4, 5}, intCmpDesc)
		expected := []int{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := QuickSort([]int{1}, intCmp)
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all equal", func(t *testing.T) {
		result := QuickSort([]int{5, 5, 5, 5, 5}, intCmp)
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
		result := QuickSort(large, intCmp)
		sorted := make([]int, len(large))
		copy(sorted, large)
		sort.Ints(sorted)
		if !reflect.DeepEqual(result, sorted) {
			t.Error("Large array sort failed")
		}
	})

	t.Run("two element arrays", func(t *testing.T) {
		result := QuickSort([]int{2, 1}, intCmp)
		expected := []int{1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}

		result = QuickSort([]int{1, 2}, intCmp)
		expected = []int{1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		original := []int{3, 1, 4, 1, 5}
		originalCopy := make([]int, len(original))
		copy(originalCopy, original)
		QuickSort(original, intCmp)
		if !reflect.DeepEqual(original, originalCopy) {
			t.Error("Original array was mutated")
		}
	})
}

// TestMergeSort tests the MergeSort algorithm
func TestMergeSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := MergeSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := MergeSort([]int{1, 2, 3}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := MergeSort([]int{3, 2, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		result := MergeSort([]int{2, 3, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := MergeSort([]int{2, 3, 3, 1, 2}, intCmp)
		expected := []int{1, 2, 2, 3, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := MergeSort([]int{1}, intCmp)
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := MergeSort([]int{3, -1, 4, -5, 2, -3}, intCmp)
		expected := []int{-5, -3, -1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings alphabetically", func(t *testing.T) {
		result := MergeSort([]string{"banana", "apple", "cherry"}, stringCmp)
		expected := []string{"apple", "banana", "cherry"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("custom comparison descending", func(t *testing.T) {
		result := MergeSort([]int{1, 3, 2}, intCmpDesc)
		expected := []int{3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings by length", func(t *testing.T) {
		lenCmp := func(a, b string) int {
			return len(a) - len(b)
		}
		result := MergeSort([]string{"aaa", "a", "aa"}, lenCmp)
		expected := []string{"a", "aa", "aaa"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestTimSort tests the TimSort algorithm
func TestTimSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := TimSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := TimSort([]int{1, 2, 3}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := TimSort([]int{3, 2, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
		result := TimSort(arr, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := TimSort([]int{2, 3, 3, 1, 2}, intCmp)
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
		result := TimSort(large, intCmp)
		sorted := make([]int, len(large))
		copy(sorted, large)
		sort.Ints(sorted)
		if !reflect.DeepEqual(result, sorted) {
			t.Error("Large array sort failed")
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := TimSort([]int{5, -1, 3, 2, 4, -5, 1, -2, 0}, intCmp)
		expected := []int{-5, -2, -1, 0, 1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		result := TimSort([]int{1, 2, 3}, intCmpDesc)
		expected := []int{3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("partial sort", func(t *testing.T) {
		result := TimSort([]int{1, 3, 2, 5, 4}, intCmp, 1, 3)
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
		result := TimSort(arr, intCmp)
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
		result := TimSort(arr, intCmp)
		if !reflect.DeepEqual(result, arr) {
			t.Error("All identical elements should remain unchanged")
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := TimSort([]string{"b", "a", "d", "c"}, stringCmp)
		expected := []string{"a", "b", "c", "d"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestDualPivotQuickSort tests the DualPivotQuickSort algorithm
func TestDualPivotQuickSort(t *testing.T) {
	t.Run("ascending numbers", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3}
		result := DualPivotQuickSort(arr, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings lexicographically", func(t *testing.T) {
		result := DualPivotQuickSort([]string{"banana", "apple", "orange", "grape"}, stringCmp)
		expected := []string{"apple", "banana", "grape", "orange"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3}
		result := DualPivotQuickSort(arr, intCmpDesc)
		expected := []int{9, 6, 5, 5, 4, 3, 3, 2, 1, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := DualPivotQuickSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := DualPivotQuickSort([]int{1}, intCmp)
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all identical", func(t *testing.T) {
		result := DualPivotQuickSort([]int{2, 2, 2, 2, 2}, intCmp)
		expected := []int{2, 2, 2, 2, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := DualPivotQuickSort([]int{1, 2, 3, 4, 5}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := DualPivotQuickSort([]int{5, 4, 3, 2, 1}, intCmp)
		expected := []int{1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		arr := []int{-3, 1, -4, 1, 5, -9, 2, 6, 5, 3}
		result := DualPivotQuickSort(arr, intCmp)
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
		result := DualPivotQuickSort(arr, intCmp)
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
		result := DualPivotQuickSort(arr, cmp)
		expected := []item{{1, "a"}, {2, "b"}, {3, "c"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestInsertionSort tests the InsertionSort algorithm
func TestInsertionSort(t *testing.T) {
	t.Run("empty array", func(t *testing.T) {
		result := InsertionSort([]int{}, intCmp)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("already sorted", func(t *testing.T) {
		result := InsertionSort([]int{1, 2, 3}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("reverse sorted", func(t *testing.T) {
		result := InsertionSort([]int{3, 2, 1}, intCmp)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("random array", func(t *testing.T) {
		arr := []int{3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5}
		result := InsertionSort(arr, intCmp)
		expected := []int{1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("duplicates", func(t *testing.T) {
		result := InsertionSort([]int{2, 3, 3, 1, 2}, intCmp)
		expected := []int{1, 2, 2, 3, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := InsertionSort([]int{5, -1, 3, 2, 4, -5, 1, -2, 0}, intCmp)
		expected := []int{-5, -2, -1, 0, 1, 2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("descending order", func(t *testing.T) {
		result := InsertionSort([]int{1, 2, 3}, intCmpDesc)
		expected := []int{3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("partial sort", func(t *testing.T) {
		result := InsertionSort([]int{1, 3, 2, 5, 4}, intCmp, 1, 3)
		expected := []int{1, 2, 3, 5, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}
