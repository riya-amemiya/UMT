package datastructure_test

import (
	"math"
	"math/rand"
	"testing"

	"github.com/riya-amemiya/umt-go/src/datastructure"
)

// maxIntCompare is a comparison function for a max-heap of ints.
// Lower return value = higher priority, so higher values come first.
// Uses comparisons instead of subtraction to avoid integer overflow.
func maxIntCompare(a, b int) int {
	if a > b {
		return -1
	}
	if a < b {
		return 1
	}
	return 0
}

// minIntCompare is a comparison function for a min-heap of ints.
// Uses comparisons instead of subtraction to avoid integer overflow.
func minIntCompare(a, b int) int {
	if a < b {
		return -1
	}
	if a > b {
		return 1
	}
	return 0
}

// ---------------------------------------------------------------------------
// Constructor
// ---------------------------------------------------------------------------

func TestNewPriorityQueueEmpty(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	if pq.Size() != 0 {
		t.Errorf("expected size 0, got %d", pq.Size())
	}
	if !pq.IsEmpty() {
		t.Error("expected IsEmpty() == true")
	}
}

// ---------------------------------------------------------------------------
// Push / Peek / Size / IsEmpty
// ---------------------------------------------------------------------------

func TestPushAndPeek(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	pq.Push(1)
	pq.Push(3)
	pq.Push(2)

	if pq.Size() != 3 {
		t.Errorf("expected size 3, got %d", pq.Size())
	}
	if pq.IsEmpty() {
		t.Error("expected IsEmpty() == false")
	}

	val, ok := pq.Peek()
	if !ok {
		t.Fatal("expected Peek to succeed")
	}
	if val != 3 {
		t.Errorf("expected peek 3, got %d", val)
	}
	// Peek should not remove the item.
	if pq.Size() != 3 {
		t.Errorf("expected size 3 after Peek, got %d", pq.Size())
	}
}

func TestPeekEmptyQueue(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	_, ok := pq.Peek()
	if ok {
		t.Error("expected Peek on empty queue to return false")
	}
}

// ---------------------------------------------------------------------------
// Pop
// ---------------------------------------------------------------------------

func TestPopReturnsInPriorityOrder(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	pq.Push(1)
	pq.Push(3)
	pq.Push(2)

	val, ok := pq.Pop()
	if !ok || val != 3 {
		t.Errorf("expected 3, got %d (ok=%v)", val, ok)
	}
	val, ok = pq.Pop()
	if !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
	val, ok = pq.Pop()
	if !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}
	_, ok = pq.Pop()
	if ok {
		t.Error("expected Pop on empty queue to return false")
	}
}

func TestPopEmptyQueue(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	_, ok := pq.Pop()
	if ok {
		t.Error("expected Pop on empty queue to return false")
	}
}

func TestPopSingleElement(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	pq.Push(42)

	val, ok := pq.Pop()
	if !ok || val != 42 {
		t.Errorf("expected 42, got %d (ok=%v)", val, ok)
	}
	if !pq.IsEmpty() {
		t.Error("expected queue to be empty after popping single element")
	}
}

// ---------------------------------------------------------------------------
// Min-heap behavior
// ---------------------------------------------------------------------------

func TestMinHeap(t *testing.T) {
	pq := datastructure.NewPriorityQueue(minIntCompare)
	pq.Push(3)
	pq.Push(1)
	pq.Push(2)

	val, ok := pq.Pop()
	if !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}
	val, ok = pq.Pop()
	if !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
	val, ok = pq.Pop()
	if !ok || val != 3 {
		t.Errorf("expected 3, got %d (ok=%v)", val, ok)
	}
}

// ---------------------------------------------------------------------------
// Negative and special values
// ---------------------------------------------------------------------------

func TestNegativeValues(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	pq.Push(-5)
	pq.Push(0)
	pq.Push(5)

	val, _ := pq.Pop()
	if val != 5 {
		t.Errorf("expected 5, got %d", val)
	}
	val, _ = pq.Pop()
	if val != 0 {
		t.Errorf("expected 0, got %d", val)
	}
	val, _ = pq.Pop()
	if val != -5 {
		t.Errorf("expected -5, got %d", val)
	}
}

// ---------------------------------------------------------------------------
// Float priority (using float64 comparison)
// ---------------------------------------------------------------------------

