import unittest

from src.array.insertion_sort import insertion_sort


class TestInsertionSort(unittest.TestCase):
    """Tests for insertion_sort function."""

    def test_empty_array(self):
        """Test with empty array."""
        self.assertEqual(insertion_sort([]), [])

    def test_single_element(self):
        """Test with single element array."""
        self.assertEqual(insertion_sort([42]), [42])

    def test_two_elements(self):
        """Test with two elements."""
        self.assertEqual(insertion_sort([2, 1]), [1, 2])
        self.assertEqual(insertion_sort([1, 2]), [1, 2])

    def test_basic_integers(self):
        """Test basic sorting with integers."""
        self.assertEqual(insertion_sort([4, 2, 7, 1, 3]), [1, 2, 3, 4, 7])

    def test_basic_strings(self):
        """Test basic sorting with strings."""
        self.assertEqual(
            insertion_sort(["banana", "apple", "orange"]),
            ["apple", "banana", "orange"],
        )

    def test_custom_compare_function(self):
        """Test with custom comparison function."""

        result = insertion_sort([4, 2, 7, 1, 3], lambda a, b: b - a)
        self.assertEqual(result, [7, 4, 3, 2, 1])

    def test_compare_function_ascending(self):
        """Test with explicit ascending comparison function."""
        result = insertion_sort([4, 2, 7, 1, 3], lambda a, b: a - b)
        self.assertEqual(result, [1, 2, 3, 4, 7])

    def test_compare_function_equal_values(self):
        """Test compare function with equal values to cover return 0."""
        arr = [1, 1, 1, 1, 1]
        result = insertion_sort(arr)
        self.assertEqual(result, [1, 1, 1, 1, 1])

    def test_compare_function_all_branches(self):
        """Test to cover all branches of _compare_function_default."""

        arr = [3, 1, 2, 1, 3, 2]
        result = insertion_sort(arr)
        self.assertEqual(result, [1, 1, 2, 2, 3, 3])

    def test_with_start_index(self):
        """Test with custom start index."""
        arr = [5, 4, 3, 2, 1]
        result = insertion_sort(arr, start=2)
        self.assertEqual(result, [5, 4, 1, 2, 3])

    def test_with_end_index(self):
        """Test with custom end index."""
        arr = [5, 4, 3, 2, 1]
        result = insertion_sort(arr, end=2)
        self.assertEqual(result, [3, 4, 5, 2, 1])

    def test_with_start_and_end_indices(self):
        """Test with both start and end indices."""
        arr = [5, 4, 3, 2, 1]
        result = insertion_sort(arr, start=1, end=3)
        self.assertEqual(result, [5, 2, 3, 4, 1])

    def test_does_not_mutate_input(self):
        """Test that input array is not mutated."""
        original = [5, 3, 8, 1, 9]
        arr = original.copy()
        insertion_sort(arr)
        self.assertEqual(arr, original)

    def test_already_sorted(self):
        """Test with already sorted array."""
        arr = [1, 2, 3, 4, 5]
        result = insertion_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_reverse_sorted(self):
        """Test with reverse sorted array."""
        arr = [5, 4, 3, 2, 1]
        result = insertion_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_with_duplicates(self):
        """Test with duplicate values."""
        arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
        result = insertion_sort(arr)
        self.assertEqual(result, [1, 1, 2, 3, 3, 4, 5, 5, 6, 9])

    def test_insertion_sort_range_while_loop(self):
        """Test insertion sort range to exercise while loop."""
        arr = [5, 4, 3, 2, 1]
        result = insertion_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_docstring_examples(self):
        """Test docstring examples."""
        self.assertEqual(insertion_sort([4, 2, 7, 1, 3]), [1, 2, 3, 4, 7])
        self.assertEqual(
            insertion_sort([4, 2, 7, 1, 3], lambda a, b: a - b),
            [1, 2, 3, 4, 7],
        )

    def test_negative_numbers(self):
        """Test with negative numbers."""
        arr = [-3, 1, -4, 1, -5, 9]
        result = insertion_sort(arr)
        self.assertEqual(result, [-5, -4, -3, 1, 1, 9])

    def test_floats(self):
        """Test with floating point numbers."""
        arr = [3.5, 1.2, 4.8, 2.1]
        result = insertion_sort(arr)
        self.assertEqual(result, [1.2, 2.1, 3.5, 4.8])


if __name__ == "__main__":
    unittest.main()
