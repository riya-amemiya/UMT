package object

// Merge performs a shallow merge of multiple maps into a single map.
// Later maps override earlier ones for duplicate keys.
// Returns a new map; the originals are not modified.
func Merge(maps ...map[string]any) map[string]any {
	result := make(map[string]any)
	for _, m := range maps {
		for k, v := range m {
			result[k] = v
		}
	}
	return result
}

// isPlainObject checks if a value is a map[string]any.
func isPlainObject(v any) (map[string]any, bool) {
	m, ok := v.(map[string]any)
	return m, ok
}

// MergeDeep performs a deep merge of multiple maps into a single map.
// Nested maps are merged recursively; non-map values are overridden.
// Returns a new map; the originals are not modified.
func MergeDeep(maps ...map[string]any) map[string]any {
	if len(maps) == 0 {
		return make(map[string]any)
	}

	result := make(map[string]any)
	// Copy the first map
	for k, v := range maps[0] {
		result[k] = v
	}

	for i := 1; i < len(maps); i++ {
		source := maps[i]
		for k, sourceVal := range source {
			targetVal, targetExists := result[k]
			targetMap, targetIsMap := isPlainObject(targetVal)
			sourceMap, sourceIsMap := isPlainObject(sourceVal)

			if targetExists && targetIsMap && sourceIsMap {
				result[k] = MergeDeep(targetMap, sourceMap)
			} else {
				result[k] = sourceVal
			}
		}
	}

	return result
}

// Pick creates a new map containing only the specified keys from the source map.
func Pick(m map[string]any, keys ...string) map[string]any {
	result := make(map[string]any)
	for _, key := range keys {
		if val, ok := m[key]; ok {
			result[key] = val
		}
	}
	return result
}

// Omit creates a new map without the specified keys.
func Omit(m map[string]any, keys ...string) map[string]any {
	result := make(map[string]any)
	omitSet := make(map[string]bool)
	for _, key := range keys {
		omitSet[key] = true
	}
	for k, v := range m {
		if !omitSet[k] {
			result[k] = v
		}
	}
	return result
}

// Has checks if a map contains the specified key.
func Has(m map[string]any, key string) bool {
	if m == nil {
		return false
	}
	_, ok := m[key]
	return ok
}

// IsEmpty checks if a map has no entries.
func IsEmpty(m map[string]any) bool {
	return len(m) == 0
}

// KeyBy creates a map from a slice by applying the given key function to each element.
// If multiple elements produce the same key, the last one wins.
func KeyBy[T any](items []T, keyFn func(T) string) map[string]T {
	result := make(map[string]T)
	for _, item := range items {
		key := keyFn(item)
		result[key] = item
	}
	return result
}