func TestFloatPriority(t *testing.T) {
	pq := datastructure.NewPriorityQueue(func(a, b float64) int {
		if a > b {
			return -1 // higher value = higher priority
		}
		if a < b {
			return 1
		}
		return 0
	})
	pq.Push(1.1)
	pq.Push(1.9)
	pq.Push(1.5)

	val, _ := pq.Pop()
	if val != 1.9 {
		t.Errorf("expected 1.9, got %f", val)
	}
	val, _ = pq.Pop()
	if val != 1.5 {
		t.Errorf("expected 1.5, got %f", val)
	}
	val, _ = pq.Pop()
	if val != 1.1 {
		t.Errorf("expected 1.1, got %f", val)
	}
}

// ---------------------------------------------------------------------------
// Large numbers and infinity-like behavior
// ---------------------------------------------------------------------------

func TestLargeValues(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	pq.Push(math.MaxInt64)
	pq.Push(math.MinInt64)
	pq.Push(1)

	val, _ := pq.Pop()
	if val != math.MaxInt64 {
		t.Errorf("expected MaxInt64, got %d", val)
	}
	val, _ = pq.Pop()
	if val != 1 {
		t.Errorf("expected 1, got %d", val)
	}
	val, _ = pq.Pop()
	if val != math.MinInt64 {
		t.Errorf("expected MinInt64, got %d", val)
	}
}

func TestInfinityFloats(t *testing.T) {
	pq := datastructure.NewPriorityQueue(func(a, b float64) int {
		if a > b {
			return -1
		}
		if a < b {
			return 1
		}
		return 0
	})
	pq.Push(math.Inf(1))
	pq.Push(math.Inf(-1))
	pq.Push(1.0)

	val, _ := pq.Pop()
	if val != math.Inf(1) {
		t.Errorf("expected +Inf, got %f", val)
	}
	val, _ = pq.Pop()
	if val != 1.0 {
		t.Errorf("expected 1.0, got %f", val)
	}
	val, _ = pq.Pop()
	if val != math.Inf(-1) {
		t.Errorf("expected -Inf, got %f", val)
	}
}

// ---------------------------------------------------------------------------
// Struct types
// ---------------------------------------------------------------------------

func TestStructType(t *testing.T) {
	type Task struct {
		ID       string
		Priority int
	}

	pq := datastructure.NewPriorityQueue(func(a, b Task) int {
		return b.Priority - a.Priority // max-heap by Priority
	})

	pq.Push(Task{ID: "1", Priority: 1})
	pq.Push(Task{ID: "2", Priority: 3})
	pq.Push(Task{ID: "3", Priority: 2})

	val, ok := pq.Pop()
	if !ok {
		t.Fatal("expected Pop to succeed")
	}
	if val.ID != "2" || val.Priority != 3 {
		t.Errorf("expected Task{ID:2, Priority:3}, got %+v", val)
	}
}

// ---------------------------------------------------------------------------
// String type
// ---------------------------------------------------------------------------

func TestStringType(t *testing.T) {
	// Lexicographic max-heap
	pq := datastructure.NewPriorityQueue(func(a, b string) int {
		if a > b {
			return -1
		}
		if a < b {
			return 1
		}
		return 0
	})

	pq.Push("apple")
	pq.Push("cherry")
	pq.Push("banana")

	val, _ := pq.Pop()
	if val != "cherry" {
		t.Errorf("expected 'cherry', got %q", val)
	}
	val, _ = pq.Pop()
	if val != "banana" {
		t.Errorf("expected 'banana', got %q", val)
	}
	val, _ = pq.Pop()
	if val != "apple" {
		t.Errorf("expected 'apple', got %q", val)
	}
}

// ---------------------------------------------------------------------------
// Mixed operations
// ---------------------------------------------------------------------------

func TestMixedOperations(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)

	pq.Push(10)
	pq.Push(30)
	pq.Push(20)

	val, _ := pq.Pop()
	if val != 30 {
		t.Errorf("expected 30, got %d", val)
	}

	pq.Push(40)
	pq.Push(15)

	val, _ = pq.Pop()
	if val != 40 {
		t.Errorf("expected 40, got %d", val)
	}
	val, _ = pq.Pop()
	if val != 20 {
		t.Errorf("expected 20, got %d", val)
	}
	val, _ = pq.Pop()
	if val != 15 {
		t.Errorf("expected 15, got %d", val)
	}
	val, _ = pq.Pop()
	if val != 10 {
		t.Errorf("expected 10, got %d", val)
	}
	if !pq.IsEmpty() {
		t.Error("expected queue to be empty")
	}
}

