package object

import (
	"fmt"
	"strings"
)

// dangerousKeys lists the prototype-polluting property names that the
// removePrototype* helpers strip from objects.
//
// In JavaScript these keys ("__proto__", "constructor", "prototype") can be
// abused to mutate Object.prototype. Go maps have no such prototype chain, but
// the helpers still drop these keys to preserve the reference TypeScript
// behavior.
var dangerousKeys = map[string]struct{}{
	"__proto__":   {},
	"constructor": {},
	"prototype":   {},
}

// isPlainMap reports whether the value is a map[string]any, the Go counterpart
// of a TypeScript plain object. Arrays, Dates, Maps, Sets, and primitives are
// not plain maps.
func isPlainMap(v any) (map[string]any, bool) {
	m, ok := v.(map[string]any)
	return m, ok
}

// IsPlainObject reports whether the value is a plain object, i.e. a
// map[string]any in Go. It mirrors the TypeScript isPlainObject, which returns
// true only for object literals / null-prototype objects and false for arrays,
// Date, Map, Set, functions, and primitives.
//
// Example:
//
//	object.IsPlainObject(map[string]any{"a": 1}) // true
//	object.IsPlainObject([]any{1, 2})            // false
//	object.IsPlainObject(nil)                    // false
func IsPlainObject(value any) bool {
	_, ok := value.(map[string]any)
	return ok
}

// DeepClone creates a deep copy of the given value. Nested maps
// (map[string]any) and slices ([]any) are cloned recursively; all other values
// (primitives and opaque references) are returned unchanged.
//
// It mirrors the TypeScript deepClone for the Go data model: plain objects and
// arrays are duplicated so that mutating the clone never affects the original.
//
// Example:
//
//	original := map[string]any{"a": 1, "b": map[string]any{"c": 2}}
//	cloned := object.DeepClone(original)
//	cloned["b"].(map[string]any)["c"] = 99
//	// original["b"].(map[string]any)["c"] is still 2
func DeepClone[T any](value T) T {
	cloned := deepCloneValue(any(value))
	if out, ok := cloned.(T); ok {
		return out
	}
	return value
}

// deepCloneValue recursively clones maps and slices, returning other values
// untouched.
func deepCloneValue(value any) any {
	switch typed := value.(type) {
	case map[string]any:
		result := make(map[string]any, len(typed))
		for key, val := range typed {
			result[key] = deepCloneValue(val)
		}
		return result
	case []any:
		result := make([]any, len(typed))
		for index, element := range typed {
			result[index] = deepCloneValue(element)
		}
		return result
	default:
		return value
	}
}

// FlattenObject recursively flattens a nested map into a single-level map keyed
// by joined paths. Slices and non-map values are kept as-is. Empty nested maps
// are treated as leaf values.
//
// The optional separator joins path segments and defaults to ".".
//
// Example:
//
//	object.FlattenObject(map[string]any{"a": map[string]any{"b": map[string]any{"c": 1}}, "d": 2})
//	// map[string]any{"a.b.c": 1, "d": 2}
func FlattenObject(object map[string]any, separator ...string) map[string]any {
	sep := "."
	if len(separator) > 0 {
		sep = separator[0]
	}

	result := make(map[string]any)
	var walk func(current map[string]any, prefix string)
	walk = func(current map[string]any, prefix string) {
		for key, value := range current {
			nextKey := key
			if prefix != "" {
				nextKey = prefix + sep + key
			}
			if nested, ok := isPlainMap(value); ok && len(nested) > 0 {
				walk(nested, nextKey)
			} else {
				result[nextKey] = value
			}
		}
	}
	walk(object, "")
	return result
}

// UnflattenObject reconstructs a nested map from a flat path-keyed map.
//
// The optional separator splits the path keys and defaults to ".".
//
// Example:
//
//	object.UnflattenObject(map[string]any{"a.b.c": 1, "d": 2})
//	// map[string]any{"a": map[string]any{"b": map[string]any{"c": 1}}, "d": 2}
func UnflattenObject(flat map[string]any, separator ...string) map[string]any {
	sep := "."
	if len(separator) > 0 {
		sep = separator[0]
	}

	result := make(map[string]any)
	for key, value := range flat {
		Set(result, strings.Split(key, sep), value)
	}
	return result
}

