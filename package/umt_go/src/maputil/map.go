package maputil

// OrderedMap is an insertion-ordered key/value collection that mirrors the
// observable behavior of a JavaScript Map: keys are kept in the order they are
// first inserted, re-setting an existing key updates its value without changing
// its position, and iteration follows that stable insertion order.
//
// The TypeScript Map utilities (groupByToMap, zipToMap) rely on these two
// guarantees that a plain Go map cannot provide — deterministic insertion order
// and stable iteration — so the Go ports return an *OrderedMap instead.
//
// Keys must be comparable so they can index the underlying lookup table.
//
// Example:
//
//	m := NewOrderedMap[string, int]()
//	m.Set("a", 1)
//	m.Set("b", 2)
//	m.Keys() // []string{"a", "b"}
type OrderedMap[K comparable, V any] struct {
	keys   []K
	values map[K]V
}

// NewOrderedMap creates an empty OrderedMap.
//
// Example:
//
//	m := NewOrderedMap[string, int]()
//	m.Size() // 0
func NewOrderedMap[K comparable, V any]() *OrderedMap[K, V] {
	return &OrderedMap[K, V]{
		keys:   nil,
		values: make(map[K]V),
	}
}

// Get retrieves the value stored for key. The second return value reports
// whether the key is present, matching the convention of a Go map read.
//
// Example:
//
//	m := NewOrderedMap[string, int]()
//	m.Set("a", 1)
//	m.Get("a") // 1, true
//	m.Get("b") // 0, false
func (m *OrderedMap[K, V]) Get(key K) (V, bool) {
	value, ok := m.values[key]
	return value, ok
}

// Set inserts or updates a key/value pair. A new key is appended to the end of
// the insertion order; an existing key keeps its position and only updates its
// value, mirroring JavaScript Map.set.
//
// Example:
//
//	m := NewOrderedMap[string, int]()
//	m.Set("a", 1)
//	m.Set("a", 2) // updates value, keeps position
func (m *OrderedMap[K, V]) Set(key K, value V) {
	if _, ok := m.values[key]; !ok {
		m.keys = append(m.keys, key)
	}
	m.values[key] = value
}

// Has reports whether key is present in the map.
//
// Example:
//
//	m := NewOrderedMap[string, int]()
//	m.Set("a", 1)
//	m.Has("a") // true
//	m.Has("b") // false
func (m *OrderedMap[K, V]) Has(key K) bool {
	_, ok := m.values[key]
	return ok
}

// Size returns the number of entries in the map.
//
// Example:
//
//	m := NewOrderedMap[string, int]()
//	m.Set("a", 1)
//	m.Size() // 1
func (m *OrderedMap[K, V]) Size() int {
	return len(m.keys)
}

// Keys returns the keys in insertion order. The returned slice is a copy, so
// mutating it does not affect the map.
//
// Example:
//
//	m := NewOrderedMap[string, int]()
//	m.Set("b", 2)
//	m.Set("a", 1)
//	m.Keys() // []string{"b", "a"}
func (m *OrderedMap[K, V]) Keys() []K {
	keys := make([]K, len(m.keys))
	copy(keys, m.keys)
	return keys
}
