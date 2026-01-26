package object

import (
	"reflect"
	"testing"
)

// --- Merge tests ---

func TestMerge(t *testing.T) {
	t.Run("should merge multiple objects", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}
		source1 := map[string]any{"b": 3, "c": 4}
		source2 := map[string]any{"c": 5, "d": 6}

		result := Merge(target, source1, source2)
		expected := map[string]any{"a": 1, "b": 3, "c": 5, "d": 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should not modify original objects", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}
		source := map[string]any{"b": 3, "c": 4}

		result := Merge(target, source)

		// Check originals unchanged
		if target["b"] != 2 {
			t.Error("target should not be modified")
		}
		if source["b"] != 3 {
			t.Error("source should not be modified")
		}
		expected := map[string]any{"a": 1, "b": 3, "c": 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty target", func(t *testing.T) {
		target := map[string]any{}
		source := map[string]any{"a": 1, "b": 2}

		result := Merge(target, source)
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no sources", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}

		result := Merge(target)
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty sources", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}
		source := map[string]any{}

		result := Merge(target, source)
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should override properties with later sources", func(t *testing.T) {
		target := map[string]any{"a": 1}
		source1 := map[string]any{"a": 2}
		source2 := map[string]any{"a": 3}

		result := Merge(target, source1, source2)
		expected := map[string]any{"a": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nil values", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": nil}
		source := map[string]any{"b": nil, "c": nil}

		result := Merge(target, source)
		if result["a"] != 1 {
			t.Error("a should be 1")
		}
		if result["b"] != nil {
			t.Error("b should be nil")
		}
		if result["c"] != nil {
			t.Error("c should be nil")
		}
	})
}

// --- MergeDeep tests ---

