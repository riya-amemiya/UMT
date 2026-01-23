import unittest

from src.array import apply_insertion_sort_if_needed


class TestApplyInsertionSortIfNeeded(unittest.TestCase):
    def test_applies_sort_when_below_threshold(self):
        """Test that sort is applied when partition is below threshold."""
        arr = [5, 2, 4, 1, 3]
        result = apply_insertion_sort_if_needed(arr, 0, 4, lambda a, b: a - b, 10)
        self.assertTrue(result)
        self.assertEqual(arr, [1, 2, 3, 4, 5])

    def test_does_not_apply_sort_when_above_threshold(self):
        """Test that sort is not applied when partition exceeds threshold."""
        arr = [5, 2, 4, 1, 3]
        original = arr.copy()
        result = apply_insertion_sort_if_needed(arr, 0, 4, lambda a, b: a - b, 3)
        self.assertFalse(result)
        self.assertEqual(arr, original)

    def test_applies_sort_at_exact_threshold(self):
        """Test that sort is applied when partition equals threshold."""
        arr = [5, 2, 4, 1, 3]

        result = apply_insertion_sort_if_needed(arr, 0, 4, lambda a, b: a - b, 5)
        self.assertTrue(result)
        self.assertEqual(arr, [1, 2, 3, 4, 5])

    def test_partial_range(self):
        """Test with partial range."""
        arr = [9, 5, 2, 4, 1, 3, 8]
        result = apply_insertion_sort_if_needed(arr, 1, 5, lambda a, b: a - b, 10)
        self.assertTrue(result)

        self.assertEqual(arr, [9, 1, 2, 3, 4, 5, 8])

    def test_single_element_range(self):
        """Test with single element range."""
        arr = [5, 2, 4, 1, 3]
        result = apply_insertion_sort_if_needed(arr, 2, 2, lambda a, b: a - b, 1)
        self.assertTrue(result)

    def test_descending_sort(self):
        """Test with descending sort comparison."""
        arr = [1, 2, 3, 4, 5]
        result = apply_insertion_sort_if_needed(arr, 0, 4, lambda a, b: b - a, 10)
        self.assertTrue(result)
        self.assertEqual(arr, [5, 4, 3, 2, 1])

    def test_docstring_examples(self):
        """Test examples from docstring."""
        arr = [5, 2, 4, 1, 3]
        result = apply_insertion_sort_if_needed(arr, 0, 4, lambda a, b: a - b, 10)
        self.assertTrue(result)
        self.assertEqual(arr, [1, 2, 3, 4, 5])

        arr2 = [5, 2, 4, 1, 3]
        result2 = apply_insertion_sort_if_needed(arr2, 0, 4, lambda a, b: a - b, 3)
        self.assertFalse(result2)
        self.assertEqual(arr2, [5, 2, 4, 1, 3])


if __name__ == "__main__":
    unittest.main()
