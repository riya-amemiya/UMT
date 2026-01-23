import unittest

from src.array.dual_pivot_quick_sort import dual_pivot_quick_sort


class TestDualPivotQuickSort(unittest.TestCase):
    """Tests for dual_pivot_quick_sort function."""

    def test_empty_array(self):
        """Test with empty array."""
        self.assertEqual(dual_pivot_quick_sort([]), [])

    def test_single_element(self):
        """Test with single element array."""
        self.assertEqual(dual_pivot_quick_sort([42]), [42])

    def test_two_elements(self):
        """Test with two elements."""
        self.assertEqual(dual_pivot_quick_sort([2, 1]), [1, 2])
        self.assertEqual(dual_pivot_quick_sort([1, 2]), [1, 2])

    def test_basic_integers(self):
        """Test basic sorting with integers."""
        self.assertEqual(
            dual_pivot_quick_sort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3]),
            [1, 1, 2, 3, 3, 4, 5, 5, 6, 9],
        )

    def test_basic_strings(self):
        """Test basic sorting with strings."""
        self.assertEqual(
            dual_pivot_quick_sort(["banana", "apple", "orange"]),
            ["apple", "banana", "orange"],
        )

    def test_custom_compare_function(self):
        """Test with custom comparison function."""

        result = dual_pivot_quick_sort([3, 1, 4, 1, 5], lambda a, b: b - a)
        self.assertEqual(result, [5, 4, 3, 1, 1])

    def test_compare_function_equal_values(self):
        """Test compare function with equal values to cover return 0."""
        arr = [1, 1, 1, 1, 1]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 1, 1, 1, 1])

    def test_compare_function_all_branches(self):
        """Test to cover all branches of _compare_function_default."""

        arr = [3, 1, 2, 1, 3, 2]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 1, 2, 2, 3, 3])

    def test_start_and_end_indices(self):
        """Test with custom start and end indices."""
        arr = [5, 4, 3, 2, 1]
        result = dual_pivot_quick_sort(arr, start_index=1, end_index=3)
        self.assertEqual(result, [5, 2, 3, 4, 1])

    def test_invalid_start_index(self):
        """Test with invalid start index (negative)."""
        arr = [5, 4, 3, 2, 1]
        result = dual_pivot_quick_sort(arr, start_index=-5)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_invalid_end_index(self):
        """Test with end index beyond array length."""
        arr = [5, 4, 3, 2, 1]
        result = dual_pivot_quick_sort(arr, end_index=100)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_start_greater_than_end(self):
        """Test when valid_start >= valid_end (no sorting needed)."""
        arr = [5, 4, 3, 2, 1]
        result = dual_pivot_quick_sort(arr, start_index=3, end_index=1)
        self.assertEqual(result, [5, 4, 3, 2, 1])

    def test_insertion_sort_threshold(self):
        """Test with different insertion sort thresholds."""
        arr = [5, 4, 3, 2, 1, 9, 8, 7, 6]
        result = dual_pivot_quick_sort(arr, insertion_sort_threshold=5)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_large_insertion_sort_threshold(self):
        """Test with large threshold to use only insertion sort."""
        arr = [5, 4, 3, 2, 1]
        result = dual_pivot_quick_sort(arr, insertion_sort_threshold=100)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_does_not_mutate_input(self):
        """Test that input array is not mutated."""
        original = [5, 3, 8, 1, 9]
        arr = original.copy()
        dual_pivot_quick_sort(arr)
        self.assertEqual(arr, original)

    def test_already_sorted(self):
        """Test with already sorted array."""
        arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

    def test_reverse_sorted(self):
        """Test with reverse sorted array."""
        arr = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10])

    def test_with_duplicates(self):
        """Test with many duplicate values."""
        arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5, 8, 9, 7, 9]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 7, 8, 9, 9, 9])

    def test_large_array(self):
        """Test with a larger array to exercise partitioning logic."""
        import random

        random.seed(42)
        arr = list(range(100))
        random.shuffle(arr)
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, list(range(100)))

    def test_partition_with_swaps(self):
        """Test to exercise partition swapping logic."""

        arr = [50, 10, 90, 20, 80, 30, 70, 40, 60]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [10, 20, 30, 40, 50, 60, 70, 80, 90])

    def test_partition_right_while_loop(self):
        """Test to cover while loop in partition where right elements > pivot."""
        arr = [1, 100, 2, 99, 3, 98, 4, 97, 5, 96, 6, 95, 7, 94, 8, 93]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(
            result, [1, 2, 3, 4, 5, 6, 7, 8, 93, 94, 95, 96, 97, 98, 99, 100]
        )

    def test_partition_with_elements_less_than_left_pivot(self):
        """Test partition when elements are less than left pivot."""
        arr = [50, 40, 30, 20, 10, 60, 70, 80, 90, 100]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [10, 20, 30, 40, 50, 60, 70, 80, 90, 100])

    def test_median_of_three(self):
        """Test with arrays designed to exercise median-of-three selection."""

        arr = [5, 1, 9, 3, 7, 2, 8, 4, 6]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8, 9])

    def test_sort_range_recursion(self):
        """Test to exercise sort_range recursion with larger arrays."""
        import random

        random.seed(123)
        arr = list(range(50))
        random.shuffle(arr)
        result = dual_pivot_quick_sort(arr, insertion_sort_threshold=5)
        self.assertEqual(result, list(range(50)))

    def test_docstring_examples(self):
        """Test docstring examples."""
        self.assertEqual(
            dual_pivot_quick_sort([3, 1, 4, 1, 5, 9, 2, 6, 5, 3]),
            [1, 1, 2, 3, 3, 4, 5, 5, 6, 9],
        )
        self.assertEqual(
            dual_pivot_quick_sort(["banana", "apple", "orange"]),
            ["apple", "banana", "orange"],
        )

    def test_partition_pivot_swap_needed(self):
        """Test partition when pivots need swapping."""

        arr = [90, 80, 70, 60, 50, 40, 30, 20, 10]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [10, 20, 30, 40, 50, 60, 70, 80, 90])

    def test_partition_element_greater_than_right_pivot(self):
        """Test partition with elements greater than right pivot."""
        arr = [5, 95, 10, 90, 15, 85, 20, 80, 25, 75, 30, 70]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [5, 10, 15, 20, 25, 30, 70, 75, 80, 85, 90, 95])

    def test_sort_range_middle_partition_larger(self):
        """Test when middle partition has elements requiring recursion."""
        import random

        random.seed(456)
        arr = list(range(30))
        random.shuffle(arr)
        result = dual_pivot_quick_sort(arr, insertion_sort_threshold=3)
        self.assertEqual(result, list(range(30)))

    def test_pivot_indices_at_different_positions(self):
        """Test pivot selection with indices at different positions."""
        arr = [15, 5, 25, 10, 20, 1, 30, 8, 22, 3, 18, 12]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 3, 5, 8, 10, 12, 15, 18, 20, 22, 25, 30])

    def test_insertion_sort_small_subarray(self):
        """Test insertion sort for small subarrays."""
        arr = [5, 4, 3, 2, 1, 0]
        result = dual_pivot_quick_sort(arr, insertion_sort_threshold=10)
        self.assertEqual(result, [0, 1, 2, 3, 4, 5])

    def test_partition_current_less_than_left_after_swap(self):
        """Test partition where after right swap, current is less than left pivot."""
        arr = [50, 100, 1, 99, 2, 98, 3, 97, 4, 96, 5, 95]
        result = dual_pivot_quick_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 50, 95, 96, 97, 98, 99, 100])


if __name__ == "__main__":
    unittest.main()
