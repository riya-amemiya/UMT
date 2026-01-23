import unittest

from src.array.merge_sort import merge_sort


class TestMergeSort(unittest.TestCase):
    """Tests for merge_sort function."""

    def test_empty_array(self):
        """Test with empty array."""
        self.assertEqual(merge_sort([]), [])

    def test_single_element(self):
        """Test with single element array."""
        self.assertEqual(merge_sort([42]), [42])

    def test_two_elements(self):
        """Test with two elements."""
        self.assertEqual(merge_sort([2, 1]), [1, 2])
        self.assertEqual(merge_sort([1, 2]), [1, 2])

    def test_basic_integers(self):
        """Test basic sorting with integers."""
        self.assertEqual(merge_sort([1, 3, 2, 4, 5]), [1, 2, 3, 4, 5])

    def test_basic_strings(self):
        """Test basic sorting with strings."""
        self.assertEqual(
            merge_sort(["banana", "apple", "orange"]),
            ["apple", "banana", "orange"],
        )

    def test_custom_compare_function(self):
        """Test with custom comparison function for descending order."""
        result = merge_sort([1, 3, 2, 4, 5], lambda a, b: b - a)
        self.assertEqual(result, [5, 4, 3, 2, 1])

    def test_compare_function_ascending(self):
        """Test with explicit ascending comparison function."""
        result = merge_sort([4, 2, 7, 1, 3], lambda a, b: a - b)
        self.assertEqual(result, [1, 2, 3, 4, 7])

    def test_compare_function_equal_values(self):
        """Test compare function with equal values to cover return 0."""
        arr = [1, 1, 1, 1, 1]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 1, 1, 1, 1])

    def test_compare_function_all_branches(self):
        """Test to cover all branches of _compare_function_default."""

        arr = [3, 1, 2, 1, 3, 2]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 1, 2, 2, 3, 3])

    def test_does_not_mutate_input(self):
        """Test that input array is not mutated."""
        original = [5, 3, 8, 1, 9]
        arr = original.copy()
        merge_sort(arr)
        self.assertEqual(arr, original)

    def test_already_sorted(self):
        """Test with already sorted array."""
        arr = [1, 2, 3, 4, 5]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_reverse_sorted(self):
        """Test with reverse sorted array."""
        arr = [5, 4, 3, 2, 1]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_with_duplicates(self):
        """Test with duplicate values."""
        arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 1, 2, 3, 3, 4, 5, 5, 6, 9])

    def test_merge_left_exhausted_first(self):
        """Test merge where left array is exhausted first."""

        arr = [1, 2, 3, 4, 5]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_merge_right_exhausted_first(self):
        """Test merge where right array is exhausted first."""

        arr = [5, 4, 3, 2, 1]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_merge_interleaved(self):
        """Test merge with interleaved elements."""
        arr = [1, 3, 5, 2, 4, 6]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6])

    def test_docstring_examples(self):
        """Test docstring examples."""
        self.assertEqual(merge_sort([1, 3, 2, 4, 5]), [1, 2, 3, 4, 5])
        self.assertEqual(
            merge_sort([1, 3, 2, 4, 5], lambda a, b: b - a), [5, 4, 3, 2, 1]
        )

    def test_negative_numbers(self):
        """Test with negative numbers."""
        arr = [-3, 1, -4, 1, -5, 9]
        result = merge_sort(arr)
        self.assertEqual(result, [-5, -4, -3, 1, 1, 9])

    def test_floats(self):
        """Test with floating point numbers."""
        arr = [3.5, 1.2, 4.8, 2.1]
        result = merge_sort(arr)
        self.assertEqual(result, [1.2, 2.1, 3.5, 4.8])

    def test_large_array(self):
        """Test with a larger array to exercise recursion."""
        import random

        random.seed(42)
        arr = list(range(100))
        random.shuffle(arr)
        result = merge_sort(arr)
        self.assertEqual(result, list(range(100)))

    def test_odd_length_array(self):
        """Test with odd length array."""
        arr = [5, 2, 8, 1, 9, 3, 7]
        result = merge_sort(arr)
        self.assertEqual(result, [1, 2, 3, 5, 7, 8, 9])


if __name__ == "__main__":
    unittest.main()
