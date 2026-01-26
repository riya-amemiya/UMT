import unittest

from src.data_structure import PriorityQueue


class TestPriorityQueue(unittest.TestCase):
    def test_empty_queue(self):
        """Test empty queue operations."""
        queue = PriorityQueue()
        self.assertEqual(queue.size, 0)
        self.assertTrue(queue.is_empty)
        self.assertIsNone(queue.peek())
        self.assertIsNone(queue.peek_priority())
        self.assertIsNone(queue.dequeue())

    def test_enqueue_single_element(self):
        """Test enqueue with a single element."""
        queue = PriorityQueue()
        queue.enqueue("first", 1)
        self.assertEqual(queue.size, 1)
        self.assertFalse(queue.is_empty)
        self.assertEqual(queue.peek(), "first")
        self.assertEqual(queue.peek_priority(), 1)

    def test_enqueue_multiple_elements(self):
        """Test enqueue with multiple elements in various orders."""
        queue = PriorityQueue()
        queue.enqueue("low", 1)
        queue.enqueue("high", 3)
        queue.enqueue("medium", 2)

        self.assertEqual(queue.peek(), "high")
        self.assertEqual(queue.peek_priority(), 3)
        self.assertEqual(queue.size, 3)

    def test_dequeue_order(self):
        """Test that dequeue returns elements in priority order."""
        queue = PriorityQueue()
        queue.enqueue("low", 1)
        queue.enqueue("high", 3)
        queue.enqueue("medium", 2)

        self.assertEqual(queue.dequeue(), "high")
        self.assertEqual(queue.dequeue(), "medium")
        self.assertEqual(queue.dequeue(), "low")
        self.assertTrue(queue.is_empty)

    def test_dequeue_single_element(self):
        """Test dequeue with a single element."""
        queue = PriorityQueue()
        queue.enqueue("only", 5)
        result = queue.dequeue()
        self.assertEqual(result, "only")
        self.assertTrue(queue.is_empty)

    def test_enqueue_back(self):
        """Test enqueue_back adds elements with lowest priority."""
        queue = PriorityQueue()
        queue.enqueue("high", 10)
        queue.enqueue_back("back1")
        queue.enqueue_back("back2")

        self.assertEqual(queue.dequeue(), "high")

        self.assertEqual(queue.dequeue(), "back1")
        self.assertEqual(queue.dequeue(), "back2")

    def test_enqueue_back_on_empty_queue(self):
        """Test enqueue_back on an empty queue."""
        queue = PriorityQueue()
        queue.enqueue_back("first")
        self.assertEqual(queue.peek(), "first")

    def test_clear(self):
        """Test clear removes all elements."""
        queue = PriorityQueue()
        queue.enqueue("a", 1)
        queue.enqueue("b", 2)
        queue.enqueue("c", 3)

        queue.clear()
        self.assertTrue(queue.is_empty)
        self.assertEqual(queue.size, 0)
        self.assertIsNone(queue.peek())

    def test_to_list(self):
        """Test to_list returns all values."""
        queue = PriorityQueue()
        queue.enqueue("a", 1)
        queue.enqueue("b", 2)
        queue.enqueue("c", 3)

        result = queue.to_list()
        self.assertEqual(len(result), 3)
        self.assertIn("a", result)
        self.assertIn("b", result)
        self.assertIn("c", result)

    def test_to_list_with_priorities(self):
        """Test to_list_with_priorities returns all tuples."""
        queue = PriorityQueue()
        queue.enqueue("a", 1)
        queue.enqueue("b", 2)

        result = queue.to_list_with_priorities()
        self.assertEqual(len(result), 2)

        values = [item[0] for item in result]
        self.assertIn("a", values)
        self.assertIn("b", values)

    def test_initial_elements(self):
        """Test initialization with initial elements."""
        initial = [("a", 1), ("b", 3), ("c", 2)]
        queue = PriorityQueue(initial_elements=initial)

        self.assertEqual(queue.size, 3)
        self.assertFalse(queue.is_empty)

        self.assertEqual(queue.peek(), "b")
        self.assertEqual(queue.peek_priority(), 3)

    def test_initial_elements_dequeue_order(self):
        """Test dequeue order with initial elements."""
        initial = [("low", 1), ("high", 5), ("medium", 3)]
        queue = PriorityQueue(initial_elements=initial)

        self.assertEqual(queue.dequeue(), "high")
        self.assertEqual(queue.dequeue(), "medium")
        self.assertEqual(queue.dequeue(), "low")

    def test_heapify_down_with_right_child_larger(self):
        """Test heapify_down when right child has higher priority."""
        queue = PriorityQueue()
        queue.enqueue("a", 5)
        queue.enqueue("b", 3)
        queue.enqueue("c", 4)

        self.assertEqual(queue.dequeue(), "a")
        self.assertEqual(queue.dequeue(), "c")
        self.assertEqual(queue.dequeue(), "b")

    def test_heapify_up_multiple_swaps(self):
        """Test heapify_up with multiple swaps needed."""
        queue = PriorityQueue()
        queue.enqueue("a", 1)
        queue.enqueue("b", 2)
        queue.enqueue("c", 3)
        queue.enqueue("d", 4)
        queue.enqueue("e", 10)

        self.assertEqual(queue.peek(), "e")

    def test_large_queue(self):
        """Test with a larger number of elements."""
        queue = PriorityQueue()
        for i in range(100):
            queue.enqueue(f"item{i}", i)

        for i in range(99, -1, -1):
            self.assertEqual(queue.dequeue(), f"item{i}")

    def test_same_priority_elements(self):
        """Test elements with same priority."""
        queue = PriorityQueue()
        queue.enqueue("a", 5)
        queue.enqueue("b", 5)
        queue.enqueue("c", 5)

        self.assertEqual(queue.size, 3)

        results: list[str] = []
        for _ in range(3):
            item = queue.dequeue()
            assert item is not None
            results.append(item)
        self.assertEqual(sorted(results), ["a", "b", "c"])

    def test_negative_priorities(self):
        """Test with negative priority values."""
        queue = PriorityQueue()
        queue.enqueue("positive", 5)
        queue.enqueue("negative", -5)
        queue.enqueue("zero", 0)

        self.assertEqual(queue.dequeue(), "positive")
        self.assertEqual(queue.dequeue(), "zero")
        self.assertEqual(queue.dequeue(), "negative")

    def test_enqueue_after_dequeue(self):
        """Test enqueue after some dequeues."""
        queue = PriorityQueue()
        queue.enqueue("a", 1)
        queue.enqueue("b", 2)

        self.assertEqual(queue.dequeue(), "b")

        queue.enqueue("c", 3)
        self.assertEqual(queue.dequeue(), "c")
        self.assertEqual(queue.dequeue(), "a")

    def test_build_heap_with_many_initial_elements(self):
        """Test _build_heap with many initial elements."""
        initial = [(f"item{i}", i) for i in range(50)]
        queue = PriorityQueue(initial_elements=initial)

        for i in range(49, -1, -1):
            self.assertEqual(queue.dequeue(), f"item{i}")

    def test_update_min_priority_on_empty_heap(self):
        """Test _update_min_priority when heap is empty."""
        queue = PriorityQueue()
        queue.enqueue("a", 5)
        queue.dequeue()

        queue.clear()
        self.assertEqual(queue._min_priority, 0.0)

    def test_heapify_down_no_children(self):
        """Test heapify_down when node has no children."""
        queue = PriorityQueue()
        queue.enqueue("only", 1)

        queue.dequeue()
        self.assertTrue(queue.is_empty)

    def test_heapify_down_left_child_only(self):
        """Test heapify_down when only left child exists."""
        queue = PriorityQueue()
        queue.enqueue("a", 3)
        queue.enqueue("b", 2)

        queue.dequeue()
        self.assertEqual(queue.dequeue(), "b")

    def test_docstring_example(self):
        """Test the docstring example."""
        queue = PriorityQueue()
        queue.enqueue("low", 1)
        queue.enqueue("high", 3)
        queue.enqueue("medium", 2)
        self.assertEqual(queue.dequeue(), "high")
        self.assertEqual(queue.dequeue(), "medium")
        self.assertEqual(queue.dequeue(), "low")

    def test_float_priorities(self):
        """Test with floating point priorities."""
        queue = PriorityQueue()
        queue.enqueue("a", 1.5)
        queue.enqueue("b", 1.7)
        queue.enqueue("c", 1.3)

        self.assertEqual(queue.dequeue(), "b")
        self.assertEqual(queue.dequeue(), "a")
        self.assertEqual(queue.dequeue(), "c")

    def test_update_min_priority_on_empty_heap_direct(self):
        """Test _update_min_priority when heap is empty (directly call private method)."""
        queue = PriorityQueue()
        queue.enqueue("a", 5)
        queue._heap = []
        queue._update_min_priority()
        self.assertEqual(queue._min_priority, 0.0)


if __name__ == "__main__":
    unittest.main()
