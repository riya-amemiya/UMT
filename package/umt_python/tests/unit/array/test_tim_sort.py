import unittest

from src.array.tim_sort import tim_sort


class TestTimSort(unittest.TestCase):
    """Tests for tim_sort function."""

    def test_empty_array(self):
        """Test with empty array."""
        self.assertEqual(tim_sort([]), [])

    def test_single_element(self):
        """Test with single element array."""
        self.assertEqual(tim_sort([42]), [42])

    def test_two_elements(self):
        """Test with two elements."""
        self.assertEqual(tim_sort([2, 1]), [1, 2])
        self.assertEqual(tim_sort([1, 2]), [1, 2])

    def test_basic_integers(self):
        """Test basic sorting with integers."""
        self.assertEqual(tim_sort([3, 1, 4, 1, 5]), [1, 1, 3, 4, 5])

    def test_basic_strings(self):
        """Test basic sorting with strings."""
        self.assertEqual(
            tim_sort(["b", "a", "c"]),
            ["a", "b", "c"],
        )

    def test_custom_compare_function(self):
        """Test with custom comparison function for descending order."""
        result = tim_sort([3, 1, 4, 1, 5], lambda a, b: b - a)
        self.assertEqual(result, [5, 4, 3, 1, 1])

    def test_compare_function_ascending(self):
        """Test with explicit ascending comparison function."""
        result = tim_sort([4, 2, 7, 1, 3], lambda a, b: a - b)
        self.assertEqual(result, [1, 2, 3, 4, 7])

    def test_compare_function_equal_values(self):
        """Test compare function with equal values to cover return 0."""
        arr = [1, 1, 1, 1, 1]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 1, 1, 1, 1])

    def test_compare_function_all_branches(self):
        """Test to cover all branches of _compare_function_default."""

        arr = [3, 1, 2, 1, 3, 2]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 1, 2, 2, 3, 3])

    def test_with_start_index(self):
        """Test with custom start index."""
        arr = [5, 4, 3, 2, 1]
        result = tim_sort(arr, start=2)
        self.assertEqual(result, [5, 4, 1, 2, 3])

    def test_with_end_index(self):
        """Test with custom end index."""
        arr = [5, 4, 3, 2, 1]
        result = tim_sort(arr, end=2)
        self.assertEqual(result, [3, 4, 5, 2, 1])

    def test_with_start_and_end_indices(self):
        """Test with both start and end indices."""
        arr = [5, 4, 3, 2, 1]
        result = tim_sort(arr, start=1, end=3)
        self.assertEqual(result, [5, 2, 3, 4, 1])

    def test_does_not_mutate_input(self):
        """Test that input array is not mutated."""
        original = [5, 3, 8, 1, 9]
        arr = original.copy()
        tim_sort(arr)
        self.assertEqual(arr, original)

    def test_already_sorted(self):
        """Test with already sorted array."""
        arr = [1, 2, 3, 4, 5]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_reverse_sorted(self):
        """Test with reverse sorted array."""
        arr = [5, 4, 3, 2, 1]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5])

    def test_with_duplicates(self):
        """Test with duplicate values."""
        arr = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 1, 2, 3, 3, 4, 5, 5, 6, 9])

    def test_large_array(self):
        """Test with a larger array to exercise merge logic."""
        import random

        random.seed(42)
        arr = list(range(100))
        random.shuffle(arr)
        result = tim_sort(arr)
        self.assertEqual(result, list(range(100)))

    def test_merge_left_exhausted_first(self):
        """Test merge where left array is exhausted first."""
        arr = [1, 2, 3, 4, 5, 6, 7, 8]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8])

    def test_merge_right_exhausted_first(self):
        """Test merge where right array is exhausted first."""
        arr = [8, 7, 6, 5, 4, 3, 2, 1]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8])

    def test_merge_interleaved(self):
        """Test merge with interleaved elements."""
        arr = [1, 3, 5, 7, 2, 4, 6, 8]
        result = tim_sort(arr)
        self.assertEqual(result, [1, 2, 3, 4, 5, 6, 7, 8])

    def test_docstring_examples(self):
        """Test docstring examples."""
        self.assertEqual(tim_sort([3, 1, 4, 1, 5]), [1, 1, 3, 4, 5])
        self.assertEqual(tim_sort(["b", "a", "c"]), ["a", "b", "c"])

    def test_negative_numbers(self):
        """Test with negative numbers."""
        arr = [-3, 1, -4, 1, -5, 9]
        result = tim_sort(arr)
        self.assertEqual(result, [-5, -4, -3, 1, 1, 9])

    def test_floats(self):
        """Test with floating point numbers."""
        arr = [3.5, 1.2, 4.8, 2.1]
        result = tim_sort(arr)
        self.assertEqual(result, [1.2, 2.1, 3.5, 4.8])

    def test_min_run_length_calculation(self):
        """Test with arrays that exercise _get_min_run_length."""

        import random

        random.seed(123)
        arr = list(range(64))
        random.shuffle(arr)
        result = tim_sort(arr)
        self.assertEqual(result, list(range(64)))

    def test_min_run_length_with_large_n(self):
        """Test min run length with larger arrays."""
        import random

        random.seed(456)

        arr = list(range(128))
        random.shuffle(arr)
        result = tim_sort(arr)
        self.assertEqual(result, list(range(128)))

    def test_multiple_merge_passes(self):
        """Test with array size requiring multiple merge passes."""
        import random

        random.seed(789)
        arr = list(range(200))
        random.shuffle(arr)
        result = tim_sort(arr)
        self.assertEqual(result, list(range(200)))

    def test_insertion_sort_range(self):
        """Test insertion sort range within tim_sort."""

        arr = [5, 4, 3, 2, 1, 0]
        result = tim_sort(arr)
        self.assertEqual(result, [0, 1, 2, 3, 4, 5])

    def test_run_end_boundary(self):
        """Test when run_end equals end."""

        arr = list(range(32, 0, -1))
        result = tim_sort(arr)
        self.assertEqual(result, list(range(1, 33)))

    def test_mid_less_than_right(self):
        """Test merge when mid < right."""
        import random

        random.seed(101)
        arr = list(range(50))
        random.shuffle(arr)
        result = tim_sort(arr)
        self.assertEqual(result, list(range(50)))

    def test_min_run_length_odd_value(self):
        """Test with array size that gives odd r value in _get_min_run_length."""
        import random

        random.seed(202)

        arr = list(range(65))
        random.shuffle(arr)
        result = tim_sort(arr)
        self.assertEqual(result, list(range(65)))

    def test_merge_left_has_remaining_elements(self):
        """Test merge where left array has remaining elements after right is exhausted.

        This covers lines 59-61 in _merge function.
        """

        arr = list(range(50, 82)) + list(range(1, 33))

        result = tim_sort(arr)
        self.assertEqual(result, list(range(1, 33)) + list(range(50, 82)))

    def test_merge_right_remaining_after_left(self):
        """Test merge where right array has remaining elements."""

        arr = list(range(1, 33)) + list(range(50, 82))
        result = tim_sort(arr)
        self.assertEqual(result, list(range(1, 33)) + list(range(50, 82)))

    def test_merge_interleaved_large(self):
        """Test merge with interleaved large arrays."""

        arr = []
        for i in range(32):
            arr.append(i * 2)
        for i in range(32):
            arr.append(i * 2 + 1)
        result = tim_sort(arr)
        self.assertEqual(result, list(range(64)))


if __name__ == "__main__":
    unittest.main()