func TestMergeDeep(t *testing.T) {
	t.Run("should deeply merge nested objects", func(t *testing.T) {
		target := map[string]any{
			"a": 1,
			"b": map[string]any{
				"c": 2,
				"d": 3,
			},
		}
		source := map[string]any{
			"b": map[string]any{
				"d": 4,
				"e": 5,
			},
			"f": 6,
		}

		result := MergeDeep(target, source)
		expected := map[string]any{
			"a": 1,
			"b": map[string]any{
				"c": 2,
				"d": 4,
				"e": 5,
			},
			"f": 6,
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should not modify original objects", func(t *testing.T) {
		target := map[string]any{"a": map[string]any{"b": 1}}
		source := map[string]any{"a": map[string]any{"c": 2}}

		result := MergeDeep(target, source)

		// Check originals
		targetA := target["a"].(map[string]any)
		if _, ok := targetA["c"]; ok {
			t.Error("target should not be modified")
		}
		sourceA := source["a"].(map[string]any)
		if _, ok := sourceA["b"]; ok {
			t.Error("source should not be modified")
		}

		expected := map[string]any{"a": map[string]any{"b": 1, "c": 2}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle multiple levels of nesting", func(t *testing.T) {
		target := map[string]any{
			"level1": map[string]any{
				"level2": map[string]any{
					"level3": map[string]any{
						"a": 1,
					},
				},
			},
		}
		source := map[string]any{
			"level1": map[string]any{
				"level2": map[string]any{
					"level3": map[string]any{
						"b": 2,
					},
					"c": 3,
				},
			},
		}

		result := MergeDeep(target, source)
		expected := map[string]any{
			"level1": map[string]any{
				"level2": map[string]any{
					"level3": map[string]any{
						"a": 1,
						"b": 2,
					},
					"c": 3,
				},
			},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle non-object values overriding objects", func(t *testing.T) {
		target := map[string]any{"a": map[string]any{"b": 1}}
		source := map[string]any{"a": "string"}

		result := MergeDeep(target, source)
		expected := map[string]any{"a": "string"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle arrays as values", func(t *testing.T) {
		target := map[string]any{
			"a": []int{1, 2},
			"b": map[string]any{"c": []int{3}},
		}
		source := map[string]any{
			"a": []int{4, 5},
			"b": map[string]any{"c": []int{6}},
		}

		result := MergeDeep(target, source)
		expected := map[string]any{
			"a": []int{4, 5},
			"b": map[string]any{"c": []int{6}},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty objects", func(t *testing.T) {
		target := map[string]any{}
		source := map[string]any{"a": map[string]any{"b": 1}}

		result := MergeDeep(target, source)
		expected := map[string]any{"a": map[string]any{"b": 1}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle multiple sources", func(t *testing.T) {
		target := map[string]any{"a": map[string]any{"b": 1}}
		source1 := map[string]any{"a": map[string]any{"c": 2}}
		source2 := map[string]any{"a": map[string]any{"d": 3}}

		result := MergeDeep(target, source1, source2)
		expected := map[string]any{"a": map[string]any{"b": 1, "c": 2, "d": 3}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nil values", func(t *testing.T) {
		target := map[string]any{"a": map[string]any{"b": 1}}
		source := map[string]any{"a": nil}

		result := MergeDeep(target, source)
		expected := map[string]any{"a": nil}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no sources provided", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": map[string]any{"c": 2}}

		result := MergeDeep(target)
		expected := map[string]any{"a": 1, "b": map[string]any{"c": 2}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})
}

// --- Pick tests ---

func TestPick(t *testing.T) {
	t.Run("should select a single key", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := Pick(obj, "a")
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should select multiple keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := Pick(obj, "a", "c")
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle non-existent keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := Pick(obj, "c")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty objects", func(t *testing.T) {
		obj := map[string]any{}
		result := Pick(obj, "a")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no keys specified", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := Pick(obj)
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should select all keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := Pick(obj, "a", "b", "c")
		if !reflect.DeepEqual(result, obj) {
			t.Errorf("Pick result = %v, want %v", result, obj)
		}
	})

	t.Run("should handle nested objects", func(t *testing.T) {
		obj := map[string]any{"a": map[string]any{"b": 1}, "c": 2}
		result := Pick(obj, "a")
		expected := map[string]any{"a": map[string]any{"b": 1}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nil values", func(t *testing.T) {
		obj := map[string]any{"a": nil, "b": nil, "c": 3}
		result := Pick(obj, "a", "b", "c")
		expected := map[string]any{"a": nil, "b": nil, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})
}

// --- Omit tests ---

func TestOmit(t *testing.T) {
	t.Run("should omit specified keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3, "d": 4}
		result := Omit(obj, "b", "d")
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should not modify original object", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := Omit(obj, "b")

		if len(obj) != 3 {
			t.Error("original should not be modified")
		}
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle non-existent keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := Omit(obj, "c")
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty object", func(t *testing.T) {
		obj := map[string]any{}
		result := Omit(obj, "a")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no keys to omit", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := Omit(obj)
		expected := map[string]any{"a": 1, "b": 2, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle omitting all keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := Omit(obj, "a", "b")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})
}

// --- Has tests ---

func TestHas(t *testing.T) {
	t.Run("should return true for existing key", func(t *testing.T) {
		m := map[string]any{"a": map[string]any{"b": 1}}
		if !Has(m, "a") {
			t.Error("Has should return true for existing key")
		}
	})

	t.Run("should return false for non-existing key", func(t *testing.T) {
		m := map[string]any{"a": map[string]any{"b": 1}}
		if Has(m, "b") {
			t.Error("Has should return false for non-existing key")
		}
	})

	t.Run("should return false for nil map", func(t *testing.T) {
		if Has(nil, "a") {
			t.Error("Has should return false for nil map")
		}
	})
}

// --- IsEmpty tests ---

func TestIsEmpty(t *testing.T) {
	t.Run("should return true for empty map", func(t *testing.T) {
		if !IsEmpty(map[string]any{}) {
			t.Error("empty map should be empty")
		}
	})

	t.Run("should return false for non-empty map", func(t *testing.T) {
		if IsEmpty(map[string]any{"a": 1}) {
			t.Error("non-empty map should not be empty")
		}
	})

	t.Run("should return false for map with nil values", func(t *testing.T) {
		if IsEmpty(map[string]any{"a": nil}) {
			t.Error("map with nil value should not be empty")
		}
	})

	t.Run("should return false for map with false values", func(t *testing.T) {
		if IsEmpty(map[string]any{"a": false}) {
			t.Error("map with false value should not be empty")
		}
	})

	t.Run("should return false for map with zero values", func(t *testing.T) {
		if IsEmpty(map[string]any{"a": 0}) {
			t.Error("map with zero value should not be empty")
		}
	})

	t.Run("should return false for map with empty string values", func(t *testing.T) {
		if IsEmpty(map[string]any{"a": ""}) {
			t.Error("map with empty string value should not be empty")
		}
	})

	t.Run("should return false for map with nested empty map", func(t *testing.T) {
		if IsEmpty(map[string]any{"a": map[string]any{}}) {
			t.Error("map with nested empty map should not be empty")
		}
	})
}

// --- KeyBy tests ---

func TestKeyBy(t *testing.T) {
	type Item struct {
		ID   string
		Name string
	}

	t.Run("should create object using field as key", func(t *testing.T) {
		input := []Item{
			{ID: "a1", Name: "Alice"},
			{ID: "b2", Name: "Bob"},
		}
		result := KeyBy(input, func(item Item) string {
			return item.ID
		})
		expected := map[string]Item{
			"a1": {ID: "a1", Name: "Alice"},
			"b2": {ID: "b2", Name: "Bob"},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("KeyBy result = %v, want %v", result, expected)
		}
	})

	t.Run("should generate keys using custom function", func(t *testing.T) {
		type DirCode struct {
			Dir  string
			Code int
		}
		input := []DirCode{
			{Dir: "left", Code: 97},
			{Dir: "right", Code: 100},
		}
		result := KeyBy(input, func(item DirCode) string {
			return string(rune(item.Code))
		})
		expected := map[string]DirCode{
			"a": {Dir: "left", Code: 97},
			"d": {Dir: "right", Code: 100},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("KeyBy result = %v, want %v", result, expected)
		}
	})

	t.Run("should return empty map for empty slice", func(t *testing.T) {
		result := KeyBy([]Item{}, func(item Item) string {
			return item.ID
		})
		if len(result) != 0 {
			t.Errorf("KeyBy on empty slice should return empty map, got %v", result)
		}
	})

	t.Run("should use later values for duplicate keys", func(t *testing.T) {
		input := []Item{
			{ID: "a1", Name: "Alice"},
			{ID: "a1", Name: "Alex"},
		}
		result := KeyBy(input, func(item Item) string {
			return item.ID
		})
		expected := map[string]Item{
			"a1": {ID: "a1", Name: "Alex"},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("KeyBy result = %v, want %v", result, expected)
		}
	})

	t.Run("should act as identity function for strings", func(t *testing.T) {
		input := []string{"a", "b"}
		result := KeyBy(input, func(item string) string {
			return item
		})
		expected := map[string]string{
			"a": "a",
			"b": "b",
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("KeyBy result = %v, want %v", result, expected)
		}
	})
}