// ---------------------------------------------------------------------------
// Large number of elements
// ---------------------------------------------------------------------------

func TestLargeNumberOfElements(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)

	for i := 0; i < 1000; i++ {
		pq.Push(rand.Intn(10000))
	}

	if pq.Size() != 1000 {
		t.Errorf("expected size 1000, got %d", pq.Size())
	}

	prev := math.MaxInt64
	for !pq.IsEmpty() {
		val, ok := pq.Pop()
		if !ok {
			t.Fatal("unexpected Pop failure")
		}
		if val > prev {
			t.Errorf("heap order violated: got %d after %d", val, prev)
		}
		prev = val
	}
}

// ---------------------------------------------------------------------------
// Equal priority elements
// ---------------------------------------------------------------------------

func TestEqualPriorityElements(t *testing.T) {
	pq := datastructure.NewPriorityQueue(maxIntCompare)
	pq.Push(1)
	pq.Push(1)
	pq.Push(1)

	count := 0
	for !pq.IsEmpty() {
		val, ok := pq.Pop()
		if !ok {
			t.Fatal("unexpected Pop failure")
		}
		if val != 1 {
			t.Errorf("expected 1, got %d", val)
		}
		count++
	}
	if count != 3 {
		t.Errorf("expected 3 elements, got %d", count)
	}
}

// ---------------------------------------------------------------------------
// Dynamic re-enqueue simulation
// ---------------------------------------------------------------------------

func TestDynamicReEnqueue(t *testing.T) {
	type entry struct {
		Name     string
		Priority int
	}

	pq := datastructure.NewPriorityQueue(func(a, b entry) int {
		return b.Priority - a.Priority
	})

	pq.Push(entry{"task1", 1})
	pq.Push(entry{"task2", 2})
	pq.Push(entry{"task3", 3})

	val, _ := pq.Pop()
	if val.Name != "task3" {
		t.Errorf("expected task3, got %s", val.Name)
	}

	// Re-enqueue task3 with lower priority
	pq.Push(entry{"task3", 0})

	val, _ = pq.Pop()
	if val.Name != "task2" {
		t.Errorf("expected task2, got %s", val.Name)
	}
	val, _ = pq.Pop()
	if val.Name != "task1" {
		t.Errorf("expected task1, got %s", val.Name)
	}
	val, _ = pq.Pop()
	if val.Name != "task3" {
		t.Errorf("expected task3 (re-enqueued), got %s", val.Name)
	}
}

// ===========================================================================
// LRUCache
// ===========================================================================

// ---------------------------------------------------------------------------
// Constructor
// ---------------------------------------------------------------------------

func TestLRUCacheEmpty(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
}

// ---------------------------------------------------------------------------
// Set and Get
// ---------------------------------------------------------------------------

func TestLRUCacheSetAndGet(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	if val, ok := cache.Get("a"); !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("c"); !ok || val != 3 {
		t.Errorf("expected 3, got %d (ok=%v)", val, ok)
	}
}

func TestLRUCacheGetMissing(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	if val, ok := cache.Get("missing"); ok || val != 0 {
		t.Errorf("expected (0, false), got (%d, %v)", val, ok)
	}
}

func TestLRUCacheUpdateExistingValue(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("a", 10)
	if val, ok := cache.Get("a"); !ok || val != 10 {
		t.Errorf("expected 10, got %d (ok=%v)", val, ok)
	}
	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
}

// ---------------------------------------------------------------------------
// Eviction
// ---------------------------------------------------------------------------

