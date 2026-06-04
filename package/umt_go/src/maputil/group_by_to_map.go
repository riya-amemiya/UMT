package maputil

// GroupByToMap groups elements of a slice into an OrderedMap based on a given
// iteratee function. Unlike a grouping that returns map[K][]T, this preserves
// the insertion order of keys (the order each group key first appears) and
// allows any comparable key type. The iteratee receives the value and its
// index, mirroring the TypeScript groupByToMap signature.
//
// Example:
//
//	import "math"
//	GroupByToMap([]float64{6.1, 4.2, 6.3}, func(v float64, _ int) float64 {
//		return math.Floor(v)
//	}) // OrderedMap { 6 => [6.1, 6.3], 4 => [4.2] }
//
//	GroupByToMap([]string{"one", "two", "three"}, func(s string, _ int) int {
//		return len(s)
//	}) // OrderedMap { 3 => ["one", "two"], 5 => ["three"] }
func GroupByToMap[T any, K comparable](
	array []T,
	iteratee func(value T, index int) K,
) *OrderedMap[K, []T] {
	result := NewOrderedMap[K, []T]()
	for index, value := range array {
		key := iteratee(value, index)
		bucket, ok := result.Get(key)
		if !ok {
			result.Set(key, []T{value})
		} else {
			result.Set(key, append(bucket, value))
		}
	}
	return result
}
