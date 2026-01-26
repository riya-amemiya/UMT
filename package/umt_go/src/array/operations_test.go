package array

import (
	"reflect"
	"sort"
	"testing"
)

// TestChunk tests the Chunk function
func TestChunk(t *testing.T) {
	t.Run("splits array into chunks", func(t *testing.T) {
		result := Chunk([]int{0, 1, 2, 3, 4, 5, 6, 7}, 3)
		expected := [][]int{{0, 1, 2}, {3, 4, 5}, {6, 7}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := Chunk([]int{}, 3)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("chunk size larger than array", func(t *testing.T) {
		result := Chunk([]int{0, 1, 2}, 5)
		expected := [][]int{{0, 1, 2}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("chunk size of 1", func(t *testing.T) {
		result := Chunk([]int{1, 2, 3, 4, 5}, 1)
		expected := [][]int{{1}, {2}, {3}, {4}, {5}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("perfect division", func(t *testing.T) {
		result := Chunk([]int{1, 2, 3, 4, 5, 6}, 3)
		expected := [][]int{{1, 2, 3}, {4, 5, 6}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("array equal to chunk size", func(t *testing.T) {
		result := Chunk([]int{1, 2, 3}, 3)
		expected := [][]int{{1, 2, 3}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := Chunk([]int{42}, 1)
		expected := [][]int{{42}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("string elements", func(t *testing.T) {
		result := Chunk([]string{"a", "b", "c", "d", "e", "f", "g"}, 3)
		expected := [][]string{{"a", "b", "c"}, {"d", "e", "f"}, {"g"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("does not mutate input", func(t *testing.T) {
		input := []int{0, 1, 2, 3, 4, 5, 6, 7}
		original := make([]int, len(input))
		copy(original, input)
		Chunk(input, 3)
		if !reflect.DeepEqual(input, original) {
			t.Error("Input was mutated")
		}
	})

	t.Run("large array", func(t *testing.T) {
		input := make([]int, 10000)
		for i := range input {
			input[i] = i
		}
		result := Chunk(input, 1000)
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

// TestDrop tests the Drop function
func TestDrop(t *testing.T) {
	t.Run("drop from start", func(t *testing.T) {
		result := Drop([]int{1, 2, 3, 4, 5}, 2)
		expected := []int{3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("drop 1 by default logic", func(t *testing.T) {
		result := Drop([]int{1, 2, 3, 4, 5}, 1)
		expected := []int{2, 3, 4, 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("drop more than length", func(t *testing.T) {
		result := Drop([]int{1, 2, 3}, 4)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("drop equal to length", func(t *testing.T) {
		result := Drop([]int{1, 2, 3}, 3)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("drop zero", func(t *testing.T) {
		result := Drop([]int{1, 2, 3}, 0)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("drop negative", func(t *testing.T) {
		result := Drop([]int{1, 2, 3}, -1)
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := Drop([]int{}, 1)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})
}

// TestFirst tests the First function
func TestFirst(t *testing.T) {
	t.Run("non-empty int array", func(t *testing.T) {
		val, ok := First([]int{1, 2, 3})
		if !ok || val != 1 {
			t.Errorf("Expected (1, true), got (%d, %v)", val, ok)
		}
	})

	t.Run("non-empty string array", func(t *testing.T) {
		val, ok := First([]string{"a", "b", "c"})
		if !ok || val != "a" {
			t.Errorf("Expected (a, true), got (%s, %v)", val, ok)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		_, ok := First([]int{})
		if ok {
			t.Error("Expected false for empty array")
		}
	})

	t.Run("single element", func(t *testing.T) {
		val, ok := First([]int{42})
		if !ok || val != 42 {
			t.Errorf("Expected (42, true), got (%d, %v)", val, ok)
		}
	})

	t.Run("bool array", func(t *testing.T) {
		val, ok := First([]bool{true, false, true})
		if !ok || val != true {
			t.Errorf("Expected (true, true), got (%v, %v)", val, ok)
		}
	})

	t.Run("zero value first element", func(t *testing.T) {
		val, ok := First([]int{0, 1, 2})
		if !ok || val != 0 {
			t.Errorf("Expected (0, true), got (%d, %v)", val, ok)
		}
	})

	t.Run("false first element", func(t *testing.T) {
		val, ok := First([]bool{false, true})
		if !ok || val != false {
			t.Errorf("Expected (false, true), got (%v, %v)", val, ok)
		}
	})

	t.Run("empty string first element", func(t *testing.T) {
		val, ok := First([]string{"", "hello"})
		if !ok || val != "" {
			t.Errorf("Expected ('', true), got (%s, %v)", val, ok)
		}
	})
}

// TestPop tests the Pop function
func TestPop(t *testing.T) {
	t.Run("non-empty array", func(t *testing.T) {
		rest, val, ok := Pop([]int{1, 2, 3})
		if !ok || val != 3 {
			t.Errorf("Expected (3, true), got (%d, %v)", val, ok)
		}
		expected := []int{1, 2}
		if !reflect.DeepEqual(rest, expected) {
			t.Errorf("Expected rest %v, got %v", expected, rest)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		rest, _, ok := Pop([]int{})
		if ok {
			t.Error("Expected false for empty array")
		}
		if len(rest) != 0 {
			t.Errorf("Expected empty rest, got %v", rest)
		}
	})

	t.Run("single element", func(t *testing.T) {
		rest, val, ok := Pop([]int{42})
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
		Pop(original)
		if !reflect.DeepEqual(original, originalCopy) {
			t.Error("Original was mutated")
		}
	})

	t.Run("string array", func(t *testing.T) {
		rest, val, ok := Pop([]string{"a", "b", "c"})
		if !ok || val != "c" {
			t.Errorf("Expected (c, true), got (%s, %v)", val, ok)
		}
		expected := []string{"a", "b"}
		if !reflect.DeepEqual(rest, expected) {
			t.Errorf("Expected rest %v, got %v", expected, rest)
		}
	})
}

// TestRange tests the Range function
func TestRange(t *testing.T) {
	t.Run("positive range", func(t *testing.T) {
		result := Range(1, 10)
		expected := []int{1, 2, 3, 4, 5, 6, 7, 8, 9}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative to positive", func(t *testing.T) {
		result := Range(-5, 5)
		expected := []int{-5, -4, -3, -2, -1, 0, 1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("zero to N", func(t *testing.T) {
		result := Range(0, 5)
		expected := []int{0, 1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("same start and end", func(t *testing.T) {
		result := Range(0, 0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
		result = Range(5, 5)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("start greater than end", func(t *testing.T) {
		result := Range(10, 5)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})
}

// TestRangeFloat tests the RangeFloat function
func TestRangeFloat(t *testing.T) {
	t.Run("positive step", func(t *testing.T) {
		result := RangeFloat(0, 10, 2)
		expected := []float64{0, 2, 4, 6, 8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("fractional step", func(t *testing.T) {
		result := RangeFloat(0, 5, 0.5)
		expected := []float64{0, 0.5, 1, 1.5, 2, 2.5, 3, 3.5, 4, 4.5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative step", func(t *testing.T) {
		result := RangeFloat(5, 0, -1)
		expected := []float64{5, 4, 3, 2, 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("zero step", func(t *testing.T) {
		result := RangeFloat(0, 5, 0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("start greater than end with positive step", func(t *testing.T) {
		result := RangeFloat(10, 5, 1)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("floating point precision", func(t *testing.T) {
		result := RangeFloat(0, 1, 0.2)
		expected := []float64{0, 0.2, 0.4, 0.6, 0.8}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("negative range with negative step", func(t *testing.T) {
		result := RangeFloat(0, -5, -1)
		expected := []float64{0, -1, -2, -3, -4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestZip tests the Zip function
func TestZip(t *testing.T) {
	t.Run("equal length arrays", func(t *testing.T) {
		result := Zip([]int{1, 2, 3}, []int{4, 5, 6}, []int{7, 8, 9})
		expected := [][]int{{1, 4, 7}, {2, 5, 8}, {3, 6, 9}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("different length arrays", func(t *testing.T) {
		result := Zip([]int{1, 2}, []int{3, 4, 5})
		expected := [][]int{{1, 3}, {2, 4}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with empty array", func(t *testing.T) {
		result := Zip([]int{1, 2, 3}, []int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := Zip[int]()
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string arrays", func(t *testing.T) {
		result := Zip([]string{"a", "b"}, []string{"x", "y"})
		expected := [][]string{{"a", "x"}, {"b", "y"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestZipLongest tests the ZipLongest function
func TestZipLongest(t *testing.T) {
	t.Run("different length arrays with default", func(t *testing.T) {
		result := ZipLongest(0, []int{1, 2}, []int{3, 4, 5})
		expected := [][]int{{1, 3}, {2, 4}, {0, 5}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("equal length arrays", func(t *testing.T) {
		result := ZipLongest(0, []int{1, 2, 3}, []int{4, 5, 6})
		expected := [][]int{{1, 4}, {2, 5}, {3, 6}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("with empty array", func(t *testing.T) {
		result := ZipLongest(0, []int{1, 2, 3}, []int{})
		expected := [][]int{{1, 0}, {2, 0}, {3, 0}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("no arguments", func(t *testing.T) {
		result := ZipLongest[int](0)
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string default value", func(t *testing.T) {
		result := ZipLongest("N/A", []string{"a", "b"}, []string{"x", "y", "z"})
		expected := [][]string{{"a", "x"}, {"b", "y"}, {"N/A", "z"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("three arrays with different lengths", func(t *testing.T) {
		result := ZipLongest(0, []int{1, 2}, []int{3, 4, 5}, []int{6})
		expected := [][]int{{1, 3, 6}, {2, 4, 0}, {0, 5, 0}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestUnique tests the Unique function
func TestUnique(t *testing.T) {
	t.Run("remove duplicates", func(t *testing.T) {
		result := Unique([]int{1, 2, 2, 3, 3, 3, 4})
		expected := []int{1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := Unique([]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("no duplicates", func(t *testing.T) {
		result := Unique([]int{1, 2, 3, 4})
		expected := []int{1, 2, 3, 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("strings", func(t *testing.T) {
		result := Unique([]string{"a", "b", "b", "c", "a"})
		expected := []string{"a", "b", "c"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all same values", func(t *testing.T) {
		result := Unique([]int{1, 1, 1, 1})
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("preserves first occurrence order", func(t *testing.T) {
		result := Unique([]int{3, 1, 2, 1, 3, 2})
		expected := []int{3, 1, 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("booleans", func(t *testing.T) {
		result := Unique([]bool{true, false, true, false, true})
		expected := []bool{true, false}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestUniqBy tests the UniqBy function
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
		result := UniqBy(data, func(p person) int { return p.ID })
		expected := []person{{1, "Alice"}, {2, "Bob"}, {3, "Charlie"}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := UniqBy([]int{}, func(x int) int { return x })
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("string first char", func(t *testing.T) {
		data := []string{"apple", "banana", "apricot", "blueberry"}
		result := UniqBy(data, func(s string) byte { return s[0] })
		expected := []string{"apple", "banana"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("by floor value", func(t *testing.T) {
		data := []float64{1.1, 1.2, 2.1, 2.2, 3.1}
		result := UniqBy(data, func(n float64) int { return int(n) })
		expected := []float64{1.1, 2.1, 3.1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("by modulo", func(t *testing.T) {
		data := []int{1, 2, 3, 11, 12, 13}
		result := UniqBy(data, func(n int) int { return n % 10 })
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("constant key returns first", func(t *testing.T) {
		data := []int{1, 2, 3, 4}
		result := UniqBy(data, func(_ int) string { return "same" })
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestCompact tests the Compact function
func TestCompact(t *testing.T) {
	t.Run("remove zero int values", func(t *testing.T) {
		result := Compact([]int{0, 1, 0, 2, 0, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("remove empty strings", func(t *testing.T) {
		result := Compact([]string{"", "hello", "", "world"})
		expected := []string{"hello", "world"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("remove false booleans", func(t *testing.T) {
		result := Compact([]bool{false, true, false, true})
		expected := []bool{true, true}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all truthy", func(t *testing.T) {
		result := Compact([]int{1, 2, 3})
		expected := []int{1, 2, 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("all zero values", func(t *testing.T) {
		result := Compact([]int{0, 0, 0})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := Compact([]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("pointer nil removal", func(t *testing.T) {
		a := 1
		b := 2
		result := Compact([]*int{nil, &a, nil, &b})
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
		Compact(original)
		if !reflect.DeepEqual(original, originalCopy) {
			t.Error("Original was mutated")
		}
	})

	t.Run("float64 zero removal", func(t *testing.T) {
		result := Compact([]float64{0.0, 1.5, 0.0, 2.5})
		expected := []float64{1.5, 2.5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})
}

// TestShuffle tests the Shuffle function
func TestShuffle(t *testing.T) {
	t.Run("same length", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		result := Shuffle(arr)
		if len(result) != len(arr) {
			t.Errorf("Expected length %d, got %d", len(arr), len(result))
		}
	})

	t.Run("same elements", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		result := Shuffle(arr)
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
		result := Shuffle([]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("single element", func(t *testing.T) {
		result := Shuffle([]int{1})
		expected := []int{1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("does not mutate original", func(t *testing.T) {
		arr := []int{1, 2, 3, 4, 5}
		original := make([]int, len(arr))
		copy(original, arr)
		Shuffle(arr)
		if !reflect.DeepEqual(arr, original) {
			t.Error("Original was mutated")
		}
	})

	t.Run("maintains frequency of duplicates", func(t *testing.T) {
		arr := []int{1, 1, 2, 2, 3, 3}
		result := Shuffle(arr)
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

// TestShuffle2DArray tests the Shuffle2DArray function
func TestShuffle2DArray(t *testing.T) {
	t.Run("single element", func(t *testing.T) {
		arr := [][]int{{1}}
		result := Shuffle2DArray(arr)
		expected := [][]int{{1}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Expected %v, got %v", expected, result)
		}
	})

	t.Run("empty array", func(t *testing.T) {
		result := Shuffle2DArray([][]int{})
		if len(result) != 0 {
			t.Errorf("Expected empty, got %v", result)
		}
	})

	t.Run("maintains dimensions", func(t *testing.T) {
		arr := [][]int{{1, 2}, {3, 4}, {5, 6}}
		result := Shuffle2DArray(arr)
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
		result := Shuffle2DArray(arr)

		// Flatten and sort both
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
		result := Shuffle2DArray(arr)
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
		Shuffle2DArray(arr)
		if !reflect.DeepEqual(arr, original) {
			t.Error("Original was mutated")
		}
	})
}