func TestLRUCacheEvictLeastRecentlyUsed(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)
	cache.Set("d", 4)

	if cache.Has("a") {
		t.Error("expected 'a' to be evicted")
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("c"); !ok || val != 3 {
		t.Errorf("expected 3, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("d"); !ok || val != 4 {
		t.Errorf("expected 4, got %d (ok=%v)", val, ok)
	}
	if cache.Size() != 3 {
		t.Errorf("expected size 3, got %d", cache.Size())
	}
}

func TestLRUCachePromoteAccessedEntries(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	cache.Get("a")
	cache.Set("d", 4)

	if !cache.Has("a") {
		t.Error("expected 'a' to remain (promoted)")
	}
	if cache.Has("b") {
		t.Error("expected 'b' to be evicted")
	}
	if !cache.Has("c") {
		t.Error("expected 'c' to remain")
	}
	if !cache.Has("d") {
		t.Error("expected 'd' to remain")
	}
}

func TestLRUCachePromoteUpdatedEntries(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	cache.Set("a", 10)
	cache.Set("d", 4)

	if !cache.Has("a") {
		t.Error("expected 'a' to remain (promoted)")
	}
	if val, ok := cache.Get("a"); !ok || val != 10 {
		t.Errorf("expected 10, got %d (ok=%v)", val, ok)
	}
	if cache.Has("b") {
		t.Error("expected 'b' to be evicted")
	}
}

// ---------------------------------------------------------------------------
// Has
// ---------------------------------------------------------------------------

func TestLRUCacheHas(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	if !cache.Has("a") {
		t.Error("expected Has('a') == true")
	}
	if cache.Has("b") {
		t.Error("expected Has('b') == false")
	}
}

// ---------------------------------------------------------------------------
// Delete
// ---------------------------------------------------------------------------

func TestLRUCacheDeleteExisting(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)

	if !cache.Delete("a") {
		t.Error("expected Delete('a') == true")
	}
	if cache.Has("a") {
		t.Error("expected 'a' removed")
	}
	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
}

func TestLRUCacheDeleteMissing(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	if cache.Delete("missing") {
		t.Error("expected Delete('missing') == false")
	}
}

func TestLRUCacheDeleteHeadNode(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	if !cache.Delete("c") {
		t.Error("expected Delete('c') == true")
	}
	if cache.Size() != 2 {
		t.Errorf("expected size 2, got %d", cache.Size())
	}
	if val, ok := cache.Get("a"); !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
}

func TestLRUCacheDeleteTailNode(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	if !cache.Delete("a") {
		t.Error("expected Delete('a') == true")
	}
	if cache.Size() != 2 {
		t.Errorf("expected size 2, got %d", cache.Size())
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("c"); !ok || val != 3 {
		t.Errorf("expected 3, got %d (ok=%v)", val, ok)
	}
}

func TestLRUCacheDeleteOnlyEntry(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	if !cache.Delete("a") {
		t.Error("expected Delete('a') == true")
	}
	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
}

// ---------------------------------------------------------------------------
// Clear
// ---------------------------------------------------------------------------

func TestLRUCacheClear(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](3)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	cache.Clear()

	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
	if _, ok := cache.Get("a"); ok {
		t.Error("expected 'a' gone after clear")
	}
	if _, ok := cache.Get("b"); ok {
		t.Error("expected 'b' gone after clear")
	}
	if _, ok := cache.Get("c"); ok {
		t.Error("expected 'c' gone after clear")
	}
}

// ---------------------------------------------------------------------------
// Size
// ---------------------------------------------------------------------------

func TestLRUCacheSizeTracking(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](5)
	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}

	cache.Set("a", 1)
	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}

	cache.Set("b", 2)
	if cache.Size() != 2 {
		t.Errorf("expected size 2, got %d", cache.Size())
	}

	cache.Delete("a")
	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

func TestLRUCacheCapacityOne(t *testing.T) {
	cache := datastructure.NewLRUCache[string, int](1)
	cache.Set("a", 1)
	if val, ok := cache.Get("a"); !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}

	cache.Set("b", 2)
	if _, ok := cache.Get("a"); ok {
		t.Error("expected 'a' evicted")
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
}

func TestLRUCacheNumericKeys(t *testing.T) {
	cache := datastructure.NewLRUCache[int, string](3)
	cache.Set(1, "one")
	cache.Set(2, "two")
	cache.Set(3, "three")

	if val, ok := cache.Get(1); !ok || val != "one" {
		t.Errorf("expected 'one', got %q (ok=%v)", val, ok)
	}
	if val, ok := cache.Get(2); !ok || val != "two" {
		t.Errorf("expected 'two', got %q (ok=%v)", val, ok)
	}
	if val, ok := cache.Get(3); !ok || val != "three" {
		t.Errorf("expected 'three', got %q (ok=%v)", val, ok)
	}
}

