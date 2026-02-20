import math
import unittest

from src.math import sum_precise


class TestSumPrecise(unittest.TestCase):
    def test_sum_simple_integers(self):
        self.assertEqual(sum_precise([1, 2, 3, 4, 5]), 15)

    def test_handle_floating_point_precision(self):
        # Using abs_tol=1e-15 to match toBeCloseTo(..., 15)
        self.assertTrue(math.isclose(sum_precise([0.1, 0.2, 0.3]), 0.6, abs_tol=1e-15))

    def test_handle_large_and_small_number_cancellation(self):
        self.assertEqual(sum_precise([1e20, 1, -1e20]), 1.0)

    def test_return_zero_for_empty_array(self):
        self.assertEqual(sum_precise([]), 0)

    def test_return_single_element(self):
        self.assertEqual(sum_precise([42]), 42)

    def test_handle_all_negative_numbers(self):
        self.assertEqual(sum_precise([-1, -2, -3]), -6)

    def test_handle_mixed_positive_and_negative_numbers(self):
        self.assertEqual(sum_precise([10, -3, 5, -2]), 10)

    def test_handle_many_small_floating_point_values(self):
        arr = [0.001] * 1000
        # toBeCloseTo(1, 10)
        self.assertTrue(math.isclose(sum_precise(arr), 1.0, abs_tol=1e-10))


if __name__ == "__main__":
    unittest.main()
