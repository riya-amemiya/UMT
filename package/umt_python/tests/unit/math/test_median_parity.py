import math
import unittest

from src.math import median


class TestMedianParity(unittest.TestCase):
    def test_empty_array_is_nan(self):
        self.assertTrue(math.isnan(median([])))

    def test_unsorted_odd_length(self):
        self.assertEqual(median([3, 1, 2]), 2)

    def test_unsorted_even_length(self):
        self.assertEqual(median([4, 1, 3, 2]), 2.5)

    def test_nan_values_sort_after_numbers(self):
        self.assertEqual(median([float("nan"), 1, 2, 3]), 2.5)
        self.assertTrue(math.isnan(median([1, float("nan")])))


if __name__ == "__main__":
    unittest.main()