func TestLRUCacheManyOperations(t *testing.T) {
	cache := datastructure.NewLRUCache[int, int](100)
	for i := 0; i < 1000; i++ {
		cache.Set(i, i*2)
	}
	if cache.Size() != 100 {
		t.Errorf("expected size 100, got %d", cache.Size())
	}

	for i := 900; i < 1000; i++ {
		if val, ok := cache.Get(i); !ok || val != i*2 {
			t.Errorf("expected %d, got %d (ok=%v)", i*2, val, ok)
		}
	}

	if _, ok := cache.Get(0); ok {
		t.Error("expected key 0 to be evicted")
	}
}

// ===========================================================================
// TTLCache
// ===========================================================================

// advanceableTTLCache wires a TTLCache to a controllable clock so expiration
// can be exercised deterministically, mirroring the fake timers of the TS test.
func advanceableTTLCache[V any](opts datastructure.TTLCacheOptions, now *int64) *datastructure.TTLCache[string, V] {
	cache := datastructure.NewTTLCache[string, V](opts)
	cache.Now = func() int64 { return *now }
	return cache
}

// ---------------------------------------------------------------------------
// Constructor
// ---------------------------------------------------------------------------

func TestTTLCacheEmpty(t *testing.T) {
	cache := datastructure.NewTTLCache[string, int](datastructure.TTLCacheOptions{DefaultTTL: 5000})
	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
}

// ---------------------------------------------------------------------------
// Set and Get
// ---------------------------------------------------------------------------

func TestTTLCacheSetAndGet(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.Set("a", 1)
	cache.Set("b", 2)

	if val, ok := cache.Get("a"); !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
}

func TestTTLCacheGetMissing(t *testing.T) {
	cache := datastructure.NewTTLCache[string, int](datastructure.TTLCacheOptions{DefaultTTL: 5000})
	if val, ok := cache.Get("missing"); ok || val != 0 {
		t.Errorf("expected (0, false), got (%d, %v)", val, ok)
	}
}

func TestTTLCacheUpdateExistingValue(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.Set("a", 1)
	cache.Set("a", 10)
	if val, ok := cache.Get("a"); !ok || val != 10 {
		t.Errorf("expected 10, got %d (ok=%v)", val, ok)
	}
	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
}

// ---------------------------------------------------------------------------
// TTL expiration
// ---------------------------------------------------------------------------

func TestTTLCacheExpireAfterDefaultTTL(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.Set("a", 1)

	if val, ok := cache.Get("a"); !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}

	now += 5000

	if _, ok := cache.Get("a"); ok {
		t.Error("expected 'a' to be expired")
	}
}

func TestTTLCacheNotExpireBeforeTTL(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.Set("a", 1)

	now += 4999

	if val, ok := cache.Get("a"); !ok || val != 1 {
		t.Errorf("expected 1, got %d (ok=%v)", val, ok)
	}
}

func TestTTLCachePerEntryTTLOverride(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.SetWithTTL("short", 1, 1000)
	cache.SetWithTTL("long", 2, 10000)

	now += 1000

	if _, ok := cache.Get("short"); ok {
		t.Error("expected 'short' to be expired")
	}
	if val, ok := cache.Get("long"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}

	now += 9000

	if _, ok := cache.Get("long"); ok {
		t.Error("expected 'long' to be expired")
	}
}

func TestTTLCacheExpireViaHas(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.Set("a", 1)

	if !cache.Has("a") {
		t.Error("expected Has('a') == true")
	}

	now += 5000

	if cache.Has("a") {
		t.Error("expected Has('a') == false after expiration")
	}
}

func TestTTLCacheCleanupOnGet(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.Set("a", 1)

	now += 5000

	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
	cache.Get("a")
	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
}

func TestTTLCacheCleanupOnHas(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000}, &now)
	cache.Set("a", 1)

	now += 5000

	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
	cache.Has("a")
	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
}

// ---------------------------------------------------------------------------
// maxSize
// ---------------------------------------------------------------------------

