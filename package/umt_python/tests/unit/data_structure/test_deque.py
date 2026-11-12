import unittest

from src.data_structure import Deque


class TestDeque(unittest.TestCase):
    def test_starts_empty(self):
        """Test that a new deque is empty and pops and peeks return None."""
        d = Deque()
        self.assertEqual(d.size, 0)
        self.assertTrue(d.is_empty)
        self.assertIsNone(d.pop_front())
        self.assertIsNone(d.pop_back())
        self.assertIsNone(d.peek_front())
        self.assertIsNone(d.peek_back())
        self.assertEqual(list(d), [])

    def test_push_and_pop_back_like_a_stack(self):
        """Test push_back and pop_back in LIFO order."""
        d = Deque()
        d.push_back(1)
        d.push_back(2)
        d.push_back(3)
        self.assertEqual(d.peek_back(), 3)
        self.assertEqual(d.pop_back(), 3)
        self.assertEqual(d.pop_back(), 2)
        self.assertEqual(d.pop_back(), 1)
        self.assertTrue(d.is_empty)

    def test_push_back_and_pop_front_like_a_queue(self):
        """Test push_back and pop_front in FIFO order."""
        d = Deque()
        d.push_back("a")
        d.push_back("b")
        self.assertEqual(d.peek_front(), "a")
        self.assertEqual(d.pop_front(), "a")
        self.assertEqual(d.pop_front(), "b")
        self.assertIsNone(d.pop_front())

    def test_push_front(self):
        """Test push_front prepends elements."""
        d = Deque()
        d.push_front(3)
        d.push_front(2)
        d.push_front(1)
        self.assertEqual(list(d), [1, 2, 3])
        self.assertEqual(d.peek_front(), 1)
        self.assertEqual(d.peek_back(), 3)
        self.assertEqual(d.size, 3)
        self.assertFalse(d.is_empty)

    def test_mixed_operations(self):
        """Test operations at both ends together."""
        d = Deque()
        d.push_back(2)
        d.push_front(1)
        d.push_back(3)
        d.push_front(0)
        self.assertEqual(list(d), [0, 1, 2, 3])
        self.assertEqual(d.pop_back(), 3)
        self.assertEqual(d.pop_front(), 0)
        self.assertEqual(list(d), [1, 2])

    def test_many_elements_keep_order(self):
        """Test order is kept with many elements at both ends."""
        d = Deque()
        for index in range(20):
            d.push_back(index)
        for index in range(1, 21):
            d.push_front(-index)
        self.assertEqual(d.size, 40)
        self.assertEqual(list(d), list(range(-20, 20)))

    def test_clear(self):
        """Test clear removes every element."""
        d = Deque()
        for index in range(30):
            d.push_back(index)
        d.clear()
        self.assertEqual(d.size, 0)
        self.assertTrue(d.is_empty)
        self.assertEqual(list(d), [])
        d.push_front(1)
        self.assertEqual(list(d), [1])

    def test_for_loop_iteration(self):
        """Test iteration with a for loop goes front to back."""
        d = Deque()
        d.push_back("b")
        d.push_front("a")
        values = []
        for value in d:
            values.append(value)
        self.assertEqual(values, ["a", "b"])


if __name__ == "__main__":
    unittest.main()
