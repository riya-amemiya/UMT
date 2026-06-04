package object

import (
	"math"
	"reflect"
)

// GetObjectsCommon extracts the key-value pairs common to all input maps.
//
// A pair is common when the key exists in every map and the value is strictly
// equal across all maps. When all values for a key are plain maps, the function
// recurses to find the common subset; if that subset is empty, the key is
// excluded. With a single input map a shallow copy is returned.
//
// Equality follows the TypeScript ===: primitives compare by value, NaN never
// equals itself, and slices/maps compare by reference (the same header is
// equal, distinct literals are not).
//
// Example:
//
//	object.GetObjectsCommon(map[string]any{"a": 1, "b": 2}, map[string]any{"a": 1, "c": 3})
//	// map[string]any{"a": 1}
func GetObjectsCommon(object map[string]any, objects ...map[string]any) map[string]any {
	if len(objects) == 0 {
		result := make(map[string]any, len(object))
		for key, value := range object {
			result[key] = value
		}
		return result
	}

	result := make(map[string]any)

	for key, value := range object {
		isCommon := true
		_, allPlainObjects := isPlainMap(value)

		for _, other := range objects {
			otherValue, exists := other[key]
			if !exists {
				isCommon = false
				break
			}

			if _, ok := isPlainMap(otherValue); !ok {
				allPlainObjects = false
			}

			if !allPlainObjects && !strictEqual(otherValue, value) {
				isCommon = false
				break
			}
		}

		if !isCommon {
			continue
		}

		if allPlainObjects {
			nestedInputs := make([]map[string]any, len(objects))
			for index, other := range objects {
				nestedInputs[index], _ = isPlainMap(other[key])
			}
			currentMap, _ := isPlainMap(value)
			nested := GetObjectsCommon(currentMap, nestedInputs...)
			if len(nested) > 0 {
				result[key] = nested
			}
		} else {
			result[key] = value
		}
	}

	return result
}

// GetObjectsDiff extracts the key-value pairs that appear in exactly one input
// map.
//
// A pair is "shared" when the same key with the same value exists in two or
// more maps; only pairs unique to a single map are returned. When all values
// for a key are plain maps, the function recurses to find the diff subset; if
// that subset is empty, the key is excluded. When multiple unique pairs share
// the same key with different values, the last value wins. With a single input
// map a shallow copy is returned.
//
// Value counting uses SameValueZero semantics (NaN equals NaN), matching the
// reference implementation's internal Map.
//
// Example:
//
//	object.GetObjectsDiff(map[string]any{"a": 1, "b": 2}, map[string]any{"b": 2, "c": 3})
//	// map[string]any{"a": 1, "c": 3}
func GetObjectsDiff(object map[string]any, objects ...map[string]any) map[string]any {
	allObjects := append([]map[string]any{object}, objects...)

	if len(allObjects) == 1 {
		result := make(map[string]any, len(object))
		for key, value := range object {
			result[key] = value
		}
		return result
	}

	orderedKeys := make([]string, 0)
	seenKeys := make(map[string]struct{})
	for _, current := range allObjects {
		for key := range current {
			if _, seen := seenKeys[key]; !seen {
				seenKeys[key] = struct{}{}
				orderedKeys = append(orderedKeys, key)
			}
		}
	}

	result := make(map[string]any)

	for _, key := range orderedKeys {
		values := make([]any, 0)
		for _, current := range allObjects {
			if value, exists := current[key]; exists {
				values = append(values, value)
			}
		}

		if len(values) == 1 {
			result[key] = values[0]
			continue
		}

		allPlain := true
		for _, value := range values {
			if _, ok := isPlainMap(value); !ok {
				allPlain = false
				break
			}
		}

		if allPlain {
			nestedInputs := make([]map[string]any, len(values)-1)
			for index := 1; index < len(values); index++ {
				nestedInputs[index-1], _ = isPlainMap(values[index])
			}
			head, _ := isPlainMap(values[0])
			nested := GetObjectsDiff(head, nestedInputs...)
			if len(nested) > 0 {
				result[key] = nested
			}
			continue
		}

		var lastUniqueValue any
		hasUnique := false

		// Count occurrences using SameValueZero semantics, matching the
		// reference implementation's internal Map (NaN equals NaN).
		type counted struct {
			value any
			count int
		}
		buckets := make([]counted, 0, len(values))
		for _, value := range values {
			found := false
			for index := range buckets {
				if sameValueZero(buckets[index].value, value) {
					buckets[index].count++
					found = true
					break
				}
			}
			if !found {
				buckets = append(buckets, counted{value: value, count: 1})
			}
		}
		for _, bucket := range buckets {
			if bucket.count == 1 {
				lastUniqueValue = bucket.value
				hasUnique = true
			}
		}

		if hasUnique {
			result[key] = lastUniqueValue
		}
	}

	return result
}

// strictEqual reports whether two values are strictly equal in the sense of the
// TypeScript === operator: primitives compare by value, NaN never equals
// itself, and slices/maps compare by reference identity.
func strictEqual(a, b any) bool {
	if a == nil || b == nil {
		return a == nil && b == nil
	}

	af, aIsFloat := a.(float64)
	bf, bIsFloat := b.(float64)
	if aIsFloat && bIsFloat {
		if math.IsNaN(af) || math.IsNaN(bf) {
			return false
		}
		return af == bf
	}

	if isReferenceType(a) || isReferenceType(b) {
		return referenceEqual(a, b)
	}

	return a == b
}

// sameValueZero reports whether two values are equal under SameValueZero
// semantics: like strictEqual but NaN equals NaN.
func sameValueZero(a, b any) bool {
	af, aIsFloat := a.(float64)
	bf, bIsFloat := b.(float64)
	if aIsFloat && bIsFloat {
		if math.IsNaN(af) && math.IsNaN(bf) {
			return true
		}
		return af == bf
	}

	if a == nil || b == nil {
		return a == nil && b == nil
	}

	if isReferenceType(a) || isReferenceType(b) {
		return referenceEqual(a, b)
	}

	return a == b
}

// isReferenceType reports whether the value is held by reference in Go (slice,
// map, or pointer) and therefore must be compared by identity rather than by
// the == operator, which panics for slices and maps.
func isReferenceType(value any) bool {
	switch reflect.TypeOf(value).Kind() {
	case reflect.Slice, reflect.Map, reflect.Ptr:
		return true
	default:
		return false
	}
}

// referenceEqual reports whether two reference values point at the same
// underlying data, mirroring JavaScript reference equality for objects/arrays.
func referenceEqual(a, b any) bool {
	ra := reflect.ValueOf(a)
	rb := reflect.ValueOf(b)
	if ra.Kind() != rb.Kind() {
		return false
	}
	switch ra.Kind() {
	case reflect.Slice, reflect.Map, reflect.Ptr:
		return ra.Pointer() == rb.Pointer()
	default:
		return false
	}
}
