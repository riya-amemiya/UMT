import math
import unittest

from src.math import n_pr


class TestNPr(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(n_pr(5, 2), 20)
        self.assertEqual(n_pr(5, 0), 1)
        self.assertEqual(n_pr(5, 1), 5)
        self.assertEqual(n_pr(5, 5), 120)

    def test_invalid_n_less_than_r(self):
        result = n_pr(2, 5)
        self.assertTrue(math.isnan(result))

    def test_negative_n(self):
        result = n_pr(-1, 2)
        self.assertTrue(math.isnan(result))

    def test_negative_r(self):
        result = n_pr(5, -1)
        self.assertTrue(math.isnan(result))

    def test_r_equals_zero(self):
        self.assertEqual(n_pr(10, 0), 1)

    def test_docstring_example(self):
        self.assertEqual(n_pr(5, 2), 20)
        self.assertEqual(n_pr(5, 0), 1)


if __name__ == "__main__":
    unittest.main()