// Get reads a deeply nested value by a dot-separated string path or a slice of
// path segments. It returns the default value (if provided) when any segment is
// missing, when traversal hits a nil value, or when the resolved value is nil.
//
// Example:
//
//	object.Get(map[string]any{"a": map[string]any{"b": map[string]any{"c": 1}}}, "a.b.c") // 1
//	object.Get(map[string]any{"a": map[string]any{"b": 1}}, []string{"a", "x"}, 0)        // 0
func Get(object any, path any, defaultValue ...any) any {
	var fallback any
	if len(defaultValue) > 0 {
		fallback = defaultValue[0]
	}

	segments := PathSegments(path)
	current := object
	for _, key := range segments {
		if current == nil {
			return fallback
		}
		asMap, ok := current.(map[string]any)
		if !ok {
			return fallback
		}
		current = asMap[key]
	}
	if current == nil {
		return fallback
	}
	return current
}

// Set assigns a value at a deeply nested path on a map, mutating it in place and
// creating intermediate maps as needed. The path may be a dot-separated string
// or a slice of path segments. It returns the same map reference for chaining.
//
// Example:
//
//	object.Set(map[string]any{}, "a.b.c", 1) // map[string]any{"a": map[string]any{"b": map[string]any{"c": 1}}}
func Set(object map[string]any, path any, value any) map[string]any {
	segments := PathSegments(path)
	if len(segments) == 0 {
		return object
	}

	current := object
	for index := 0; index < len(segments)-1; index++ {
		key := segments[index]
		next, ok := current[key].(map[string]any)
		if !ok {
			next = make(map[string]any)
			current[key] = next
		}
		current = next
	}
	current[segments[len(segments)-1]] = value
	return object
}

// PathSegments normalizes a path argument into a slice of string segments. A
// string is split on ".", a []string is returned as-is, and anything else
// yields an empty slice.
//
// Example:
//
//	object.PathSegments("a.b.c")            // []string{"a", "b", "c"}
//	object.PathSegments([]string{"a", "b"}) // []string{"a", "b"}
func PathSegments(path any) []string {
	switch typed := path.(type) {
	case string:
		return strings.Split(typed, ".")
	case []string:
		return typed
	default:
		return nil
	}
}

// Invert creates a new map with keys and values swapped. When multiple keys
// share the same value, later entries overwrite earlier ones. Values are
// stringified to be usable as map keys, matching how JavaScript coerces
// property keys.
//
// Example:
//
//	object.Invert(map[string]any{"a": 1, "b": 2}) // map[string]string{"1": "a", "2": "b"}
func Invert(object map[string]any) map[string]string {
	result := make(map[string]string, len(object))
	for key, value := range object {
		result[stringifyKey(value)] = key
	}
	return result
}

// stringifyKey converts a value into the string form JavaScript would use when
// it becomes an object property key.
func stringifyKey(value any) string {
	if str, ok := value.(string); ok {
		return str
	}
	return fmt.Sprintf("%v", value)
}

// MapKeys creates a new map with the same values, but with keys transformed by
// the provided function. The function receives the value and the original key.
// When two keys map to the same new key, the later entry wins.
//
// Example:
//
//	object.MapKeys(map[string]any{"a": 1, "b": 2}, func(_ any, key string) string {
//		return strings.ToUpper(key)
//	}) // map[string]any{"A": 1, "B": 2}
func MapKeys(object map[string]any, function func(value any, key string) string) map[string]any {
	result := make(map[string]any, len(object))
	for key, value := range object {
		result[function(value, key)] = value
	}
	return result
}

// MapValues creates a new map with the same keys, but with values transformed
// by the provided function. The function receives the value and the key.
//
// Example:
//
//	object.MapValues(map[string]any{"a": 1, "b": 2}, func(value any, _ string) any {
//		return value.(int) * 2
//	}) // map[string]any{"a": 2, "b": 4}
func MapValues(object map[string]any, function func(value any, key string) any) map[string]any {
	result := make(map[string]any, len(object))
	for key, value := range object {
		result[key] = function(value, key)
	}
	return result
}

// PickBy creates a new map containing only the entries for which the predicate
// returns true. The predicate receives the value and the key.
//
// Example:
//
//	object.PickBy(map[string]any{"a": 1, "b": 2, "c": 3}, func(value any, _ string) bool {
//		return value.(int) > 1
//	}) // map[string]any{"b": 2, "c": 3}
func PickBy(object map[string]any, predicate func(value any, key string) bool) map[string]any {
	result := make(map[string]any)
	for key, value := range object {
		if predicate(value, key) {
			result[key] = value
		}
	}
	return result
}

// OmitBy creates a new map containing only the entries for which the predicate
// returns false. The predicate receives the value and the key.
//
// Example:
//
//	object.OmitBy(map[string]any{"a": 1, "b": nil, "c": 3}, func(value any, _ string) bool {
//		return value == nil
//	}) // map[string]any{"a": 1, "c": 3}
func OmitBy(object map[string]any, predicate func(value any, key string) bool) map[string]any {
	result := make(map[string]any)
	for key, value := range object {
		if !predicate(value, key) {
			result[key] = value
		}
	}
	return result
}
