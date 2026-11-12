import math
import unittest

from src.math import n_cr


class TestNCr(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(n_cr(5, 2), 10)
        self.assertEqual(n_cr(5, 0), 1)
        self.assertEqual(n_cr(5, 5), 1)
        self.assertEqual(n_cr(10, 3), 120)

    def test_invalid_n_less_than_r(self):
        result = n_cr(2, 5)
        self.assertTrue(math.isnan(result))

    def test_negative_n(self):
        result = n_cr(-1, 2)
        self.assertTrue(math.isnan(result))

    def test_negative_r(self):
        result = n_cr(5, -1)
        self.assertTrue(math.isnan(result))

    def test_r_equals_zero(self):
        self.assertEqual(n_cr(10, 0), 1)

    def test_n_equals_r(self):
        self.assertEqual(n_cr(10, 10), 1)

    def test_docstring_example(self):
        self.assertEqual(n_cr(5, 2), 10)
        self.assertEqual(n_cr(5, 0), 1)

    def test_large_results_stay_finite(self):
        self.assertEqual(n_cr(1000, 500), 2.7028824094543666e299)
        self.assertEqual(n_cr(400, 170), 1.1372332041575198e117)
        self.assertEqual(n_cr(20, 10), 184756)

    def test_intermediate_overflow_stays_finite(self):
        self.assertEqual(n_cr(1029, 514), 1.429820686498903e308)

    def test_result_above_float_max_is_infinity(self):
        self.assertEqual(n_cr(1030, 515), float("inf"))
        self.assertEqual(n_cr(10**10, 5 * 10**9), float("inf"))


if __name__ == "__main__":
    unittest.main()
