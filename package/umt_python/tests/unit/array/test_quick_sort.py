import unittest

from src.array.quick_sort import quick_sort


class TestQuickSort(unittest.TestCase):
    """Tests for quick_sort function."""

    def test_empty_array(self):
        """Test with empty array."""
        self.assertEqual(quick_sort([]), [])

    def test_single_element(self):
        """Test with single element array."""
        self.assertEqual(quick_sort([42]), [42])

    def test_two_elements(self):
        """Test with two elements."""
        self.assertEqual(quick_sort([2, 1]), [1, 2])
        self.assertEqual(quick_sort([1, 2]), [1, 2])

    def test_basic_integers(self):
        """Test basic sorting with integers."""
        self.assertEqual(quick_sort([1, 3, 2, 4, 5]), [1, 2, 3, 4, 5])

    def test_basic_strings(self):
        """Test basic sorting with strings."""
        self.assertEqual(
            quick_sort(["b", "a", "c"]),
            ["a", "b", "c"],
        )

    def test_custom_compare_function(self):
        """Test with custom comparison function for descending order."""
        result = quick_sort([1, 3, 2], lambda a, b: b - a)
        self.assertEqual(result, [3, 2, 1])

    def test_compare_function_ascending(self):
        """Test with explicit ascending comparison function."""
        result = quick_sort([4, 2, 7, 1, 3], lambda a, b: a - b)
        self.assertEqual(result, [1, 2, 3, 4, 7])

    def test_compare_function_equal_values(self):
        """Test compare function with equal values to cover return 0."""
        arr = [1, 1, 1, 1, 1]
        result = quick_sort(arr)
        self.assertEqual(result, [1, 1, 1, 1, 1])

    def test_compare_function_all_branches(self):
        """Test to cover all branches of _compare_function_default."""

        arr = [3, 1, 2, 1, 3, 2]
        result = quick_sort(arr)
        self.assertEqual(result, [1, 1, 2, 2, 3, 3])

    def test_with_start_index(self):
        """Test with custom start index."""
        arr = [5, 4, 3, 2, 1]
        result = quick_sort(arr, start_index=2)
        self.assertEqual(result, [5, 4, 1, 2, 3])

    def test_with_end_index(self):
        """Test with custom end index."""
        arr = [5, 4, 3, 2, 1]
        result = quick_sort(arr, end_index=2)
        self.assertEqual(result, [3, 4, 5, 2, 1])

    def test_with_start_and_end_indices(self):
        """Test with both start and end indices."""
        arr = [5, 4, 3, 2, 1]
        result = quick_sort(arr, start_index=1, end_index=3)
        self.assertEqual(result, [5, 2, 3, 4, 1])

    def test_invalid_start_index(self):
        """Test with invalid start index (negative)."""
        arr = [5, 4, 3, 2, 1]
        result = quick_sort(arr, start_index=-5)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_invalid_end_index(self):
        """Test with end index beyond array length."""
        arr = [5, 4, 3, 2, 1]
        result = quick_sort(arr, end_index=100)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_start_greater_than_end(self):
        """Test when valid_start >= valid_end (no sorting needed)."""
        arr = [5, 4, 3, 2, 1]
        result = quick_sort(arr, start_index=3, end_index=1)
        self.assertEqual(result, [5, 4, 3, 2, 1])

    def test_insertion_sort_threshold(self):
        """Test with different insertion sort thresholds."""
        arr = [5, 4, 3, 2, 1, 9, 8, 7, 6]
        result = quick_sort(arr, insertion_sort_threshold=5)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_large_insertion_sort_threshold(self):
        """Test with large threshold to use only insertion sort."""
        arr = [5, 4, 3, 2, 1]
        result = quick_sort(arr, insertion_sort_threshold=100)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_does_not_mutate_input(self):
        """Test that input array is not mutated."""
        original = [5, 3, 8, 1, 9]
        arr = original.copy()
        quick_sort(arr)
        self.assertEqual(arr, original)

    def test_already_sorted(self):
        """Test with already sorted array."""
        arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        result = quick_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

    def test_reverse_sorted(self):
        """Test with reverse sorted array."""
        arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        result = quick_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

    def test_with_duplicates(self):
        """Test with duplicate values."""
        arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
        result = quick_sort(arr)
        self.assertEqual(result, [1, 1, 2, 3, 3, 4, 5, 5, 6, 9])

    def test_median_of_three_ab_less_bc_less(self):
        """Test median of three: a < b, b < c -> median is b."""
        arr = [1, 5, 9, 2, 3, 4, 6, 7, 8]
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_median_of_three_ab_less_ac_less(self):
        """Test median of three: a < b, b >= c, a < c -> median is c."""
        arr = [3, 9, 5, 1, 2, 4, 6, 7, 8]
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_median_of_three_ab_less_ac_geq(self):
        """Test median of three: a < b, b >= c, a >= c -> median is a."""
        arr = [5, 9, 3, 1, 2, 4, 6, 7, 8]
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_median_of_three_ab_geq_ac_less(self):
        """Test median of three: a >= b, a < c -> median is a."""
        arr = [5, 1, 9, 2, 3, 4, 6, 7, 8]
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_median_of_three_ab_geq_bc_less(self):
        """Test median of three: a >= b, a >= c, b < c -> median is c."""
        arr = [9, 1, 5, 2, 3, 4, 6, 7, 8]
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_median_of_three_ab_geq_bc_geq(self):
        """Test median of three: a >= b, a >= c, b >= c -> median is b."""
        arr = [9, 5, 1, 2, 3, 4, 6, 7, 8]
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_partition_with_swaps(self):
        """Test partition swapping logic."""
        arr = [50, 10, 90, 20, 80, 30, 70, 40, 60]
        result = quick_sort(arr)
        self.assertEqual(result, [10, 20, 30, 40, 50, 60, 70, 80, 90])

    def test_sort_impl_left_partition_smaller(self):
        """Test sort_impl when left partition is smaller."""
        import random

        random.seed(42)
        arr = list(range(30))
        random.shuffle(arr)
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, list(range(30)))

    def test_sort_impl_right_partition_smaller(self):
        """Test sort_impl when right partition is smaller."""

        arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        result = quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])

    def test_docstring_examples(self):
        """Test docstring examples."""
        self.assertEqual(quick_sort([1, 3, 2, 4, 5]), [1, 2, 3, 4, 5])
        self.assertEqual(quick_sort([1, 3, 2], lambda a, b: b - a), [3, 2, 1])
        self.assertEqual(quick_sort(["b", "a", "c"]), ["a", "b", "c"])

    def test_negative_numbers(self):
        """Test with negative numbers."""
        arr = [-3, 1, -4, 1, -5, 9]
        result = quick_sort(arr)
        self.assertEqual(result, [-5, -4, -3, 1, 1, 9])

    def test_floats(self):
        """Test with floating point numbers."""
        arr = [3.5, 1.2, 4.8, 2.1]
        result = quick_sort(arr)
        self.assertEqual(result, [1.2, 2.1, 3.5, 4.8])

    def test_large_array(self):
        """Test with a larger array to exercise partitioning logic."""
        import random

        random.seed(123)
        arr = list(range(100))
        random.shuffle(arr)
        result = quick_sort(arr)
        self.assertEqual(result, list(range(100)))

    def test_insertion_sort_within_quicksort(self):
        """Test that insertion sort is used for small subarrays."""
        arr = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        result = quick_sort(arr, insertion_sort_threshold=8)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])


if __name__ == "__main__":
    unittest.main()
