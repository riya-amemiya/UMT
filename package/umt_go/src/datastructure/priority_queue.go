package datastructure

// PriorityQueue is a generic priority queue backed by a binary heap.
// The ordering is determined by a comparison function provided at construction.
//
// The compare function should return:
//   - a negative value if a has higher priority than b (a should be dequeued first)
//   - zero if a and b have equal priority
//   - a positive value if b has higher priority than a (b should be dequeued first)
//
// This follows the convention where compare(a, b) < 0 means a comes first,
// similar to sort.Slice's less function but returning an int.
type PriorityQueue[T any] struct {
	heap    []T
	compare func(a, b T) int
}

// NewPriorityQueue creates a new PriorityQueue with the given comparison function.
//
// The compare function determines element ordering:
//   - compare(a, b) < 0: a has higher priority (dequeued first)
//   - compare(a, b) == 0: equal priority
//   - compare(a, b) > 0: b has higher priority (dequeued first)
//
// Example (max-heap of ints):
//
//	pq := NewPriorityQueue(func(a, b int) int { return b - a })
//
// Example (min-heap of ints):
//
//	pq := NewPriorityQueue(func(a, b int) int { return a - b })
func NewPriorityQueue[T any](compare func(a, b T) int) *PriorityQueue[T] {
	return &PriorityQueue[T]{
		heap:    make([]T, 0),
		compare: compare,
	}
}

// Push adds an item to the priority queue. The item is placed according to
// its priority as determined by the comparison function. O(log n).
func (pq *PriorityQueue[T]) Push(item T) {
	pq.heap = append(pq.heap, item)
	pq.heapifyUp(len(pq.heap) - 1)
}

// Pop removes and returns the highest-priority item from the queue.
// The second return value is false if the queue is empty.
// O(log n).
func (pq *PriorityQueue[T]) Pop() (T, bool) {
	if len(pq.heap) == 0 {
		var zero T
		return zero, false
	}

	if len(pq.heap) == 1 {
		item := pq.heap[0]
		pq.heap = pq.heap[:0]
		return item, true
	}

	item := pq.heap[0]
	last := len(pq.heap) - 1
	pq.heap[0] = pq.heap[last]
	pq.heap = pq.heap[:last]
	pq.heapifyDown(0)
	return item, true
}

// Peek returns the highest-priority item without removing it.
// The second return value is false if the queue is empty. O(1).
func (pq *PriorityQueue[T]) Peek() (T, bool) {
	if len(pq.heap) == 0 {
		var zero T
		return zero, false
	}
	return pq.heap[0], true
}

// Size returns the number of items in the queue.
func (pq *PriorityQueue[T]) Size() int {
	return len(pq.heap)
}

// IsEmpty returns true if the queue contains no items.
func (pq *PriorityQueue[T]) IsEmpty() bool {
	return len(pq.heap) == 0
}

// heapifyUp restores the heap property by moving the element at index up
// towards the root.
func (pq *PriorityQueue[T]) heapifyUp(index int) {
	for index > 0 {
		parent := (index - 1) / 2
		if pq.compare(pq.heap[index], pq.heap[parent]) >= 0 {
			break
		}
		pq.heap[index], pq.heap[parent] = pq.heap[parent], pq.heap[index]
		index = parent
	}
}

// heapifyDown restores the heap property by moving the element at index down
// towards the leaves.
func (pq *PriorityQueue[T]) heapifyDown(index int) {
	n := len(pq.heap)
	for {
		left := 2*index + 1
		right := 2*index + 2
		smallest := index

		if left < n && pq.compare(pq.heap[left], pq.heap[smallest]) < 0 {
			smallest = left
		}
		if right < n && pq.compare(pq.heap[right], pq.heap[smallest]) < 0 {
			smallest = right
		}

		if smallest == index {
			break
		}

		pq.heap[index], pq.heap[smallest] = pq.heap[smallest], pq.heap[index]
		index = smallest
	}
}
