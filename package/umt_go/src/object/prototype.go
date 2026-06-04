package object

// RemovePrototype creates a new map with the same entries as the given map, but
// with the prototype-polluting keys ("__proto__", "constructor", "prototype")
// removed. The copy is shallow; nested values are shared by reference.
//
// Example:
//
//	object.RemovePrototype(map[string]any{"__proto__": map[string]any{"polluted": true}, "a": 1, "b": 2})
//	// map[string]any{"a": 1, "b": 2}
func RemovePrototype(object map[string]any) map[string]any {
	result := make(map[string]any)
	for key, value := range object {
		if _, dangerous := dangerousKeys[key]; dangerous {
			continue
		}
		result[key] = value
	}
	return result
}

// RemovePrototypeDeep creates a new map with the prototype-polluting keys
// removed recursively from nested maps and from maps inside slices. Non-map,
// non-slice values are passed through by reference.
//
// Example:
//
//	object.RemovePrototypeDeep(map[string]any{"a": map[string]any{"__proto__": 2, "b": 3}})
//	// map[string]any{"a": map[string]any{"b": 3}}
func RemovePrototypeDeep(object map[string]any) map[string]any {
	cloned := removePrototypeDeepValue(object)
	if out, ok := cloned.(map[string]any); ok {
		return out
	}
	return make(map[string]any)
}

// removePrototypeDeepValue recursively sanitizes maps and slices.
func removePrototypeDeepValue(value any) any {
	switch typed := value.(type) {
	case map[string]any:
		result := make(map[string]any)
		for key, val := range typed {
			if _, dangerous := dangerousKeys[key]; dangerous {
				continue
			}
			result[key] = removePrototypeDeepValue(val)
		}
		return result
	case []any:
		result := make([]any, len(typed))
		for index, element := range typed {
			result[index] = removePrototypeDeepValue(element)
		}
		return result
	default:
		return value
	}
}

// RemovePrototypeMap returns a new slice in which each map has its
// prototype-polluting keys removed shallowly.
//
// Example:
//
//	object.RemovePrototypeMap([]map[string]any{{"__proto__": 1, "a": 2}})
//	// []map[string]any{{"a": 2}}
func RemovePrototypeMap(objects []map[string]any) []map[string]any {
	result := make([]map[string]any, len(objects))
	for index, object := range objects {
		result[index] = RemovePrototype(object)
	}
	return result
}

// RemovePrototypeMapDeep returns a new slice in which each map has its
// prototype-polluting keys removed recursively.
//
// Example:
//
//	object.RemovePrototypeMapDeep([]map[string]any{{"a": map[string]any{"__proto__": 1, "b": 2}}})
//	// []map[string]any{{"a": map[string]any{"b": 2}}}
func RemovePrototypeMapDeep(objects []map[string]any) []map[string]any {
	result := make([]map[string]any, len(objects))
	for index, object := range objects {
		result[index] = RemovePrototypeDeep(object)
	}
	return result
}
