import math
import unittest

from src.array.ultra_number_sort import ultra_number_sort


class TestUltraNumberSort(unittest.TestCase):
    """Tests for ultra_number_sort function."""

    def test_empty_array(self):
        """Test with empty array."""
        self.assertEqual(ultra_number_sort([]), [])

    def test_single_element(self):
        """Test with single element array."""
        self.assertEqual(ultra_number_sort([42]), [42])

    def test_two_elements_ascending(self):
        """Test with two elements in ascending order."""
        self.assertEqual(ultra_number_sort([1, 2]), [1, 2])
        self.assertEqual(ultra_number_sort([2, 1]), [1, 2])

    def test_two_elements_descending(self):
        """Test with two elements in descending order."""
        self.assertEqual(ultra_number_sort([1, 2], ascending=False), [2, 1])
        self.assertEqual(ultra_number_sort([2, 1], ascending=False), [2, 1])

    def test_three_elements_ascending(self):
        """Test with three elements ascending - covers _inline_sort_3."""

        self.assertEqual(ultra_number_sort([1, 2, 3]), [1, 2, 3])

        self.assertEqual(ultra_number_sort([3, 2, 1]), [1, 2, 3])

        self.assertEqual(ultra_number_sort([2, 3, 1]), [1, 2, 3])
        self.assertEqual(ultra_number_sort([3, 1, 2]), [1, 2, 3])
        self.assertEqual(ultra_number_sort([2, 1, 3]), [1, 2, 3])
        self.assertEqual(ultra_number_sort([1, 3, 2]), [1, 2, 3])

    def test_three_elements_descending(self):
        """Test with three elements descending - covers _inline_sort_3."""

        self.assertEqual(ultra_number_sort([1, 2, 3], ascending=False), [3, 2, 1])

        self.assertEqual(ultra_number_sort([3, 2, 1], ascending=False), [3, 2, 1])

        self.assertEqual(ultra_number_sort([2, 3, 1], ascending=False), [3, 2, 1])
        self.assertEqual(ultra_number_sort([3, 1, 2], ascending=False), [3, 2, 1])
        self.assertEqual(ultra_number_sort([2, 1, 3], ascending=False), [3, 2, 1])
        self.assertEqual(ultra_number_sort([1, 3, 2], ascending=False), [3, 2, 1])

    def test_nan_handling_ascending(self):
        """Test that NaN values are moved to the end - covers _handle_nan_sort."""
        result = ultra_number_sort([3, float("nan"), 1, float("nan"), 2])

        self.assertEqual(result[:3], [1, 2, 3])
        self.assertTrue(math.isnan(result[3]))
        self.assertTrue(math.isnan(result[4]))

    def test_nan_handling_descending(self):
        """Test NaN values in descending sort."""
        result = ultra_number_sort([3, float("nan"), 1, 2], ascending=False)
        self.assertEqual(result[:3], [3, 2, 1])
        self.assertTrue(math.isnan(result[3]))

    def test_counting_sort_small_range_ascending(self):
        """Test counting sort for small integer ranges ascending."""

        arr = [5, 3, 8, 4, 2, 7, 1, 6, 3, 5]
        result = ultra_number_sort(arr)
        self.assertEqual(result, [1, 2, 3, 3, 4, 5, 5, 6, 7, 8])

    def test_counting_sort_small_range_descending(self):
        """Test counting sort for small integer ranges descending."""
        arr = [5, 3, 8, 4, 2, 7, 1, 6, 3, 5]
        result = ultra_number_sort(arr, ascending=False)
        self.assertEqual(result, [8, 7, 6, 5, 5, 4, 3, 3, 2, 1])

    def test_radix_sort_large_positive_integers(self):
        """Test radix sort for large positive integers - covers _radix_sort_positive and _radix_sort."""

        arr = list(range(1, 102)) + list(range(1000, 1102))
        import random

        random.seed(42)
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        expected = list(range(1, 102)) + list(range(1000, 1102))
        self.assertEqual(result, expected)

    def test_radix_sort_large_integers_descending(self):
        """Test radix sort descending."""

        arr = list(range(1, 102)) + list(range(1000, 1102))
        import random

        random.seed(42)
        random.shuffle(arr)
        result = ultra_number_sort(arr, ascending=False)
        expected = list(range(1101, 999, -1)) + list(range(101, 0, -1))
        self.assertEqual(result, expected)

    def test_radix_sort_with_negative_integers(self):
        """Test radix sort with negative integers."""
        arr = list(range(-100, 101))
        import random

        random.seed(42)
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        self.assertEqual(result, list(range(-100, 101)))

    def test_radix_sort_negative_integers_descending(self):
        """Test radix sort with negative integers descending."""
        arr = list(range(-100, 101))
        import random

        random.seed(42)
        random.shuffle(arr)
        result = ultra_number_sort(arr, ascending=False)
        self.assertEqual(result, list(range(100, -101, -1)))

    def test_radix_sort_with_zeros(self):
        """Test radix sort with zeros."""
        arr = [-50, -20, 0, 0, 0, 20, 50] * 20
        import random

        random.seed(42)
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        expected = [-50] * 20 + [-20] * 20 + [0] * 60 + [20] * 20 + [50] * 20
        self.assertEqual(result, expected)

    def test_quicksort_with_floats(self):
        """Test quicksort with floats - not integers, not counting sort."""
        arr = [3.5, 1.2, 4.8, 2.1, 5.9, 0.3, 8.7, 6.4]
        result = ultra_number_sort(arr)
        self.assertEqual(result, [0.3, 1.2, 2.1, 3.5, 4.8, 5.9, 6.4, 8.7])

    def test_quicksort_with_floats_descending(self):
        """Test quicksort with floats descending."""
        arr = [3.5, 1.2, 4.8, 2.1, 5.9, 0.3, 8.7, 6.4]
        result = ultra_number_sort(arr, ascending=False)
        self.assertEqual(result, [8.7, 6.4, 5.9, 4.8, 3.5, 2.1, 1.2, 0.3])

    def test_quicksort_large_floats_array(self):
        """Test quicksort with larger float array to test insertion sort cutoff."""
        import random

        random.seed(42)
        arr = [random.random() * 100 for _ in range(50)]
        result = ultra_number_sort(arr)

        for i in range(len(result) - 1):
            self.assertLessEqual(result[i], result[i + 1])

    def test_does_not_mutate_input(self):
        """Test that input array is not mutated."""
        original = [5, 3, 8, 1, 9, 2, 7, 4, 6]
        arr = original.copy()
        ultra_number_sort(arr)
        self.assertEqual(arr, original)

    def test_docstring_examples(self):
        """Test the docstring examples."""
        self.assertEqual(
            ultra_number_sort([3, 1, 4, 1, 5, 9, 2, 6]), [1, 1, 2, 3, 4, 5, 6, 9]
        )
        self.assertEqual(ultra_number_sort([3, 1, 4, 1, 5], False), [5, 4, 3, 1, 1])

    def test_large_integer_range_uses_quicksort(self):
        """Test that large integer ranges use quicksort instead of counting sort."""

        arr = [1, 1000000, 500000, 250000, 750000]
        result = ultra_number_sort(arr)
        self.assertEqual(result, [1, 250000, 500000, 750000, 1000000])

    def test_negative_floats(self):
        """Test with negative float values."""
        arr = [-3.5, -1.2, -4.8, -2.1]
        result = ultra_number_sort(arr)
        self.assertEqual(result, [-4.8, -3.5, -2.1, -1.2])

    def test_mixed_positive_negative_floats(self):
        """Test with mixed positive and negative floats."""
        arr = [-3.5, 1.2, -4.8, 2.1, 0.0]
        result = ultra_number_sort(arr)
        self.assertEqual(result, [-4.8, -3.5, 0.0, 1.2, 2.1])

    def test_partition_branches(self):
        """Test to cover various partition branches."""

        arr = [10, 5, 15, 3, 8, 12, 18, 1, 20, 7, 13, 2, 17, 4, 9, 16, 6, 14, 11, 19]
        result = ultra_number_sort(arr)
        self.assertEqual(result, list(range(1, 21)))

    def test_partition_descending_branches(self):
        """Test partition branches in descending order."""
        arr = [10, 5, 15, 3, 8, 12, 18, 1, 20, 7, 13, 2, 17, 4, 9, 16, 6, 14, 11, 19]
        result = ultra_number_sort(arr, ascending=False)
        self.assertEqual(result, list(range(20, 0, -1)))

    def test_insertion_sort_small_subarray(self):
        """Test insertion sort for small subarrays (< 16 elements) within quicksort."""

        arr = [15, 14, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0.5, -1, -2, -3, -4]
        result = ultra_number_sort(arr)
        self.assertEqual(
            result,
            [-4, -3, -2, -1, 0.5, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        )

    def test_duplicates(self):
        """Test with many duplicate values."""
        arr = [1, 2, 1, 2, 1, 2, 1, 2] * 20
        result = ultra_number_sort(arr)
        self.assertEqual(result, [1] * 80 + [2] * 80)

    def test_all_same_values(self):
        """Test with all same values."""
        arr = [5] * 50
        result = ultra_number_sort(arr)
        self.assertEqual(result, [5] * 50)

    def test_stack_push_order_in_quicksort(self):
        """Test to cover different stack push orders in quicksort."""

        import random

        random.seed(123)
        arr = [random.random() * 1000 for _ in range(100)]
        result = ultra_number_sort(arr)

        for i in range(len(result) - 1):
            self.assertLessEqual(result[i], result[i + 1])

    def test_quicksort_descending_large_array(self):
        """Test quicksort descending with large float array to cover partition descending branch."""
        import random

        random.seed(456)

        arr = [random.random() * 1000 for _ in range(100)]
        result = ultra_number_sort(arr, ascending=False)

        for i in range(len(result) - 1):
            self.assertGreaterEqual(result[i], result[i + 1])

    def test_quicksort_descending_specific_array(self):
        """Test quicksort descending with specific array to cover median-of-three descending."""

        arr = [
            50.5,
            25.3,
            75.1,
            12.8,
            37.4,
            62.9,
            87.2,
            6.1,
            18.6,
            43.7,
            56.2,
            68.4,
            81.5,
            93.8,
            31.2,
            3.5,
            9.2,
            21.7,
            46.3,
            71.9,
        ]
        result = ultra_number_sort(arr, ascending=False)
        expected = sorted(arr, reverse=True)
        self.assertEqual(result, expected)

    def test_radix_sort_only_positive(self):
        """Test radix sort with only positive integers (no negatives or zeros)."""
        import random

        random.seed(789)

        arr = list(range(500, 1000)) + list(range(1500, 2000))
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        expected = list(range(500, 1000)) + list(range(1500, 2000))
        self.assertEqual(result, expected)

    def test_radix_sort_only_negative(self):
        """Test radix sort with only negative integers."""
        import random

        random.seed(101)

        arr = list(range(-500, -100))
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        self.assertEqual(result, list(range(-500, -100)))

    def test_radix_sort_only_zeros(self):
        """Test with mostly zeros and some other values."""
        import random

        random.seed(202)
        arr = [0] * 50 + [100, 200, -100, -200] * 25
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        expected = [-200] * 25 + [-100] * 25 + [0] * 50 + [100] * 25 + [200] * 25
        self.assertEqual(result, expected)

    def test_partition_swap_branches(self):
        """Test to cover partition swap branches with specific patterns."""

        arr = [
            90.5,
            10.1,
            80.2,
            20.3,
            70.4,
            30.5,
            60.6,
            40.7,
            50.8,
            95.1,
            5.2,
            85.3,
            15.4,
            75.5,
            25.6,
            65.7,
            35.8,
            55.9,
            45.0,
        ]
        result = ultra_number_sort(arr)
        expected = sorted(arr)
        self.assertEqual(result, expected)

    def test_partition_descending_swap_branches(self):
        """Test to cover partition descending swap branches."""
        arr = [
            90.5,
            10.1,
            80.2,
            20.3,
            70.4,
            30.5,
            60.6,
            40.7,
            50.8,
            95.1,
            5.2,
            85.3,
            15.4,
            75.5,
            25.6,
            65.7,
            35.8,
            55.9,
            45.0,
        ]
        result = ultra_number_sort(arr, ascending=False)
        expected = sorted(arr, reverse=True)
        self.assertEqual(result, expected)

    def test_insertion_sort_descending(self):
        """Test insertion sort descending for small subarrays."""

        arr = [5.5, 3.3, 8.8, 1.1, 9.9, 2.2, 7.7, 4.4, 6.6]
        result = ultra_number_sort(arr, ascending=False)
        expected = sorted(arr, reverse=True)
        self.assertEqual(result, expected)

    def test_radix_sort_large_values(self):
        """Test radix sort with large integer values to test multiple byte passes."""
        import random

        random.seed(303)

        arr = [100000, 500000, 200000, 800000, 300000, 700000, 400000, 600000] * 20
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        expected = (
            [100000] * 20
            + [200000] * 20
            + [300000] * 20
            + [400000] * 20
            + [500000] * 20
            + [600000] * 20
            + [700000] * 20
            + [800000] * 20
        )
        self.assertEqual(result, expected)

    def test_radix_sort_no_positive_values_descending(self):
        """Test radix sort descending with only negative values (no positive array)."""
        import random

        random.seed(404)

        arr = list(range(-1000, -100)) + list(range(-2000, -1500))
        random.shuffle(arr)
        result = ultra_number_sort(arr, ascending=False)

        expected = list(range(-101, -1001, -1)) + list(range(-1501, -2001, -1))
        self.assertEqual(result, expected)

    def test_radix_sort_with_zeros_descending(self):
        """Test radix sort descending with zeros to cover lines 149-150."""
        import random

        random.seed(505)

        arr = [0] * 50 + list(range(100, 126)) + list(range(-500, -474))

        random.shuffle(arr)
        result = ultra_number_sort(arr, ascending=False)
        expected = list(range(125, 99, -1)) + [0] * 50 + list(range(-475, -501, -1))
        self.assertEqual(result, expected)

    def test_radix_sort_with_negatives_descending(self):
        """Test radix sort descending with negatives to cover lines 152-153."""
        import random

        random.seed(606)

        arr = list(range(100, 151)) + list(range(-500, -449))

        random.shuffle(arr)
        result = ultra_number_sort(arr, ascending=False)
        expected = list(range(150, 99, -1)) + list(range(-450, -501, -1))
        self.assertEqual(result, expected)

    def test_radix_sort_single_positive(self):
        """Test radix sort with single positive value (covers _radix_sort_positive early return)."""
        import random

        random.seed(707)

        arr = [-500, -400, -300, -200, -100] * 50 + [1000]
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        expected = (
            [-500] * 50 + [-400] * 50 + [-300] * 50 + [-200] * 50 + [-100] * 50 + [1000]
        )
        self.assertEqual(result, expected)

    def test_radix_sort_empty_positive(self):
        """Test radix sort with no positive values (empty positive array)."""
        import random

        random.seed(808)

        arr = list(range(-1000, -100)) + list(range(-3000, -2000))
        random.shuffle(arr)
        result = ultra_number_sort(arr)
        expected = list(range(-3000, -2000)) + list(range(-1000, -100))
        self.assertEqual(result, expected)


if __name__ == "__main__":
    unittest.main()
