from typing import TypeVar, Generic, List, Optional, Tuple

T = TypeVar("T")


class PriorityQueue(Generic[T]):
    """
    A priority queue implementation using a binary heap.
    Higher priority values are dequeued first.

    Features:
        enqueue(value, priority): Add element with priority
        enqueue_back(value): Add element to the back with lowest priority
        dequeue(): Remove and return highest priority element
        peek(): View highest priority element without removing
        peek_priority(): View highest priority value
        size: Get number of elements
        is_empty: Check if queue is empty
        clear(): Remove all elements
        to_list(): Get all elements as list
        to_list_with_priorities(): Get all elements with priorities

    Time Complexity:
        enqueue: O(log n)
        enqueue_back: O(log n)
        dequeue: O(log n)
        peek: O(1)
        peek_priority: O(1)

    Example:
        >>> queue = PriorityQueue()
        >>> queue.enqueue("low", 1)
        >>> queue.enqueue("high", 3)
        >>> queue.enqueue("medium", 2)
        >>> queue.dequeue()
        'high'
        >>> queue.dequeue()
        'medium'
        >>> queue.dequeue()
        'low'
    """

    def __init__(self, initial_elements: Optional[List[Tuple[T, float]]] = None):
        """
        Creates a new PriorityQueue instance.

        Args:
            initial_elements: Optional list of tuples (value, priority)
        """
        self._heap: List[Tuple[T, float]] = []
        self._min_priority = 0.0

        if initial_elements:
            self._heap = list(initial_elements)
            self._update_min_priority()
            self._build_heap()

    @property
    def size(self) -> int:
        """Returns the number of elements in the queue."""
        return len(self._heap)

    @property
    def is_empty(self) -> bool:
        """Checks if the queue is empty."""
        return len(self._heap) == 0

    def enqueue(self, value: T, priority: float) -> None:
        """
        Adds an element to the queue with a specified priority.
        Higher priority values are dequeued first.

        Args:
            value: The value to add
            priority: The priority value (higher values have higher priority)
        """
        self._heap.append((value, priority))
        self._update_min_priority_on_add(priority)
        self._heapify_up(len(self._heap) - 1)

    def enqueue_back(self, value: T) -> None:
        """
        Adds an element to the end of the queue with lowest priority.

        Args:
            value: The value to add
        """
        new_priority = self._min_priority - 1
        self._heap.append((value, new_priority))
        self._min_priority = new_priority
        self._heapify_up(len(self._heap) - 1)

    def dequeue(self) -> Optional[T]:
        """
        Removes and returns the element with the highest priority.

        Returns:
            The element with highest priority, or None if queue is empty
        """
        if len(self._heap) == 0:
            return None

        if len(self._heap) == 1:
            return self._heap.pop()[0]

        result = self._heap[0][0]
        self._heap[0] = self._heap.pop()
        self._heapify_down(0)
        return result

    def peek(self) -> Optional[T]:
        """
        Returns the element with the highest priority without removing it.

        Returns:
            The element with highest priority, or None if queue is empty
        """
        if len(self._heap) == 0:
            return None
        return self._heap[0][0]

    def peek_priority(self) -> Optional[float]:
        """
        Returns the priority of the element with the highest priority.

        Returns:
            The highest priority value, or None if queue is empty
        """
        if len(self._heap) == 0:
            return None
        return self._heap[0][1]

    def clear(self) -> None:
        """Removes all elements from the queue."""
        self._heap = []
        self._min_priority = 0.0

    def to_list(self) -> List[T]:
        """
        Returns a list of all elements in the queue.
        The order is not guaranteed to be sorted by priority.

        Returns:
            List of all elements in the queue
        """
        return [item[0] for item in self._heap]

    def to_list_with_priorities(self) -> List[Tuple[T, float]]:
        """
        Returns a list of all elements with their priorities.
        The order is not guaranteed to be sorted by priority.

        Returns:
            List of all elements with their priorities
        """
        return list(self._heap)

    def _update_min_priority_on_add(self, priority: float) -> None:
        if len(self._heap) == 1 or priority < self._min_priority:
            self._min_priority = priority

    def _update_min_priority(self) -> None:
        if len(self._heap) == 0:
            self._min_priority = 0.0
            return
        self._min_priority = min(item[1] for item in self._heap)

    def _build_heap(self) -> None:
        for i in range(len(self._heap) // 2 - 1, -1, -1):
            self._heapify_down(i)

    def _heapify_up(self, index: int) -> None:
        current_index = index
        while current_index > 0:
            parent_index = (current_index - 1) // 2
            if self._heap[current_index][1] <= self._heap[parent_index][1]:
                break
            self._swap(current_index, parent_index)
            current_index = parent_index

    def _heapify_down(self, index: int) -> None:
        current_index = index
        while True:
            left_child = 2 * current_index + 1
            right_child = 2 * current_index + 2
            largest = current_index

            if (
                left_child < len(self._heap)
                and self._heap[left_child][1] > self._heap[largest][1]
            ):
                largest = left_child

            if (
                right_child < len(self._heap)
                and self._heap[right_child][1] > self._heap[largest][1]
            ):
                largest = right_child

            if largest == current_index:
                break

            self._swap(current_index, largest)
            current_index = largest

    def _swap(self, i: int, j: int) -> None:
        self._heap[i], self._heap[j] = self._heap[j], self._heap[i]
