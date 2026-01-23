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


if __name__ == "__main__":
    unittest.main()
