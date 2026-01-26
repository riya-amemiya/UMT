package datastructure

import (
	"math"
	"math/rand"
	"testing"
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
	pq := NewPriorityQueue(maxIntCompare)
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
	pq := NewPriorityQueue(maxIntCompare)
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
	pq := NewPriorityQueue(maxIntCompare)
	_, ok := pq.Peek()
	if ok {
		t.Error("expected Peek on empty queue to return false")
	}
}

// ---------------------------------------------------------------------------
// Pop
// ---------------------------------------------------------------------------

func TestPopReturnsInPriorityOrder(t *testing.T) {
	pq := NewPriorityQueue(maxIntCompare)
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
	pq := NewPriorityQueue(maxIntCompare)
	_, ok := pq.Pop()
	if ok {
		t.Error("expected Pop on empty queue to return false")
	}
}

func TestPopSingleElement(t *testing.T) {
	pq := NewPriorityQueue(maxIntCompare)
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
	pq := NewPriorityQueue(minIntCompare)
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
	pq := NewPriorityQueue(maxIntCompare)
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
	pq := NewPriorityQueue(func(a, b float64) int {
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
	pq := NewPriorityQueue(maxIntCompare)
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
	pq := NewPriorityQueue(func(a, b float64) int {
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

	pq := NewPriorityQueue(func(a, b Task) int {
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
	pq := NewPriorityQueue(func(a, b string) int {
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
	pq := NewPriorityQueue(maxIntCompare)

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
	pq := NewPriorityQueue(maxIntCompare)

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
	pq := NewPriorityQueue(maxIntCompare)
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

	pq := NewPriorityQueue(func(a, b entry) int {
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
