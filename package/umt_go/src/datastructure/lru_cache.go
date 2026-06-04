package datastructure

import "container/list"

// LRUCache is a Least Recently Used (LRU) cache providing O(1) get/set
// operations.
//
// When the cache exceeds its capacity, the least recently used entry is
// evicted. Both Get and Set mark the touched entry as most recently used,
// mirroring the insertion-order semantics of a JavaScript Map: refreshing a
// key moves it to the end of the recency order, and eviction always removes
// the front (least recently used) entry.
//
// Keys must be comparable so they can be used as map keys.
//
// Example:
//
//	cache := NewLRUCache[string, int](3)
//	cache.Set("a", 1)
//	cache.Set("b", 2)
//	cache.Set("c", 3)
//	cache.Get("a")  // 1, true (moves "a" to most recently used)
//	cache.Set("d", 4) // evicts "b" (least recently used)
//	cache.Has("b")  // false
type LRUCache[K comparable, V any] struct {
	capacity int
	items    map[K]*list.Element
	order    *list.List
}

// lruEntry is the value stored in each list element, pairing the key with its
// cached value so that eviction can locate the key to delete from the map.
type lruEntry[K comparable, V any] struct {
	key   K
	value V
}

// NewLRUCache creates a new LRUCache that can hold up to capacity entries.
//
// Example:
//
//	cache := NewLRUCache[string, int](100)
func NewLRUCache[K comparable, V any](capacity int) *LRUCache[K, V] {
	return &LRUCache[K, V]{
		capacity: capacity,
		items:    make(map[K]*list.Element),
		order:    list.New(),
	}
}

// Size returns the current number of entries in the cache.
//
// Example:
//
//	cache := NewLRUCache[string, int](10)
//	cache.Set("a", 1)
//	cache.Size() // 1
func (c *LRUCache[K, V]) Size() int {
	return c.order.Len()
}

// Get retrieves a value by key and marks it as most recently used.
// The second return value is false if the key is not present.
//
// Example:
//
//	cache := NewLRUCache[string, int](10)
//	cache.Set("a", 1)
//	cache.Get("a") // 1, true
//	cache.Get("b") // 0, false
func (c *LRUCache[K, V]) Get(key K) (V, bool) {
	element, ok := c.items[key]
	if !ok {
		var zero V
		return zero, false
	}
	c.order.MoveToBack(element)
	return element.Value.(*lruEntry[K, V]).value, true
}

// Set inserts or updates a key-value pair, marking it as most recently used.
// If the key is new and the cache is at capacity, the least recently used
// entry is evicted first.
//
// Example:
//
//	cache := NewLRUCache[string, int](2)
//	cache.Set("a", 1)
//	cache.Set("b", 2)
//	cache.Set("c", 3) // evicts "a"
func (c *LRUCache[K, V]) Set(key K, value V) {
	if element, ok := c.items[key]; ok {
		element.Value.(*lruEntry[K, V]).value = value
		c.order.MoveToBack(element)
		return
	}
	if c.order.Len() >= c.capacity {
		c.evict()
	}
	element := c.order.PushBack(&lruEntry[K, V]{key: key, value: value})
	c.items[key] = element
}

// Has reports whether a key exists in the cache without affecting the
// recently-used order.
//
// Example:
//
//	cache := NewLRUCache[string, int](10)
//	cache.Set("a", 1)
//	cache.Has("a") // true
//	cache.Has("b") // false
func (c *LRUCache[K, V]) Has(key K) bool {
	_, ok := c.items[key]
	return ok
}

// Delete removes an entry by key, returning true if it was present.
//
// Example:
//
//	cache := NewLRUCache[string, int](10)
//	cache.Set("a", 1)
//	cache.Delete("a") // true
//	cache.Delete("b") // false
func (c *LRUCache[K, V]) Delete(key K) bool {
	element, ok := c.items[key]
	if !ok {
		return false
	}
	c.order.Remove(element)
	delete(c.items, key)
	return true
}

// Clear removes all entries from the cache.
//
// Example:
//
//	cache := NewLRUCache[string, int](10)
//	cache.Set("a", 1)
//	cache.Set("b", 2)
//	cache.Clear()
//	cache.Size() // 0
func (c *LRUCache[K, V]) Clear() {
	c.items = make(map[K]*list.Element)
	c.order.Init()
}

// evict removes the least recently used entry (the front of the order list).
// It is a no-op when the cache is empty.
func (c *LRUCache[K, V]) evict() {
	front := c.order.Front()
	if front == nil {
		return
	}
	c.order.Remove(front)
	delete(c.items, front.Value.(*lruEntry[K, V]).key)
}
