package array

import (
	"math"
	"reflect"
	"testing"
)

// TestBinarySearch tests the BinarySearch function
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
		if idx := BinarySearch(arr, 7, cmp); idx != 3 {
			t.Errorf("Expected 3, got %d", idx)
		}
		if idx := BinarySearch(arr, 1, cmp); idx != 0 {
			t.Errorf("Expected 0, got %d", idx)
		}
		if idx := BinarySearch(arr, 19, cmp); idx != 9 {
			t.Errorf("Expected 9, got %d", idx)
		}
	})

	t.Run("target not present", func(t *testing.T) {
		arr := []int{1, 3, 5, 7, 9, 11, 13, 15, 17, 19}
		if idx := BinarySearch(arr, 2, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
		if idx := BinarySearch(arr, 20, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
		if idx := BinarySearch(arr, 0, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		if idx := BinarySearch([]int{}, 1, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})

	t.Run("single element found", func(t *testing.T) {
		if idx := BinarySearch([]int{5}, 5, cmp); idx != 0 {
			t.Errorf("Expected 0, got %d", idx)
		}
	})

	t.Run("single element not found", func(t *testing.T) {
		if idx := BinarySearch([]int{5}, 1, cmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})

	t.Run("two element array", func(t *testing.T) {
		arr := []int{3, 6}
		if idx := BinarySearch(arr, 3, cmp); idx != 0 {
			t.Errorf("Expected 0, got %d", idx)
		}
		if idx := BinarySearch(arr, 6, cmp); idx != 1 {
			t.Errorf("Expected 1, got %d", idx)
		}
		if idx := BinarySearch(arr, 4, cmp); idx != -1 {
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
		if idx := BinarySearch(arr, "cherry", strCmp); idx != 2 {
			t.Errorf("Expected 2, got %d", idx)
		}
		if idx := BinarySearch(arr, "fig", strCmp); idx != -1 {
			t.Errorf("Expected -1, got %d", idx)
		}
	})
}

// TestGetArraysCommon tests the GetArraysCommon function
func TestGetArraysCommon(t *testing.T) {
	t.Run("two arrays", func(t *testing.T) {
		result := GetArraysCommon([]int{1, 2, 3}, []int{2, 3, 4})
		expected := []int{2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("three arrays", func(t *testing.T) {
		result := GetArraysCommon([]int{1, 2, 3}, []int{2, 3, 9}, []int{3, 4, 5})
		expected := []int{3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty arrays", func(t *testing.T) {
		result := GetArraysCommon([]int{}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
		result = GetArraysCommon([]int{1, 2, 3}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("no common elements", func(t *testing.T) {
		result := GetArraysCommon([]int{1, 2, 3}, []int{4, 5, 6})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string arrays", func(t *testing.T) {
		result := GetArraysCommon([]string{"a", "b", "c"}, []string{"b", "c", "d"})
		expected := []string{"b", "c"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single array", func(t *testing.T) {
		result := GetArraysCommon([]int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("preserves order from first array", func(t *testing.T) {
		result := GetArraysCommon([]int{3, 1, 2}, []int{2, 3, 1}, []int{1, 3, 2})
		expected := []int{3, 1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("handles duplicates", func(t *testing.T) {
		result := GetArraysCommon([]int{1, 1, 2, 2, 3}, []int{2, 2, 3, 3, 4}, []int{3, 3, 4, 4, 5})
		expected := []int{3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("booleans", func(t *testing.T) {
		result := GetArraysCommon([]bool{true, false}, []bool{false, true}, []bool{true, false})
		expected := []bool{true, false}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := GetArraysCommon[int]()
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
		result := GetArraysCommon(arr1, arr2, arr3)
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

// TestGetArraysDiff tests the GetArraysDiff function
func TestGetArraysDiff(t *testing.T) {
	t.Run("standard arrays", func(t *testing.T) {
		result := GetArraysDiff([]int{1, 2, 3}, []int{2, 3, 4})
		expected := []int{1, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty arrays", func(t *testing.T) {
		result := GetArraysDiff([]int{}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("first empty", func(t *testing.T) {
		result := GetArraysDiff([]int{}, []int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("second empty", func(t *testing.T) {
		result := GetArraysDiff([]int{1, 2, 3}, []int{})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no duplicates between arrays", func(t *testing.T) {
		result := GetArraysDiff([]int{1, 2}, []int{3, 4})
		expected := []int{1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("identical arrays", func(t *testing.T) {
		result := GetArraysDiff([]int{1, 2, 3}, []int{1, 2, 3})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("identical arrays different order", func(t *testing.T) {
		result := GetArraysDiff([]int{1, 2, 3}, []int{3, 1, 2})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := GetArraysDiff([]string{"a", "b", "c"}, []string{"b", "c", "d"})
		expected := []string{"a", "d"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative numbers", func(t *testing.T) {
		result := GetArraysDiff([]int{-1, 0, 1}, []int{0, 1, 2})
		expected := []int{-1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("floats", func(t *testing.T) {
		result := GetArraysDiff([]float64{0.1, 0.2, 0.3}, []float64{0.2, 0.3, 0.4})
		expected := []float64{0.1, 0.4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestArraysJoin tests the ArraysJoin function
func TestArraysJoin(t *testing.T) {
	t.Run("basic join", func(t *testing.T) {
		result := ArraysJoin([]int{1, 2, 3}, []int{4, 5, 6})
		expected := []int{1, 2, 3, 4, 5, 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("multiple arrays", func(t *testing.T) {
		result := ArraysJoin([]int{1, 2}, []int{3, 4}, []int{5, 6})
		expected := []int{1, 2, 3, 4, 5, 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with empty arrays", func(t *testing.T) {
		result := ArraysJoin([]int{}, []int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single array", func(t *testing.T) {
		result := ArraysJoin([]int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := ArraysJoin[int]()
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := ArraysJoin([]string{"a", "b"}, []string{"c", "d"})
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
		result := ArraysJoin(arr1, arr2)
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

// TestGroupBy tests the GroupBy function
func TestGroupBy(t *testing.T) {
	t.Run("odd and even", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		result := GroupBy(arr, func(n int) string {
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
		result := GroupBy(arr, func(s string) int { return len(s) })
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
		result := GroupBy(arr, func(i item) string { return i.Type })
		if len(result["fruit"]) != 2 || len(result["vegetable"]) != 2 {
			t.Errorf("Expected 2 fruits and 2 vegetables, got %v", result)
		}
		if result["fruit"][0].Name != "apple" || result["fruit"][1].Name != "banana" {
			t.Errorf("Wrong fruit grouping: %v", result["fruit"])
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := GroupBy([]int{}, func(n int) string { return "x" })
		if len(result) != 0 {
			t.Errorf("Expected empty map, got %v", result)
		}
	})

	t.Run("booleans", func(t *testing.T) {
		arr := []bool{true, false, true, false, true}
		result := GroupBy(arr, func(b bool) bool { return b })
		if len(result[true]) != 3 || len(result[false]) != 2 {
			t.Errorf("Expected 3 true, 2 false, got %v", result)
		}
	})
}

// TestRandomSelect tests the RandomSelect function
func TestRandomSelect(t *testing.T) {
	t.Run("selects from array", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		val, err := RandomSelect(arr)
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
		_, err := RandomSelect([]int{})
		if err == nil {
			t.Error("Expected error for empty array")
		}
	})

	t.Run("single element", func(t *testing.T) {
		val, err := RandomSelect([]int{42})
		if err != nil {
			t.Errorf("Expected no error, got %v", err)
		}
		if val != 42 {
			t.Errorf("Expected 42, got %d", val)
		}
	})

	t.Run("string array", func(t *testing.T) {
		arr := []string{"a", "b", "c"}
		val, err := RandomSelect(arr)
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

// TestSum tests the Sum function
func TestSum(t *testing.T) {
	t.Run("integer values", func(t *testing.T) {
		if result := Sum([]float64{1, 2, 3}); result != 6 {
			t.Errorf("Expected 6, got %f", result)
		}
	})

	t.Run("negative values", func(t *testing.T) {
		if result := Sum([]float64{-1, -2, -3}); result != -6 {
			t.Errorf("Expected -6, got %f", result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		if result := Sum([]float64{}); result != 0 {
			t.Errorf("Expected 0, got %f", result)
		}
	})

	t.Run("decimal values", func(t *testing.T) {
		result := Sum([]float64{0.1, 0.2, 0.3})
		if math.Abs(result-0.6) > 1e-10 {
			t.Errorf("Expected ~0.6, got %f", result)
		}
	})

	t.Run("mixed values", func(t *testing.T) {
		result := Sum([]float64{1, 2.5, 3.7})
		if math.Abs(result-7.2) > 1e-10 {
			t.Errorf("Expected ~7.2, got %f", result)
		}
	})

	t.Run("large numbers", func(t *testing.T) {
		largeNum := 1_000_000_000.0
		result := Sum([]float64{largeNum, largeNum})
		if result != 2*largeNum {
			t.Errorf("Expected %f, got %f", 2*largeNum, result)
		}
	})

	t.Run("cancellation", func(t *testing.T) {
		largeNum := 1_000_000_000.0
		result := Sum([]float64{largeNum, -largeNum})
		if result != 0 {
			t.Errorf("Expected 0, got %f", result)
		}
	})
}

// TestGenerateNumberArray tests the GenerateNumberArray function
func TestGenerateNumberArray(t *testing.T) {
	t.Run("positive step", func(t *testing.T) {
		result := GenerateNumberArray(0, 10, 2)
		expected := []float64{0, 2, 4, 6, 8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("fractional step", func(t *testing.T) {
		result := GenerateNumberArray(0, 1, 0.2)
		expected := []float64{0, 0.2, 0.4, 0.6, 0.8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative step", func(t *testing.T) {
		result := GenerateNumberArray(5, 0, -1)
		expected := []float64{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("zero step", func(t *testing.T) {
		result := GenerateNumberArray(0, 5, 0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("start greater than end with positive step", func(t *testing.T) {
		result := GenerateNumberArray(10, 5, 1)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("negative range", func(t *testing.T) {
		result := GenerateNumberArray(0, -5, -1)
		expected := []float64{0, -1, -2, -3, -4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("large negative step", func(t *testing.T) {
		result := GenerateNumberArray(10, 5, -2)
		expected := []float64{10, 8, 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("half step", func(t *testing.T) {
		result := GenerateNumberArray(0, 5, 0.5)
		expected := []float64{0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestCheckFlagAlignment tests the CheckFlagAlignment function
func TestCheckFlagAlignment(t *testing.T) {
	t.Run("flag is set", func(t *testing.T) {
		if !CheckFlagAlignment(0b1111, 0b0101) {
			t.Error("Expected true for 0b1111 & 0b0101")
		}
	})

	t.Run("flag is not set", func(t *testing.T) {
		if CheckFlagAlignment(0b1010, 0b0101) {
			t.Error("Expected false for 0b1010 & 0b0101")
		}
	})

	t.Run("exact match", func(t *testing.T) {
		if !CheckFlagAlignment(0b1010, 0b1010) {
			t.Error("Expected true for exact match")
		}
	})

	t.Run("zero flag", func(t *testing.T) {
		if !CheckFlagAlignment(42, 0) {
			t.Error("Expected true for zero flag")
		}
	})

	t.Run("zero value", func(t *testing.T) {
		if CheckFlagAlignment(0, 1) {
			t.Error("Expected false for zero value with non-zero flag")
		}
	})

	t.Run("both zero", func(t *testing.T) {
		if !CheckFlagAlignment(0, 0) {
			t.Error("Expected true for both zero")
		}
	})

	t.Run("partial flag set", func(t *testing.T) {
		// value = 0b1010, flag = 0b1110 -> 0b1010 & 0b1110 = 0b1010 != 0b1110
		if CheckFlagAlignment(0b1010, 0b1110) {
			t.Error("Expected false for partial flag set")
		}
	})

	t.Run("practical example: permissions", func(t *testing.T) {
		const (
			read    = 1 << 0 // 1
			write   = 1 << 1 // 2
			execute = 1 << 2 // 4
		)
		perms := read | write // 3
		if !CheckFlagAlignment(perms, read) {
			t.Error("Expected read permission to be set")
		}
		if !CheckFlagAlignment(perms, write) {
			t.Error("Expected write permission to be set")
		}
		if CheckFlagAlignment(perms, execute) {
			t.Error("Expected execute permission to not be set")
		}
		if !CheckFlagAlignment(perms, read|write) {
			t.Error("Expected read|write to be set")
		}
	})
}
