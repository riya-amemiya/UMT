package datastructure

import (
	"container/list"
	"time"
)

// TTLCache is a time-to-live (TTL) cache that automatically expires entries
// after a configured duration, expressed in milliseconds.
//
// Expiration is lazy: an expired entry is only removed when it is accessed via
// Get or Has. An entry expires once the current time is greater than or equal
// to its expiry timestamp, so a TTL of 0 expires immediately. Size counts every
// stored entry, including ones that have expired but not yet been accessed.
//
// When MaxSize is configured and the cache is full, inserting a new key evicts
// the oldest entry by insertion order, matching the JavaScript Map semantics of
// the reference implementation.
//
// The Now field supplies the current time in milliseconds and defaults to
// time.Now().UnixMilli(). Tests can override it to drive expiration
// deterministically.
//
// Keys must be comparable so they can be used as map keys.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.Set("a", 1)
//	cache.Get("a") // 1, true
//	// after 5000ms have elapsed
//	cache.Get("a") // 0, false (expired)
type TTLCache[K comparable, V any] struct {
	defaultTTL int64
	maxSize    int
	hasMaxSize bool
	items      map[K]*list.Element
	order      *list.List
	// Now returns the current time in milliseconds since the Unix epoch.
	Now func() int64
}

// TTLCacheOptions configures a TTLCache.
//
// DefaultTTL is the default time-to-live in milliseconds applied to entries
// inserted without an explicit override. MaxSize, when set via WithMaxSize,
// caps the number of stored entries; leaving it unset (the default zero value)
// means the cache is unbounded.
type TTLCacheOptions struct {
	DefaultTTL int64
	// MaxSize caps the number of entries. It is only honored when HasMaxSize
	// is true, mirroring the optional maxSize field of the reference options.
	MaxSize    int
	HasMaxSize bool
}

// ttlEntry pairs a stored value with its absolute expiry timestamp (in
// milliseconds) and the key, so eviction can locate the key to delete.
type ttlEntry[K comparable, V any] struct {
	key       K
	value     V
	expiresAt int64
}

// NewTTLCache creates a new TTLCache from the given options.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{
//	    DefaultTTL: 60000,
//	    MaxSize:    1000,
//	    HasMaxSize: true,
//	})
func NewTTLCache[K comparable, V any](options TTLCacheOptions) *TTLCache[K, V] {
	return &TTLCache[K, V]{
		defaultTTL: options.DefaultTTL,
		maxSize:    options.MaxSize,
		hasMaxSize: options.HasMaxSize,
		items:      make(map[K]*list.Element),
		order:      list.New(),
		Now: func() int64 {
			return time.Now().UnixMilli()
		},
	}
}

// Size returns the number of stored entries, including any that have expired
// but not yet been removed by a lazy Get/Has access.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.Set("a", 1)
//	cache.Size() // 1
func (c *TTLCache[K, V]) Size() int {
	return c.order.Len()
}

// Get retrieves a value by key. The second return value is false if the key is
// absent or has expired; an expired entry is removed as a side effect.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.Set("a", 1)
//	cache.Get("a") // 1, true
func (c *TTLCache[K, V]) Get(key K) (V, bool) {
	element, ok := c.items[key]
	if !ok {
		var zero V
		return zero, false
	}
	entry := element.Value.(*ttlEntry[K, V])
	if c.Now() >= entry.expiresAt {
		c.removeElement(key, element)
		var zero V
		return zero, false
	}
	return entry.value, true
}

// Set inserts or updates a key-value pair using the default TTL.
// If MaxSize is configured and the cache is full, the oldest entry is evicted
// before a new key is inserted; updating an existing key never evicts.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.Set("a", 1)
func (c *TTLCache[K, V]) Set(key K, value V) {
	c.SetWithTTL(key, value, c.defaultTTL)
}

// SetWithTTL inserts or updates a key-value pair with an explicit TTL override
// in milliseconds. If MaxSize is configured and the cache is full, the oldest
// entry is evicted before a new key is inserted; updating an existing key never
// evicts.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.SetWithTTL("b", 2, 10000) // custom 10s TTL
func (c *TTLCache[K, V]) SetWithTTL(key K, value V, ttl int64) {
	expiresAt := c.Now() + ttl

	if element, ok := c.items[key]; ok {
		entry := element.Value.(*ttlEntry[K, V])
		entry.value = value
		entry.expiresAt = expiresAt
		return
	}

	if c.hasMaxSize && c.order.Len() >= c.maxSize {
		front := c.order.Front()
		if front != nil {
			c.removeElement(front.Value.(*ttlEntry[K, V]).key, front)
		}
	}

	element := c.order.PushBack(&ttlEntry[K, V]{
		key:       key,
		value:     value,
		expiresAt: expiresAt,
	})
	c.items[key] = element
}

// Has reports whether a non-expired key exists, removing the entry if it has
// expired.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.Set("a", 1)
//	cache.Has("a") // true
//	cache.Has("b") // false
func (c *TTLCache[K, V]) Has(key K) bool {
	element, ok := c.items[key]
	if !ok {
		return false
	}
	entry := element.Value.(*ttlEntry[K, V])
	if c.Now() >= entry.expiresAt {
		c.removeElement(key, element)
		return false
	}
	return true
}

// Delete removes an entry by key, returning true if it was present.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.Set("a", 1)
//	cache.Delete("a") // true
//	cache.Delete("b") // false
func (c *TTLCache[K, V]) Delete(key K) bool {
	element, ok := c.items[key]
	if !ok {
		return false
	}
	c.removeElement(key, element)
	return true
}

// Clear removes all entries from the cache.
//
// Example:
//
//	cache := NewTTLCache[string, int](TTLCacheOptions{DefaultTTL: 5000})
//	cache.Set("a", 1)
//	cache.Clear()
//	cache.Size() // 0
func (c *TTLCache[K, V]) Clear() {
	c.items = make(map[K]*list.Element)
	c.order.Init()
}

// removeElement deletes the given element from both the order list and the
// lookup map.
func (c *TTLCache[K, V]) removeElement(key K, element *list.Element) {
	c.order.Remove(element)
	delete(c.items, key)
}
