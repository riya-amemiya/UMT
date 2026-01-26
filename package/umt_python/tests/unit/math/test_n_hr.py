import math
import unittest

from src.math import n_hr


class TestNHr(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(n_hr(5, 2), 15)
        self.assertEqual(n_hr(3, 3), 10)

    def test_n_equals_zero(self):
        result = n_hr(0, 2)
        self.assertTrue(math.isnan(result))

    def test_r_equals_zero(self):
        result = n_hr(5, 0)
        self.assertTrue(math.isnan(result))

    def test_negative_n(self):
        result = n_hr(-1, 2)
        self.assertTrue(math.isnan(result))

    def test_negative_r(self):
        result = n_hr(5, -1)
        self.assertTrue(math.isnan(result))

    def test_docstring_example(self):
        self.assertEqual(n_hr(5, 2), 15)


if __name__ == "__main__":
    unittest.main()
