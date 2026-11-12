import math
import unittest

from src.math import n_cr


class TestNCrParity(unittest.TestCase):
    def test_overflow_with_large_selection_is_nan(self):
        self.assertTrue(math.isnan(n_cr(100000, 300)))

    def test_overflow_with_small_selection_is_infinity(self):
        self.assertEqual(n_cr(10**10, 100), float("inf"))

    def test_large_finite_value(self):
        self.assertEqual(n_cr(100, 97), 161700)

    def test_permutation_overflow_is_infinity_even_when_result_fits(self):
        self.assertEqual(n_cr(1000, 150), float("inf"))
        self.assertEqual(n_cr(1000, 850), float("inf"))

    def test_selection_above_factorial_range_is_nan_even_when_result_fits(self):
        self.assertTrue(math.isnan(n_cr(1000, 500)))

    def test_symmetric_selection(self):
        self.assertEqual(n_cr(20, 17), 1140)


if __name__ == "__main__":
    unittest.main()
