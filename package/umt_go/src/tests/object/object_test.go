package object_test

import (
	"fmt"
	"math"
	"reflect"
	"strings"
	"testing"

	"github.com/riya-amemiya/umt-go/src/object"
)

// --- Merge tests ---

func TestMerge(t *testing.T) {
	t.Run("should merge multiple objects", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}
		source1 := map[string]any{"b": 3, "c": 4}
		source2 := map[string]any{"c": 5, "d": 6}

		result := object.Merge(target, source1, source2)
		expected := map[string]any{"a": 1, "b": 3, "c": 5, "d": 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should not modify original objects", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}
		source := map[string]any{"b": 3, "c": 4}

		result := object.Merge(target, source)

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

		result := object.Merge(target, source)
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no sources", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}

		result := object.Merge(target)
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty sources", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": 2}
		source := map[string]any{}

		result := object.Merge(target, source)
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should override properties with later sources", func(t *testing.T) {
		target := map[string]any{"a": 1}
		source1 := map[string]any{"a": 2}
		source2 := map[string]any{"a": 3}

		result := object.Merge(target, source1, source2)
		expected := map[string]any{"a": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Merge result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nil values", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": nil}
		source := map[string]any{"b": nil, "c": nil}

		result := object.Merge(target, source)
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

		result := object.MergeDeep(target, source)
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

		result := object.MergeDeep(target, source)

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

		result := object.MergeDeep(target, source)
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

		result := object.MergeDeep(target, source)
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

		result := object.MergeDeep(target, source)
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

		result := object.MergeDeep(target, source)
		expected := map[string]any{"a": map[string]any{"b": 1}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle multiple sources", func(t *testing.T) {
		target := map[string]any{"a": map[string]any{"b": 1}}
		source1 := map[string]any{"a": map[string]any{"c": 2}}
		source2 := map[string]any{"a": map[string]any{"d": 3}}

		result := object.MergeDeep(target, source1, source2)
		expected := map[string]any{"a": map[string]any{"b": 1, "c": 2, "d": 3}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nil values", func(t *testing.T) {
		target := map[string]any{"a": map[string]any{"b": 1}}
		source := map[string]any{"a": nil}

		result := object.MergeDeep(target, source)
		expected := map[string]any{"a": nil}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MergeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no sources provided", func(t *testing.T) {
		target := map[string]any{"a": 1, "b": map[string]any{"c": 2}}

		result := object.MergeDeep(target)
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
		result := object.Pick(obj, "a")
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should select multiple keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := object.Pick(obj, "a", "c")
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle non-existent keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := object.Pick(obj, "c")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty objects", func(t *testing.T) {
		obj := map[string]any{}
		result := object.Pick(obj, "a")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no keys specified", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := object.Pick(obj)
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should select all keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := object.Pick(obj, "a", "b", "c")
		if !reflect.DeepEqual(result, obj) {
			t.Errorf("Pick result = %v, want %v", result, obj)
		}
	})

	t.Run("should handle nested objects", func(t *testing.T) {
		obj := map[string]any{"a": map[string]any{"b": 1}, "c": 2}
		result := object.Pick(obj, "a")
		expected := map[string]any{"a": map[string]any{"b": 1}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Pick result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nil values", func(t *testing.T) {
		obj := map[string]any{"a": nil, "b": nil, "c": 3}
		result := object.Pick(obj, "a", "b", "c")
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
		result := object.Omit(obj, "b", "d")
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should not modify original object", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := object.Omit(obj, "b")

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
		result := object.Omit(obj, "c")
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty object", func(t *testing.T) {
		obj := map[string]any{}
		result := object.Omit(obj, "a")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle no keys to omit", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := object.Omit(obj)
		expected := map[string]any{"a": 1, "b": 2, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Omit result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle omitting all keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := object.Omit(obj, "a", "b")
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
		if !object.Has(m, "a") {
			t.Error("Has should return true for existing key")
		}
	})

	t.Run("should return false for non-existing key", func(t *testing.T) {
		m := map[string]any{"a": map[string]any{"b": 1}}
		if object.Has(m, "b") {
			t.Error("Has should return false for non-existing key")
		}
	})

	t.Run("should return false for nil map", func(t *testing.T) {
		if object.Has(nil, "a") {
			t.Error("Has should return false for nil map")
		}
	})
}

// --- IsEmpty tests ---

func TestIsEmpty(t *testing.T) {
	t.Run("should return true for empty map", func(t *testing.T) {
		if !object.IsEmpty(map[string]any{}) {
			t.Error("empty map should be empty")
		}
	})

	t.Run("should return false for non-empty map", func(t *testing.T) {
		if object.IsEmpty(map[string]any{"a": 1}) {
			t.Error("non-empty map should not be empty")
		}
	})

	t.Run("should return false for map with nil values", func(t *testing.T) {
		if object.IsEmpty(map[string]any{"a": nil}) {
			t.Error("map with nil value should not be empty")
		}
	})

	t.Run("should return false for map with false values", func(t *testing.T) {
		if object.IsEmpty(map[string]any{"a": false}) {
			t.Error("map with false value should not be empty")
		}
	})

	t.Run("should return false for map with zero values", func(t *testing.T) {
		if object.IsEmpty(map[string]any{"a": 0}) {
			t.Error("map with zero value should not be empty")
		}
	})

	t.Run("should return false for map with empty string values", func(t *testing.T) {
		if object.IsEmpty(map[string]any{"a": ""}) {
			t.Error("map with empty string value should not be empty")
		}
	})

	t.Run("should return false for map with nested empty map", func(t *testing.T) {
		if object.IsEmpty(map[string]any{"a": map[string]any{}}) {
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
		result := object.KeyBy(input, func(item Item) string {
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
		result := object.KeyBy(input, func(item DirCode) string {
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
		result := object.KeyBy([]Item{}, func(item Item) string {
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
		result := object.KeyBy(input, func(item Item) string {
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
		result := object.KeyBy(input, func(item string) string {
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

// --- PickDeep tests ---

func TestPickDeep(t *testing.T) {
	t.Run("should pick nested properties using dot notation", func(t *testing.T) {
		obj := map[string]any{
			"a": map[string]any{
				"b": map[string]any{
					"c": 1,
					"d": 2,
				},
			},
			"f": 4,
		}
		result := object.PickDeep(obj, "a.b.c", "f")
		expected := map[string]any{
			"a": map[string]any{
				"b": map[string]any{
					"c": 1,
				},
			},
			"f": 4,
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle top-level keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2, "c": 3}
		result := object.PickDeep(obj, "a", "c")
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should skip non-existent keys", func(t *testing.T) {
		obj := map[string]any{"a": 1}
		result := object.PickDeep(obj, "b", "c.d")
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty keys", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := object.PickDeep(obj)
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle multiple nested picks from same parent", func(t *testing.T) {
		obj := map[string]any{
			"a": map[string]any{
				"b": 1,
				"c": 2,
				"d": 3,
			},
		}
		result := object.PickDeep(obj, "a.b", "a.c")
		expected := map[string]any{
			"a": map[string]any{
				"b": 1,
				"c": 2,
			},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle deeply nested single path", func(t *testing.T) {
		obj := map[string]any{
			"a": map[string]any{
				"b": map[string]any{
					"c": map[string]any{
						"d": 42,
					},
				},
			},
		}
		result := object.PickDeep(obj, "a.b.c.d")
		expected := map[string]any{
			"a": map[string]any{
				"b": map[string]any{
					"c": map[string]any{
						"d": 42,
					},
				},
			},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickDeep result = %v, want %v", result, expected)
		}
	})
}

// --- IsPlainObject tests ---

func TestIsPlainObject(t *testing.T) {
	t.Run("should return true for maps", func(t *testing.T) {
		if !object.IsPlainObject(map[string]any{}) {
			t.Error("empty map should be plain object")
		}
		if !object.IsPlainObject(map[string]any{"a": 1}) {
			t.Error("non-empty map should be plain object")
		}
	})

	t.Run("should return false for slices", func(t *testing.T) {
		if object.IsPlainObject([]any{}) {
			t.Error("slice should not be plain object")
		}
		if object.IsPlainObject([]int{1, 2}) {
			t.Error("typed slice should not be plain object")
		}
	})

	t.Run("should return false for nil", func(t *testing.T) {
		if object.IsPlainObject(nil) {
			t.Error("nil should not be plain object")
		}
	})

	t.Run("should return false for primitives", func(t *testing.T) {
		if object.IsPlainObject(1) {
			t.Error("int should not be plain object")
		}
		if object.IsPlainObject("string") {
			t.Error("string should not be plain object")
		}
		if object.IsPlainObject(true) {
			t.Error("bool should not be plain object")
		}
	})
}

// --- DeepClone tests ---

func TestDeepClone(t *testing.T) {
	t.Run("should deep clone a simple map", func(t *testing.T) {
		original := map[string]any{"a": 1, "b": "hello"}
		cloned := object.DeepClone(original)
		if !reflect.DeepEqual(cloned, original) {
			t.Errorf("DeepClone result = %v, want %v", cloned, original)
		}
		cloned["a"] = 99
		if original["a"] != 1 {
			t.Error("original should not be modified")
		}
	})

	t.Run("should deep clone nested maps", func(t *testing.T) {
		original := map[string]any{"a": map[string]any{"b": map[string]any{"c": 3}}}
		cloned := object.DeepClone(original)
		if !reflect.DeepEqual(cloned, original) {
			t.Errorf("DeepClone result = %v, want %v", cloned, original)
		}
		cloned["a"].(map[string]any)["b"].(map[string]any)["c"] = 99
		if original["a"].(map[string]any)["b"].(map[string]any)["c"] != 3 {
			t.Error("nested original should not be modified")
		}
	})

	t.Run("should deep clone slices", func(t *testing.T) {
		original := []any{1, []any{2, []any{3}}}
		cloned := object.DeepClone(original)
		if !reflect.DeepEqual(cloned, original) {
			t.Errorf("DeepClone result = %v, want %v", cloned, original)
		}
		cloned[1].([]any)[0] = 99
		if original[1].([]any)[0] != 2 {
			t.Error("nested slice original should not be modified")
		}
	})

	t.Run("should clone primitive values", func(t *testing.T) {
		if object.DeepClone(42) != 42 {
			t.Error("int clone failed")
		}
		if object.DeepClone("hello") != "hello" {
			t.Error("string clone failed")
		}
		if object.DeepClone(true) != true {
			t.Error("bool clone failed")
		}
	})

	t.Run("should clone an empty map and empty slice", func(t *testing.T) {
		if !reflect.DeepEqual(object.DeepClone(map[string]any{}), map[string]any{}) {
			t.Error("empty map clone failed")
		}
		if !reflect.DeepEqual(object.DeepClone([]any{}), []any{}) {
			t.Error("empty slice clone failed")
		}
	})

	t.Run("should deep clone slices of maps together", func(t *testing.T) {
		original := map[string]any{
			"list": []any{
				map[string]any{"tags": []any{"a", "b"}},
				map[string]any{"tags": []any{"c", "d"}},
			},
		}
		cloned := object.DeepClone(original)
		if !reflect.DeepEqual(cloned, original) {
			t.Errorf("DeepClone result = %v, want %v", cloned, original)
		}
		list := cloned["list"].([]any)
		first := list[0].(map[string]any)
		first["tags"] = append(first["tags"].([]any), "z")
		origFirst := original["list"].([]any)[0].(map[string]any)
		if !reflect.DeepEqual(origFirst["tags"], []any{"a", "b"}) {
			t.Error("nested slice in map should be independent")
		}
	})
}

// --- FlattenObject tests ---

func TestFlattenObject(t *testing.T) {
	t.Run("flattens a nested object using dot separator", func(t *testing.T) {
		result := object.FlattenObject(map[string]any{"a": map[string]any{"b": map[string]any{"c": 1}}})
		expected := map[string]any{"a.b.c": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("FlattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("preserves top-level keys", func(t *testing.T) {
		result := object.FlattenObject(map[string]any{"a": 1, "b": 2})
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("FlattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("supports custom separator", func(t *testing.T) {
		result := object.FlattenObject(map[string]any{"a": map[string]any{"b": 1}}, "/")
		expected := map[string]any{"a/b": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("FlattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("keeps slices as values", func(t *testing.T) {
		result := object.FlattenObject(map[string]any{"a": map[string]any{"b": []any{1, 2}}})
		expected := map[string]any{"a.b": []any{1, 2}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("FlattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("treats empty plain objects as leaf values", func(t *testing.T) {
		result := object.FlattenObject(map[string]any{"a": map[string]any{}})
		expected := map[string]any{"a": map[string]any{}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("FlattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("returns empty for empty input", func(t *testing.T) {
		result := object.FlattenObject(map[string]any{})
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("FlattenObject result = %v, want %v", result, expected)
		}
	})
}

// --- UnflattenObject tests ---

func TestUnflattenObject(t *testing.T) {
	t.Run("rebuilds a nested object from dotted paths", func(t *testing.T) {
		result := object.UnflattenObject(map[string]any{"a.b.c": 1})
		expected := map[string]any{"a": map[string]any{"b": map[string]any{"c": 1}}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("UnflattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("merges multiple paths into shared parents", func(t *testing.T) {
		result := object.UnflattenObject(map[string]any{"a.b": 1, "a.c": 2})
		expected := map[string]any{"a": map[string]any{"b": 1, "c": 2}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("UnflattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("supports custom separator", func(t *testing.T) {
		result := object.UnflattenObject(map[string]any{"a/b": 1}, "/")
		expected := map[string]any{"a": map[string]any{"b": 1}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("UnflattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("preserves top-level scalars", func(t *testing.T) {
		result := object.UnflattenObject(map[string]any{"a": 1, "b": 2})
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("UnflattenObject result = %v, want %v", result, expected)
		}
	})

	t.Run("returns empty for empty input", func(t *testing.T) {
		result := object.UnflattenObject(map[string]any{})
		expected := map[string]any{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("UnflattenObject result = %v, want %v", result, expected)
		}
	})
}

// --- Get tests ---

func TestGet(t *testing.T) {
	t.Run("reads a top-level property", func(t *testing.T) {
		if object.Get(map[string]any{"a": 1}, "a") != 1 {
			t.Error("Get top-level failed")
		}
	})

	t.Run("reads a nested property by dotted path", func(t *testing.T) {
		got := object.Get(map[string]any{"a": map[string]any{"b": map[string]any{"c": 42}}}, "a.b.c")
		if got != 42 {
			t.Errorf("Get nested = %v, want 42", got)
		}
	})

	t.Run("reads a nested property by slice path", func(t *testing.T) {
		got := object.Get(map[string]any{"a": map[string]any{"b": map[string]any{"c": 42}}}, []string{"a", "b", "c"})
		if got != 42 {
			t.Errorf("Get nested by slice = %v, want 42", got)
		}
	})

	t.Run("returns the default when a segment is missing", func(t *testing.T) {
		got := object.Get(map[string]any{"a": map[string]any{"b": 1}}, "a.x", "fallback")
		if got != "fallback" {
			t.Errorf("Get missing = %v, want fallback", got)
		}
	})

	t.Run("returns the default when traversing through nil", func(t *testing.T) {
		got := object.Get(map[string]any{"a": nil}, "a.b", "fallback")
		if got != "fallback" {
			t.Errorf("Get through nil = %v, want fallback", got)
		}
	})

	t.Run("returns nil when no default and missing", func(t *testing.T) {
		got := object.Get(map[string]any{"a": 1}, "x")
		if got != nil {
			t.Errorf("Get missing no default = %v, want nil", got)
		}
	})

	t.Run("returns the default when the resolved value is nil", func(t *testing.T) {
		got := object.Get(map[string]any{"a": nil}, "a", 0)
		if got != 0 {
			t.Errorf("Get nil value = %v, want 0", got)
		}
	})

	t.Run("handles object root being nil", func(t *testing.T) {
		got := object.Get(nil, "a.b", 7)
		if got != 7 {
			t.Errorf("Get nil root = %v, want 7", got)
		}
	})
}

// --- Set tests ---

func TestSet(t *testing.T) {
	t.Run("sets a top-level property", func(t *testing.T) {
		target := map[string]any{}
		object.Set(target, "a", 1)
		if !reflect.DeepEqual(target, map[string]any{"a": 1}) {
			t.Errorf("Set result = %v", target)
		}
	})

	t.Run("creates nested objects when missing", func(t *testing.T) {
		target := map[string]any{}
		object.Set(target, "a.b.c", 1)
		expected := map[string]any{"a": map[string]any{"b": map[string]any{"c": 1}}}
		if !reflect.DeepEqual(target, expected) {
			t.Errorf("Set nested result = %v, want %v", target, expected)
		}
	})

	t.Run("respects slice path input", func(t *testing.T) {
		target := map[string]any{}
		object.Set(target, []string{"a", "b"}, 2)
		expected := map[string]any{"a": map[string]any{"b": 2}}
		if !reflect.DeepEqual(target, expected) {
			t.Errorf("Set slice path result = %v, want %v", target, expected)
		}
	})

	t.Run("overwrites existing leaf values", func(t *testing.T) {
		target := map[string]any{"a": map[string]any{"b": 1}}
		object.Set(target, "a.b", 2)
		expected := map[string]any{"a": map[string]any{"b": 2}}
		if !reflect.DeepEqual(target, expected) {
			t.Errorf("Set overwrite result = %v, want %v", target, expected)
		}
	})

	t.Run("replaces non-object intermediate values", func(t *testing.T) {
		target := map[string]any{"a": 5}
		object.Set(target, "a.b", 1)
		expected := map[string]any{"a": map[string]any{"b": 1}}
		if !reflect.DeepEqual(target, expected) {
			t.Errorf("Set replace result = %v, want %v", target, expected)
		}
	})

	t.Run("returns the same object reference", func(t *testing.T) {
		target := map[string]any{}
		result := object.Set(target, "a", 1)
		result["x"] = 9
		if target["x"] != 9 {
			t.Error("Set should return the same reference")
		}
	})

	t.Run("returns the object unchanged when path is empty", func(t *testing.T) {
		target := map[string]any{"a": 1}
		object.Set(target, []string{}, 2)
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(target, expected) {
			t.Errorf("Set empty path result = %v, want %v", target, expected)
		}
	})
}

// --- Invert tests ---

func TestInvert(t *testing.T) {
	t.Run("swaps keys and values", func(t *testing.T) {
		result := object.Invert(map[string]any{"a": 1, "b": 2})
		expected := map[string]string{"1": "a", "2": "b"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Invert result = %v, want %v", result, expected)
		}
	})

	t.Run("handles string-to-string swap", func(t *testing.T) {
		result := object.Invert(map[string]any{"x": "X", "y": "Y"})
		expected := map[string]string{"X": "x", "Y": "y"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Invert result = %v, want %v", result, expected)
		}
	})

	t.Run("returns empty object for empty input", func(t *testing.T) {
		result := object.Invert(map[string]any{})
		expected := map[string]string{}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("Invert result = %v, want %v", result, expected)
		}
	})

	t.Run("overwrites earlier keys when values collide", func(t *testing.T) {
		result := object.Invert(map[string]any{"a": 1, "b": 1})
		if len(result) != 1 {
			t.Errorf("Invert collide result = %v, want one entry", result)
		}
		if result["1"] != "a" && result["1"] != "b" {
			t.Errorf("Invert collide result = %v, want a or b", result)
		}
	})

	t.Run("does not mutate the input", func(t *testing.T) {
		input := map[string]any{"a": 1, "b": 2}
		object.Invert(input)
		if !reflect.DeepEqual(input, map[string]any{"a": 1, "b": 2}) {
			t.Error("Invert should not mutate input")
		}
	})
}

// --- MapKeys tests ---

func TestMapKeys(t *testing.T) {
	t.Run("should transform keys using the provided function", func(t *testing.T) {
		result := object.MapKeys(map[string]any{"a": 1, "b": 2, "c": 3}, func(_ any, key string) string {
			return strings.ToUpper(key)
		})
		expected := map[string]any{"A": 1, "B": 2, "C": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MapKeys result = %v, want %v", result, expected)
		}
	})

	t.Run("should pass value and key to the transformer", func(t *testing.T) {
		result := object.MapKeys(map[string]any{"x": 10, "y": 20}, func(value any, key string) string {
			return fmt.Sprintf("%s_%v", key, value)
		})
		expected := map[string]any{"x_10": 10, "y_20": 20}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MapKeys result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle an empty object", func(t *testing.T) {
		result := object.MapKeys(map[string]any{}, func(_ any, key string) string {
			return key
		})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("MapKeys empty result = %v", result)
		}
	})

	t.Run("should handle keys that map to the same value", func(t *testing.T) {
		result := object.MapKeys(map[string]any{"a": 1, "b": 2}, func(_ any, _ string) string {
			return "same"
		})
		if len(result) != 1 {
			t.Errorf("MapKeys collide result = %v, want one entry", result)
		}
	})

	t.Run("should not modify the original object", func(t *testing.T) {
		original := map[string]any{"a": 1, "b": 2}
		object.MapKeys(original, func(_ any, key string) string {
			return strings.ToUpper(key)
		})
		if !reflect.DeepEqual(original, map[string]any{"a": 1, "b": 2}) {
			t.Error("MapKeys should not mutate input")
		}
	})
}

// --- MapValues tests ---

func TestMapValues(t *testing.T) {
	t.Run("should transform values using the provided function", func(t *testing.T) {
		result := object.MapValues(map[string]any{"a": 1, "b": 2, "c": 3}, func(value any, _ string) any {
			return value.(int) * 2
		})
		expected := map[string]any{"a": 2, "b": 4, "c": 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MapValues result = %v, want %v", result, expected)
		}
	})

	t.Run("should pass value and key to the transformer", func(t *testing.T) {
		result := object.MapValues(map[string]any{"x": 10, "y": 20}, func(value any, key string) any {
			return fmt.Sprintf("%s=%v", key, value)
		})
		expected := map[string]any{"x": "x=10", "y": "y=20"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MapValues result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle an empty object", func(t *testing.T) {
		result := object.MapValues(map[string]any{}, func(value any, _ string) any {
			return value
		})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("MapValues empty result = %v", result)
		}
	})

	t.Run("should not modify the original object", func(t *testing.T) {
		original := map[string]any{"a": 1, "b": 2}
		object.MapValues(original, func(value any, _ string) any {
			return value.(int) * 2
		})
		if !reflect.DeepEqual(original, map[string]any{"a": 1, "b": 2}) {
			t.Error("MapValues should not mutate input")
		}
	})

	t.Run("should handle different return types", func(t *testing.T) {
		result := object.MapValues(map[string]any{"a": 1, "b": 2, "c": 3}, func(value any, _ string) any {
			return value.(int) > 1
		})
		expected := map[string]any{"a": false, "b": true, "c": true}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("MapValues result = %v, want %v", result, expected)
		}
	})

	t.Run("should preserve reference equality for returned values", func(t *testing.T) {
		nested := map[string]any{"inner": 1}
		result := object.MapValues(map[string]any{"a": nested}, func(value any, _ string) any {
			return value
		})
		nested["inner"] = 99
		if result["a"].(map[string]any)["inner"] != 99 {
			t.Error("MapValues should preserve reference for returned values")
		}
	})
}

// --- PickBy tests ---

func TestPickBy(t *testing.T) {
	t.Run("selects entries where predicate returns true", func(t *testing.T) {
		result := object.PickBy(map[string]any{"a": 1, "b": 2, "c": 3}, func(value any, _ string) bool {
			return value.(int) > 1
		})
		expected := map[string]any{"b": 2, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickBy result = %v, want %v", result, expected)
		}
	})

	t.Run("returns empty when nothing matches", func(t *testing.T) {
		result := object.PickBy(map[string]any{"a": 1}, func(_ any, _ string) bool {
			return false
		})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("PickBy result = %v", result)
		}
	})

	t.Run("returns all entries when everything matches", func(t *testing.T) {
		result := object.PickBy(map[string]any{"a": 1, "b": 2}, func(_ any, _ string) bool {
			return true
		})
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickBy result = %v, want %v", result, expected)
		}
	})

	t.Run("passes the key as second argument", func(t *testing.T) {
		result := object.PickBy(map[string]any{"a": 1, "b": 2}, func(_ any, key string) bool {
			return key == "a"
		})
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PickBy result = %v, want %v", result, expected)
		}
	})

	t.Run("returns empty for empty input", func(t *testing.T) {
		result := object.PickBy(map[string]any{}, func(_ any, _ string) bool {
			return true
		})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("PickBy result = %v", result)
		}
	})

	t.Run("does not mutate the input", func(t *testing.T) {
		input := map[string]any{"a": 1, "b": 2}
		object.PickBy(input, func(_ any, _ string) bool {
			return true
		})
		if !reflect.DeepEqual(input, map[string]any{"a": 1, "b": 2}) {
			t.Error("PickBy should not mutate input")
		}
	})
}

// --- OmitBy tests ---

func TestOmitBy(t *testing.T) {
	t.Run("removes entries where predicate returns true", func(t *testing.T) {
		result := object.OmitBy(map[string]any{"a": 1, "b": nil, "c": 3}, func(value any, _ string) bool {
			return value == nil
		})
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("OmitBy result = %v, want %v", result, expected)
		}
	})

	t.Run("returns all entries when predicate always false", func(t *testing.T) {
		result := object.OmitBy(map[string]any{"a": 1, "b": 2}, func(_ any, _ string) bool {
			return false
		})
		expected := map[string]any{"a": 1, "b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("OmitBy result = %v, want %v", result, expected)
		}
	})

	t.Run("returns empty when predicate always true", func(t *testing.T) {
		result := object.OmitBy(map[string]any{"a": 1, "b": 2}, func(_ any, _ string) bool {
			return true
		})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("OmitBy result = %v", result)
		}
	})

	t.Run("passes the key as second argument", func(t *testing.T) {
		result := object.OmitBy(map[string]any{"a": 1, "b": 2}, func(_ any, key string) bool {
			return key == "a"
		})
		expected := map[string]any{"b": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("OmitBy result = %v, want %v", result, expected)
		}
	})

	t.Run("returns empty for empty input", func(t *testing.T) {
		result := object.OmitBy(map[string]any{}, func(_ any, _ string) bool {
			return false
		})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("OmitBy result = %v", result)
		}
	})
}

// --- RemovePrototype tests ---

func TestRemovePrototype(t *testing.T) {
	t.Run("should remove prototype polluting properties", func(t *testing.T) {
		obj := map[string]any{
			"__proto__":   map[string]any{"polluted": true},
			"constructor": map[string]any{"prototype": map[string]any{"polluted": true}},
			"prototype":   map[string]any{"polluted": true},
			"a":           1,
		}
		result := object.RemovePrototype(obj)
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototype result = %v, want %v", result, expected)
		}
	})

	t.Run("should preserve other properties", func(t *testing.T) {
		obj := map[string]any{
			"a": 1,
			"b": "test",
			"c": true,
			"d": []any{1, 2, 3},
			"e": map[string]any{"nested": true},
		}
		result := object.RemovePrototype(obj)
		if !reflect.DeepEqual(result, obj) {
			t.Errorf("RemovePrototype result = %v, want %v", result, obj)
		}
	})

	t.Run("should not modify the original object", func(t *testing.T) {
		obj := map[string]any{"__proto__": map[string]any{"polluted": true}, "a": 1}
		object.RemovePrototype(obj)
		if _, ok := obj["__proto__"]; !ok {
			t.Error("original should retain __proto__")
		}
	})

	t.Run("should perform a shallow copy", func(t *testing.T) {
		nested := map[string]any{"b": 2}
		obj := map[string]any{"a": nested}
		result := object.RemovePrototype(obj)
		nested["b"] = 99
		if result["a"].(map[string]any)["b"] != 99 {
			t.Error("RemovePrototype should share nested references")
		}
	})

	t.Run("should handle an empty object", func(t *testing.T) {
		result := object.RemovePrototype(map[string]any{})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("RemovePrototype empty result = %v", result)
		}
	})

	t.Run("should handle mixed dangerous and safe keys", func(t *testing.T) {
		obj := map[string]any{
			"__proto__": 1, "a": 2, "constructor": 3, "b": 4, "prototype": 5, "c": 6,
		}
		result := object.RemovePrototype(obj)
		expected := map[string]any{"a": 2, "b": 4, "c": 6}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototype result = %v, want %v", result, expected)
		}
	})

	t.Run("should not share references between successive calls", func(t *testing.T) {
		obj := map[string]any{"a": 1}
		first := object.RemovePrototype(obj)
		second := object.RemovePrototype(obj)
		first["x"] = 9
		if _, ok := second["x"]; ok {
			t.Error("successive calls should not share references")
		}
	})
}

// --- RemovePrototypeDeep tests ---

func TestRemovePrototypeDeep(t *testing.T) {
	t.Run("should remove prototype polluting properties recursively", func(t *testing.T) {
		obj := map[string]any{
			"safe": map[string]any{
				"nested": map[string]any{"__proto__": map[string]any{"polluted": true}, "value": 1},
			},
			"list": []any{
				map[string]any{"constructor": map[string]any{"prototype": map[string]any{"polluted": true}}, "ok": 1},
				map[string]any{"prototype": map[string]any{"polluted": true}, "safe": 2},
			},
		}
		result := object.RemovePrototypeDeep(obj)
		expected := map[string]any{
			"safe": map[string]any{"nested": map[string]any{"value": 1}},
			"list": []any{map[string]any{"ok": 1}, map[string]any{"safe": 2}},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle arrays containing arrays and primitives", func(t *testing.T) {
		obj := map[string]any{
			"matrix": []any{
				[]any{map[string]any{"__proto__": map[string]any{"polluted": true}, "a": 1}},
				[]any{"text", 42, true, nil},
			},
		}
		result := object.RemovePrototypeDeep(obj)
		expected := map[string]any{
			"matrix": []any{
				[]any{map[string]any{"a": 1}},
				[]any{"text", 42, true, nil},
			},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should not modify the original object", func(t *testing.T) {
		obj := map[string]any{
			"safe": map[string]any{"__proto__": map[string]any{"polluted": true}, "value": 1},
		}
		object.RemovePrototypeDeep(obj)
		safe := obj["safe"].(map[string]any)
		if _, ok := safe["__proto__"]; !ok {
			t.Error("original nested map should retain __proto__")
		}
	})

	t.Run("should handle an empty object", func(t *testing.T) {
		result := object.RemovePrototypeDeep(map[string]any{})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("RemovePrototypeDeep empty result = %v", result)
		}
	})

	t.Run("should sanitize dangerous keys at multiple depths", func(t *testing.T) {
		obj := map[string]any{
			"__proto__": 1,
			"a":         map[string]any{"__proto__": 2, "b": map[string]any{"__proto__": 3, "c": 4}},
		}
		result := object.RemovePrototypeDeep(obj)
		expected := map[string]any{"a": map[string]any{"b": map[string]any{"c": 4}}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should return new plain objects (not the same reference)", func(t *testing.T) {
		nested := map[string]any{"b": 1}
		obj := map[string]any{"a": nested}
		result := object.RemovePrototypeDeep(obj)
		nested["b"] = 99
		if result["a"].(map[string]any)["b"] != 1 {
			t.Error("RemovePrototypeDeep should return new nested maps")
		}
	})

	t.Run("should recurse through arrays that contain arrays", func(t *testing.T) {
		obj := map[string]any{
			"matrix": []any{[]any{[]any{map[string]any{"__proto__": map[string]any{"polluted": true}, "deep": 1}}}},
		}
		result := object.RemovePrototypeDeep(obj)
		expected := map[string]any{"matrix": []any{[]any{[]any{map[string]any{"deep": 1}}}}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeDeep result = %v, want %v", result, expected)
		}
	})
}

// --- RemovePrototypeMap tests ---

func TestRemovePrototypeMap(t *testing.T) {
	t.Run("should remove prototype polluting properties from each object", func(t *testing.T) {
		nested := map[string]any{"keep": true}
		objects := []map[string]any{
			{"__proto__": map[string]any{"polluted": true}, "a": 1},
			{"constructor": map[string]any{"prototype": map[string]any{"polluted": true}}, "b": 2, "nested": nested},
			{"prototype": map[string]any{"polluted": true}, "c": 3},
		}
		result := object.RemovePrototypeMap(objects)
		expected := []map[string]any{
			{"a": 1},
			{"b": 2, "nested": nested},
			{"c": 3},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeMap result = %v, want %v", result, expected)
		}
		nestedRef, _ := result[1]["nested"].(map[string]any)
		nestedRef["keep"] = false
		if nested["keep"] != false {
			t.Error("RemovePrototypeMap should share nested references (shallow)")
		}
	})

	t.Run("should return an empty array when given an empty array", func(t *testing.T) {
		result := object.RemovePrototypeMap([]map[string]any{})
		if len(result) != 0 {
			t.Errorf("RemovePrototypeMap empty result = %v", result)
		}
	})

	t.Run("should preserve element order", func(t *testing.T) {
		objects := []map[string]any{{"a": 1}, {"a": 2}, {"a": 3}, {"a": 4}}
		result := object.RemovePrototypeMap(objects)
		for index, want := range []int{1, 2, 3, 4} {
			if result[index]["a"] != want {
				t.Errorf("RemovePrototypeMap order at %d = %v, want %d", index, result[index]["a"], want)
			}
		}
	})

	t.Run("should not mutate any of the input objects", func(t *testing.T) {
		objects := []map[string]any{
			{"__proto__": map[string]any{"polluted": true}, "a": 1},
			{"constructor": map[string]any{"polluted": true}, "b": 2},
		}
		object.RemovePrototypeMap(objects)
		if _, ok := objects[0]["__proto__"]; !ok {
			t.Error("input[0] should retain __proto__")
		}
		if _, ok := objects[1]["constructor"]; !ok {
			t.Error("input[1] should retain constructor")
		}
	})
}

// --- RemovePrototypeMapDeep tests ---

func TestRemovePrototypeMapDeep(t *testing.T) {
	t.Run("should remove prototype polluting properties recursively from each object", func(t *testing.T) {
		objects := []map[string]any{
			{"safe": map[string]any{"__proto__": map[string]any{"polluted": true}, "value": 1}},
			{
				"list": []any{map[string]any{"prototype": map[string]any{"polluted": true}, "ok": 1}},
				"meta": map[string]any{"constructor": map[string]any{"prototype": map[string]any{"polluted": true}}, "keep": 2},
			},
		}
		result := object.RemovePrototypeMapDeep(objects)
		expected := []map[string]any{
			{"safe": map[string]any{"value": 1}},
			{"list": []any{map[string]any{"ok": 1}}, "meta": map[string]any{"keep": 2}},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeMapDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should return an empty array when given an empty array", func(t *testing.T) {
		result := object.RemovePrototypeMapDeep([]map[string]any{})
		if len(result) != 0 {
			t.Errorf("RemovePrototypeMapDeep empty result = %v", result)
		}
	})

	t.Run("should sanitize dangerous keys nested inside arrays", func(t *testing.T) {
		objects := []map[string]any{
			{"items": []any{
				map[string]any{"__proto__": map[string]any{"polluted": true}, "a": 1},
				map[string]any{"constructor": map[string]any{"polluted": true}, "b": 2},
			}},
		}
		result := object.RemovePrototypeMapDeep(objects)
		expected := []map[string]any{
			{"items": []any{map[string]any{"a": 1}, map[string]any{"b": 2}}},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeMapDeep result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle arrays that mix sanitized and already-safe objects", func(t *testing.T) {
		objects := []map[string]any{
			{"clean": map[string]any{"a": 1}},
			{"dirty": map[string]any{"__proto__": map[string]any{"polluted": true}, "b": 2}},
		}
		result := object.RemovePrototypeMapDeep(objects)
		expected := []map[string]any{
			{"clean": map[string]any{"a": 1}},
			{"dirty": map[string]any{"b": 2}},
		}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("RemovePrototypeMapDeep result = %v, want %v", result, expected)
		}
	})
}

// --- GetObjectsCommon tests ---

func TestGetObjectsCommon(t *testing.T) {
	t.Run("should find common key-value pairs between two objects", func(t *testing.T) {
		result := object.GetObjectsCommon(map[string]any{"a": 1, "b": 2}, map[string]any{"a": 1, "c": 3})
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsCommon result = %v, want %v", result, expected)
		}
	})

	t.Run("should find common key-value pairs among three objects", func(t *testing.T) {
		result := object.GetObjectsCommon(
			map[string]any{"a": 1, "b": 2, "c": 3},
			map[string]any{"a": 1, "b": 2, "d": 4},
			map[string]any{"a": 1, "e": 5},
		)
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsCommon result = %v, want %v", result, expected)
		}
	})

	t.Run("should return a shallow copy for a single object", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := object.GetObjectsCommon(obj)
		if !reflect.DeepEqual(result, obj) {
			t.Errorf("GetObjectsCommon single = %v, want %v", result, obj)
		}
		result["x"] = 9
		if _, ok := obj["x"]; ok {
			t.Error("GetObjectsCommon single should be a copy")
		}
	})

	t.Run("should return empty object when one input is empty", func(t *testing.T) {
		if !reflect.DeepEqual(object.GetObjectsCommon(map[string]any{"a": 1, "b": 2}, map[string]any{}), map[string]any{}) {
			t.Error("GetObjectsCommon with empty second should be empty")
		}
		if !reflect.DeepEqual(object.GetObjectsCommon(map[string]any{}, map[string]any{"a": 1, "b": 2}), map[string]any{}) {
			t.Error("GetObjectsCommon with empty first should be empty")
		}
	})

	t.Run("should exclude keys with different values", func(t *testing.T) {
		result := object.GetObjectsCommon(map[string]any{"a": 1, "b": 2}, map[string]any{"a": 1, "b": 5})
		expected := map[string]any{"a": 1}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsCommon result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle falsy values correctly", func(t *testing.T) {
		result := object.GetObjectsCommon(
			map[string]any{"a": nil, "c": 0, "d": false, "e": ""},
			map[string]any{"a": nil, "c": 0, "d": false, "e": ""},
		)
		expected := map[string]any{"a": nil, "c": 0, "d": false, "e": ""}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsCommon result = %v, want %v", result, expected)
		}
	})

	t.Run("should find common nested objects recursively", func(t *testing.T) {
		result := object.GetObjectsCommon(
			map[string]any{"a": map[string]any{"b": 1, "c": 2}, "d": 3},
			map[string]any{"a": map[string]any{"b": 1, "d": 4}, "d": 3},
		)
		expected := map[string]any{"a": map[string]any{"b": 1}, "d": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsCommon result = %v, want %v", result, expected)
		}
	})

	t.Run("should exclude key when recursive result is empty", func(t *testing.T) {
		result := object.GetObjectsCommon(map[string]any{"a": map[string]any{"b": 1}}, map[string]any{"a": map[string]any{"c": 2}})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("GetObjectsCommon result = %v, want empty", result)
		}
	})

	t.Run("should handle deeply nested objects", func(t *testing.T) {
		result := object.GetObjectsCommon(
			map[string]any{"a": map[string]any{"b": map[string]any{"c": map[string]any{"d": 1, "e": 2}}}},
			map[string]any{"a": map[string]any{"b": map[string]any{"c": map[string]any{"d": 1, "f": 3}}}},
		)
		expected := map[string]any{"a": map[string]any{"b": map[string]any{"c": map[string]any{"d": 1}}}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsCommon result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle mixed nested and primitive values", func(t *testing.T) {
		result := object.GetObjectsCommon(map[string]any{"a": map[string]any{"b": 1}}, map[string]any{"a": "hello"})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("GetObjectsCommon result = %v, want empty", result)
		}
	})

	t.Run("should compare slices by reference", func(t *testing.T) {
		arr := []any{1, 2, 3}
		if !reflect.DeepEqual(object.GetObjectsCommon(map[string]any{"a": arr}, map[string]any{"a": arr}), map[string]any{"a": arr}) {
			t.Error("same slice reference should be common")
		}
		if !reflect.DeepEqual(object.GetObjectsCommon(map[string]any{"a": []any{1, 2, 3}}, map[string]any{"a": []any{1, 2, 3}}), map[string]any{}) {
			t.Error("distinct slices should not be common")
		}
	})

	t.Run("should compare using strict equality (no coercion)", func(t *testing.T) {
		if !reflect.DeepEqual(object.GetObjectsCommon(map[string]any{"a": 1}, map[string]any{"a": "1"}), map[string]any{}) {
			t.Error("1 and \"1\" should not be common")
		}
		if !reflect.DeepEqual(object.GetObjectsCommon(map[string]any{"a": true}, map[string]any{"a": 1}), map[string]any{}) {
			t.Error("true and 1 should not be common")
		}
	})

	t.Run("should handle NaN reference equality", func(t *testing.T) {
		result := object.GetObjectsCommon(map[string]any{"a": math.NaN()}, map[string]any{"a": math.NaN()})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("GetObjectsCommon NaN = %v, want empty", result)
		}
	})

	t.Run("should not mutate original objects", func(t *testing.T) {
		obj1 := map[string]any{"a": 1, "b": map[string]any{"c": 2}}
		obj2 := map[string]any{"a": 1, "b": map[string]any{"c": 2, "d": 3}}
		object.GetObjectsCommon(obj1, obj2)
		if len(obj1["b"].(map[string]any)) != 1 {
			t.Error("obj1 should not be mutated")
		}
		if len(obj2["b"].(map[string]any)) != 2 {
			t.Error("obj2 should not be mutated")
		}
	})
}

// --- GetObjectsDiff tests ---

func TestGetObjectsDiff(t *testing.T) {
	t.Run("should find diff key-value pairs between two objects", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": 1, "b": 2}, map[string]any{"b": 2, "c": 3})
		expected := map[string]any{"a": 1, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should find diff key-value pairs among three objects", func(t *testing.T) {
		result := object.GetObjectsDiff(
			map[string]any{"a": 1, "b": 2, "c": 3},
			map[string]any{"b": 2, "c": 3, "d": 4},
			map[string]any{"c": 3, "d": 4, "e": 5},
		)
		expected := map[string]any{"a": 1, "e": 5}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should return a shallow copy for a single object", func(t *testing.T) {
		obj := map[string]any{"a": 1, "b": 2}
		result := object.GetObjectsDiff(obj)
		if !reflect.DeepEqual(result, obj) {
			t.Errorf("GetObjectsDiff single = %v, want %v", result, obj)
		}
		result["x"] = 9
		if _, ok := obj["x"]; ok {
			t.Error("GetObjectsDiff single should be a copy")
		}
	})

	t.Run("should return empty object when objects are identical", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": 1, "b": 2}, map[string]any{"a": 1, "b": 2})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("GetObjectsDiff identical = %v, want empty", result)
		}
	})

	t.Run("should return all pairs when no keys overlap", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": 1}, map[string]any{"b": 2}, map[string]any{"c": 3})
		expected := map[string]any{"a": 1, "b": 2, "c": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle empty objects", func(t *testing.T) {
		if !reflect.DeepEqual(object.GetObjectsDiff(map[string]any{}, map[string]any{"a": 1}), map[string]any{"a": 1}) {
			t.Error("empty first diff failed")
		}
		if !reflect.DeepEqual(object.GetObjectsDiff(map[string]any{"a": 1}, map[string]any{}), map[string]any{"a": 1}) {
			t.Error("empty second diff failed")
		}
		if !reflect.DeepEqual(object.GetObjectsDiff(map[string]any{}, map[string]any{}), map[string]any{}) {
			t.Error("both empty diff failed")
		}
	})

	t.Run("should use last value when same key has different unique values", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": 1}, map[string]any{"a": 2})
		expected := map[string]any{"a": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle same key with shared and unique values across three objects", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": 1}, map[string]any{"a": 1}, map[string]any{"a": 2})
		expected := map[string]any{"a": 2}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should find diff in nested objects recursively", func(t *testing.T) {
		result := object.GetObjectsDiff(
			map[string]any{"a": map[string]any{"b": 1, "c": 2}, "d": 3},
			map[string]any{"a": map[string]any{"b": 1, "d": 4}, "d": 3},
		)
		expected := map[string]any{"a": map[string]any{"c": 2, "d": 4}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should exclude key when nested diff is empty", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": map[string]any{"b": 1}}, map[string]any{"a": map[string]any{"b": 1}})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("GetObjectsDiff result = %v, want empty", result)
		}
	})

	t.Run("should handle deeply nested objects", func(t *testing.T) {
		result := object.GetObjectsDiff(
			map[string]any{"a": map[string]any{"b": map[string]any{"c": map[string]any{"d": 1, "e": 2}}}},
			map[string]any{"a": map[string]any{"b": map[string]any{"c": map[string]any{"d": 1, "f": 3}}}},
		)
		expected := map[string]any{"a": map[string]any{"b": map[string]any{"c": map[string]any{"e": 2, "f": 3}}}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nested objects among three objects", func(t *testing.T) {
		result := object.GetObjectsDiff(
			map[string]any{"a": map[string]any{"b": 1, "c": 2}},
			map[string]any{"a": map[string]any{"b": 1, "d": 3}},
			map[string]any{"a": map[string]any{"b": 1, "e": 4}},
		)
		expected := map[string]any{"a": map[string]any{"c": 2, "d": 3, "e": 4}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle nested shared values across three objects", func(t *testing.T) {
		result := object.GetObjectsDiff(
			map[string]any{"a": map[string]any{"b": 1}},
			map[string]any{"a": map[string]any{"b": 1}},
			map[string]any{"a": map[string]any{"b": 2}},
		)
		expected := map[string]any{"a": map[string]any{"b": 2}}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle mixed nested and primitive values", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": map[string]any{"b": 1}}, map[string]any{"a": "hello"})
		expected := map[string]any{"a": "hello"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should compare slices by reference", func(t *testing.T) {
		arr := []any{1, 2, 3}
		if !reflect.DeepEqual(object.GetObjectsDiff(map[string]any{"a": arr}, map[string]any{"a": arr}), map[string]any{}) {
			t.Error("same slice reference should produce empty diff")
		}
		result := object.GetObjectsDiff(map[string]any{"a": []any{1, 2, 3}}, map[string]any{"a": []any{1, 2, 3}})
		if !reflect.DeepEqual(result, map[string]any{"a": []any{1, 2, 3}}) {
			t.Errorf("distinct slices diff = %v", result)
		}
	})

	t.Run("should compare using strict equality (no coercion)", func(t *testing.T) {
		if !reflect.DeepEqual(object.GetObjectsDiff(map[string]any{"a": 1}, map[string]any{"a": "1"}), map[string]any{"a": "1"}) {
			t.Error("1 vs \"1\" diff failed")
		}
		if !reflect.DeepEqual(object.GetObjectsDiff(map[string]any{"a": true}, map[string]any{"a": 1}), map[string]any{"a": 1}) {
			t.Error("true vs 1 diff failed")
		}
	})

	t.Run("should handle NaN with SameValueZero semantics", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"a": math.NaN()}, map[string]any{"a": math.NaN()})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("GetObjectsDiff NaN = %v, want empty", result)
		}
	})

	t.Run("should include keys present in only some of the objects", func(t *testing.T) {
		result := object.GetObjectsDiff(
			map[string]any{"a": 1, "only1": "x"},
			map[string]any{"a": 1, "only2": "y"},
			map[string]any{"a": 1, "only3": "z"},
		)
		expected := map[string]any{"only1": "x", "only2": "y", "only3": "z"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should keep the last unique value when 4+ objects conflict", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"x": 1}, map[string]any{"x": 2}, map[string]any{"x": 3}, map[string]any{"x": 4})
		expected := map[string]any{"x": 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should not include a key when all values identical across many objects", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"x": 1}, map[string]any{"x": 1}, map[string]any{"x": 1}, map[string]any{"x": 1})
		if !reflect.DeepEqual(result, map[string]any{}) {
			t.Errorf("GetObjectsDiff result = %v, want empty", result)
		}
	})

	t.Run("should pair shared values correctly when shared value is not first", func(t *testing.T) {
		result := object.GetObjectsDiff(map[string]any{"x": 3}, map[string]any{"x": 1}, map[string]any{"x": 1})
		expected := map[string]any{"x": 3}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should handle keys that exist in only a subset of the inputs", func(t *testing.T) {
		if !reflect.DeepEqual(object.GetObjectsDiff(map[string]any{"a": 1}, map[string]any{"b": 2}, map[string]any{"a": 1, "b": 2}), map[string]any{}) {
			t.Error("subset diff failed (expected empty)")
		}
		result := object.GetObjectsDiff(map[string]any{"a": 1}, map[string]any{"b": 2}, map[string]any{"a": 3, "b": 4})
		expected := map[string]any{"a": 3, "b": 4}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("GetObjectsDiff result = %v, want %v", result, expected)
		}
	})

	t.Run("should not mutate original objects", func(t *testing.T) {
		obj1 := map[string]any{"a": 1, "b": map[string]any{"c": 2}}
		obj2 := map[string]any{"a": 1, "b": map[string]any{"c": 2, "d": 3}}
		object.GetObjectsDiff(obj1, obj2)
		if len(obj1["b"].(map[string]any)) != 1 {
			t.Error("obj1 should not be mutated")
		}
		if len(obj2["b"].(map[string]any)) != 2 {
			t.Error("obj2 should not be mutated")
		}
	})
}

// --- PathSegments tests ---

func TestPathSegments(t *testing.T) {
	t.Run("splits a dotted string into segments", func(t *testing.T) {
		result := object.PathSegments("a.b.c")
		expected := []string{"a", "b", "c"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PathSegments result = %v, want %v", result, expected)
		}
	})

	t.Run("returns the input slice unchanged", func(t *testing.T) {
		input := []string{"a", "b"}
		result := object.PathSegments(input)
		if &result[0] != &input[0] {
			t.Error("PathSegments should return the same slice reference")
		}
		if !reflect.DeepEqual(result, input) {
			t.Errorf("PathSegments result = %v, want %v", result, input)
		}
	})

	t.Run("returns a single-element slice for a non-dotted string", func(t *testing.T) {
		result := object.PathSegments("a")
		expected := []string{"a"}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PathSegments result = %v, want %v", result, expected)
		}
	})

	t.Run("splits empty string into a single empty segment", func(t *testing.T) {
		result := object.PathSegments("")
		expected := []string{""}
		if !reflect.DeepEqual(result, expected) {
			t.Errorf("PathSegments result = %v, want %v", result, expected)
		}
	})
}
