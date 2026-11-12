import math
import unittest

from src.math import n_cr


class TestNCrParity(unittest.TestCase):
    def test_overflow_with_large_selection_is_infinity(self):
        self.assertEqual(n_cr(100000, 300), float("inf"))

    def test_overflow_with_small_selection_is_infinity(self):
        self.assertEqual(n_cr(10**10, 100), float("inf"))

    def test_large_finite_value(self):
        self.assertEqual(n_cr(100, 97), 161700)

    def test_finite_result_when_intermediate_factorials_overflow(self):
        self.assertEqual(n_cr(1000, 150), 1.343171898100407e182)
        self.assertEqual(n_cr(1000, 850), 1.343171898100407e182)

    def test_finite_result_above_factorial_range(self):
        self.assertEqual(n_cr(1000, 500), 2.7028824094543666e299)

    def test_symmetric_selection(self):
        self.assertEqual(n_cr(20, 17), 1140)


if __name__ == "__main__":
    unittest.main()
