package function

import "container/list"

// MemoizeCache is the cache exposed by a MemoizedFunc. It preserves insertion
// order so that the oldest entry can be evicted first when a maximum size is
// configured, mirroring the JavaScript Map used by the TypeScript memoize.
//
// Keys must be comparable so they can be used as map keys.
type MemoizeCache[K comparable, R any] struct {
	items map[K]*list.Element
	order *list.List
}

// memoizeEntry pairs a key with its cached value inside each list element so
// that eviction can locate the key to delete from the map.
type memoizeEntry[K comparable, R any] struct {
	key   K
	value R
}

func newMemoizeCache[K comparable, R any]() *MemoizeCache[K, R] {
	return &MemoizeCache[K, R]{
		items: make(map[K]*list.Element),
		order: list.New(),
	}
}

// Size returns the current number of cached entries.
//
// Example:
//
//	memoized := Memoize(func(args ...any) int { return args[0].(int) * 2 })
//	memoized.Call(5)
//	memoized.Cache.Size() // 1
func (c *MemoizeCache[K, R]) Size() int {
	return c.order.Len()
}

// Has reports whether key is present in the cache.
//
// Example:
//
//	memoized := Memoize(func(args ...any) int { return args[0].(int) * 2 })
//	memoized.Call(5)
//	memoized.Cache.Has(5) // true
func (c *MemoizeCache[K, R]) Has(key K) bool {
	_, ok := c.items[key]
	return ok
}

// Get returns the cached value for key. The second return value is false when
// the key is not present.
//
// Example:
//
//	memoized := Memoize(func(args ...any) int { return args[0].(int) * 2 })
//	memoized.Call(5)
//	memoized.Cache.Get(5) // 10, true
func (c *MemoizeCache[K, R]) Get(key K) (R, bool) {
	element, ok := c.items[key]
	if !ok {
		var zero R
		return zero, false
	}
	return element.Value.(*memoizeEntry[K, R]).value, true
}

// set inserts a new key-value pair at the back of the insertion order.
func (c *MemoizeCache[K, R]) set(key K, value R) {
	element := c.order.PushBack(&memoizeEntry[K, R]{key: key, value: value})
	c.items[key] = element
}

// evictOldest removes the first-inserted entry, matching the TypeScript
// cache.delete(cache.keys().next().value) eviction.
func (c *MemoizeCache[K, R]) evictOldest() {
	front := c.order.Front()
	if front == nil {
		return
	}
	c.order.Remove(front)
	delete(c.items, front.Value.(*memoizeEntry[K, R]).key)
}

// MemoizedFunc is the memoized wrapper produced by Memoize. Call returns the
// cached result for a key when present, otherwise it computes the result,
// stores it, and returns it. The Cache field is exposed for inspection.
type MemoizedFunc[K comparable, R any] struct {
	fn       func(args ...any) R
	resolver func(args ...any) K
	maxSize  int
	hasMax   bool
	// Cache holds the memoized results keyed by the resolver output.
	Cache *MemoizeCache[K, R]
}

// MemoizeOption configures a MemoizedFunc.
type MemoizeOption[K comparable, R any] func(*MemoizedFunc[K, R])

// WithMaxSize limits the cache to maxSize entries, evicting the oldest entry
// before inserting a new one once the limit is reached. It mirrors the maxSize
// option of the TypeScript memoize.
func WithMaxSize[K comparable, R any](maxSize int) MemoizeOption[K, R] {
	return func(m *MemoizedFunc[K, R]) {
		m.maxSize = maxSize
		m.hasMax = true
	}
}

// WithResolver sets a custom cache-key resolver derived from the call
// arguments, mirroring the resolver option of the TypeScript memoize. Without a
// resolver the first argument is used as the key.
func WithResolver[K comparable, R any](resolver func(args ...any) K) MemoizeOption[K, R] {
	return func(m *MemoizedFunc[K, R]) {
		m.resolver = resolver
	}
}

// Memoize creates a memoized wrapper around fn. By default the cache is keyed by
// the first call argument; supply WithResolver to derive a custom key, and
// WithMaxSize to bound the cache with oldest-first eviction.
//
// The key type K must be comparable. When no resolver is given, the first
// argument is asserted to type K and used directly as the key, so K must match
// the runtime type of that argument.
//
// Example:
//
//	memoized := Memoize(func(args ...any) int { return args[0].(int) * 2 })
//	memoized.Call(5) // 10 (computed)
//	memoized.Call(5) // 10 (cached)
//	memoized.Cache.Size() // 1
func Memoize[K comparable, R any](fn func(args ...any) R, options ...MemoizeOption[K, R]) *MemoizedFunc[K, R] {
	m := &MemoizedFunc[K, R]{
		fn:    fn,
		Cache: newMemoizeCache[K, R](),
	}
	for _, option := range options {
		option(m)
	}
	return m
}

// Call returns the cached result for the resolved key when present, otherwise it
// computes the result with the underlying function, stores it (evicting the
// oldest entry first when the configured maximum size would be exceeded), and
// returns it.
//
// Example:
//
//	add := Memoize(
//		func(args ...any) int { return args[0].(int) + args[1].(int) },
//		WithResolver[string, int](func(args ...any) string {
//			return fmt.Sprintf("%v-%v", args[0], args[1])
//		}),
//	)
//	add.Call(1, 2) // 3 (computed)
//	add.Call(1, 2) // 3 (cached)
func (m *MemoizedFunc[K, R]) Call(args ...any) R {
	var key K
	if m.resolver != nil {
		key = m.resolver(args...)
	} else {
		key = args[0].(K)
	}

	if cached, ok := m.Cache.Get(key); ok {
		return cached
	}

	result := m.fn(args...)
	if m.hasMax && m.Cache.Size() >= m.maxSize {
		m.Cache.evictOldest()
	}
	m.Cache.set(key, result)
	return result
}
