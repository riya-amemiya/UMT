import unittest

from src.array import insertion_sort_range


class TestInsertionSortRange(unittest.TestCase):
    def test_full_array_sort(self):
        """Test sorting entire array."""
        arr = [5, 2, 4, 1, 3]
        insertion_sort_range(arr, lambda a, b: a - b, 0, 4)
        self.assertEqual(arr, [1, 2, 3, 4, 5])

    def test_partial_range_sort(self):
        """Test sorting a partial range."""
        arr = [3, 1, 4, 1, 5, 9, 2, 6]
        insertion_sort_range(arr, lambda a, b: a - b, 2, 5)
        self.assertEqual(arr, [3, 1, 1, 4, 5, 9, 2, 6])

    def test_already_sorted(self):
        """Test sorting already sorted array."""
        arr = [1, 2, 3, 4, 5]
        insertion_sort_range(arr, lambda a, b: a - b, 0, 4)
        self.assertEqual(arr, [1, 2, 3, 4, 5])

    def test_reverse_sorted(self):
        """Test sorting reverse sorted array."""
        arr = [5, 4, 3, 2, 1]
        insertion_sort_range(arr, lambda a, b: a - b, 0, 4)
        self.assertEqual(arr, [1, 2, 3, 4, 5])

    def test_descending_order(self):
        """Test sorting in descending order."""
        arr = [1, 2, 3, 4, 5]
        insertion_sort_range(arr, lambda a, b: b - a, 0, 4)
        self.assertEqual(arr, [5, 4, 3, 2, 1])

    def test_single_element_range(self):
        """Test sorting single element range."""
        arr = [3, 1, 4, 1, 5]
        original = arr.copy()
        insertion_sort_range(arr, lambda a, b: a - b, 2, 2)
        self.assertEqual(arr, original)

    def test_two_element_range(self):
        """Test sorting two element range."""
        arr = [5, 1, 4, 2, 3]
        insertion_sort_range(arr, lambda a, b: a - b, 1, 2)
        self.assertEqual(arr, [5, 1, 4, 2, 3])

        arr = [5, 4, 1, 2, 3]
        insertion_sort_range(arr, lambda a, b: a - b, 1, 2)
        self.assertEqual(arr, [5, 1, 4, 2, 3])

    def test_with_duplicates(self):
        """Test sorting with duplicate values."""
        arr = [3, 1, 3, 1, 3]
        insertion_sort_range(arr, lambda a, b: a - b, 0, 4)
        self.assertEqual(arr, [1, 1, 3, 3, 3])

    def test_string_sorting(self):
        """Test sorting strings."""
        arr = ["c", "a", "b", "e", "d"]
        insertion_sort_range(arr, lambda a, b: (a > b) - (a < b), 0, 4)
        self.assertEqual(arr, ["a", "b", "c", "d", "e"])

    def test_docstring_examples(self):
        """Test examples from docstring."""
        arr = [5, 2, 4, 1, 3]
        insertion_sort_range(arr, lambda a, b: a - b, 0, 4)
        self.assertEqual(arr, [1, 2, 3, 4, 5])

        arr2 = [3, 1, 4, 1, 5, 9, 2, 6]
        insertion_sort_range(arr2, lambda a, b: a - b, 2, 5)
        self.assertEqual(arr2, [3, 1, 1, 4, 5, 9, 2, 6])


if __name__ == "__main__":
    unittest.main()