func TestTTLCacheEvictOldestOnMaxSize(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000, MaxSize: 2, HasMaxSize: true}, &now)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	if cache.Size() != 2 {
		t.Errorf("expected size 2, got %d", cache.Size())
	}
	if _, ok := cache.Get("a"); ok {
		t.Error("expected 'a' to be evicted")
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("c"); !ok || val != 3 {
		t.Errorf("expected 3, got %d (ok=%v)", val, ok)
	}
}

func TestTTLCacheNoEvictOnUpdate(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000, MaxSize: 2, HasMaxSize: true}, &now)
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("a", 10)

	if cache.Size() != 2 {
		t.Errorf("expected size 2, got %d", cache.Size())
	}
	if val, ok := cache.Get("a"); !ok || val != 10 {
		t.Errorf("expected 10, got %d (ok=%v)", val, ok)
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
}

// ---------------------------------------------------------------------------
// Has
// ---------------------------------------------------------------------------

func TestTTLCacheHas(t *testing.T) {
	cache := datastructure.NewTTLCache[string, int](datastructure.TTLCacheOptions{DefaultTTL: 5000})
	cache.Set("a", 1)
	if !cache.Has("a") {
		t.Error("expected Has('a') == true")
	}
	if cache.Has("b") {
		t.Error("expected Has('b') == false")
	}
}

// ---------------------------------------------------------------------------
// Delete
// ---------------------------------------------------------------------------

func TestTTLCacheDeleteExisting(t *testing.T) {
	cache := datastructure.NewTTLCache[string, int](datastructure.TTLCacheOptions{DefaultTTL: 5000})
	cache.Set("a", 1)
	if !cache.Delete("a") {
		t.Error("expected Delete('a') == true")
	}
	if cache.Has("a") {
		t.Error("expected 'a' removed")
	}
	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
}

func TestTTLCacheDeleteMissing(t *testing.T) {
	cache := datastructure.NewTTLCache[string, int](datastructure.TTLCacheOptions{DefaultTTL: 5000})
	if cache.Delete("missing") {
		t.Error("expected Delete('missing') == false")
	}
}

// ---------------------------------------------------------------------------
// Clear
// ---------------------------------------------------------------------------

func TestTTLCacheClear(t *testing.T) {
	cache := datastructure.NewTTLCache[string, int](datastructure.TTLCacheOptions{DefaultTTL: 5000})
	cache.Set("a", 1)
	cache.Set("b", 2)
	cache.Set("c", 3)

	cache.Clear()

	if cache.Size() != 0 {
		t.Errorf("expected size 0, got %d", cache.Size())
	}
	if _, ok := cache.Get("a"); ok {
		t.Error("expected 'a' gone after clear")
	}
}

// ---------------------------------------------------------------------------
// Edge cases
// ---------------------------------------------------------------------------

func TestTTLCacheMaxSizeOne(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 5000, MaxSize: 1, HasMaxSize: true}, &now)
	cache.Set("a", 1)
	cache.Set("b", 2)

	if cache.Size() != 1 {
		t.Errorf("expected size 1, got %d", cache.Size())
	}
	if _, ok := cache.Get("a"); ok {
		t.Error("expected 'a' to be evicted")
	}
	if val, ok := cache.Get("b"); !ok || val != 2 {
		t.Errorf("expected 2, got %d (ok=%v)", val, ok)
	}
}

func TestTTLCacheTTLZeroImmediateExpiration(t *testing.T) {
	now := int64(0)
	cache := advanceableTTLCache[int](datastructure.TTLCacheOptions{DefaultTTL: 0}, &now)
	cache.Set("a", 1)

	if _, ok := cache.Get("a"); ok {
		t.Error("expected 'a' to expire immediately with TTL 0")
	}
}

func TestTTLCacheNumericKeys(t *testing.T) {
	cache := datastructure.NewTTLCache[int, string](datastructure.TTLCacheOptions{DefaultTTL: 5000})
	cache.Set(1, "one")
	cache.Set(2, "two")

	if val, ok := cache.Get(1); !ok || val != "one" {
		t.Errorf("expected 'one', got %q (ok=%v)", val, ok)
	}
	if val, ok := cache.Get(2); !ok || val != "two" {
		t.Errorf("expected 'two', got %q (ok=%v)", val, ok)
	}
}
