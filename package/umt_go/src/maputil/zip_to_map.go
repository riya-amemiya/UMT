package maputil

// ZipToMap creates an OrderedMap from two slices, using the first as keys and
// the second as values. Pairs are formed at corresponding positions; iteration
// stops at the shorter length, matching the behavior of zip. Duplicate keys
// keep the latest value while retaining the key's first-seen position.
//
// Example:
//
//	ZipToMap([]string{"a", "b"}, []int{1, 2})    // OrderedMap { "a" => 1, "b" => 2 }
//	ZipToMap([]string{"a", "b", "c"}, []int{1, 2}) // OrderedMap { "a" => 1, "b" => 2 }
func ZipToMap[K comparable, V any](keys []K, values []V) *OrderedMap[K, V] {
	length := len(keys)
	if len(values) < length {
		length = len(values)
	}
	result := NewOrderedMap[K, V]()
	for index := 0; index < length; index++ {
		result.Set(keys[index], values[index])
	}
	return result
}
