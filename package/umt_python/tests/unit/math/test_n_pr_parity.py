import unittest

from src.math import n_pr


class TestNPrParity(unittest.TestCase):
    def test_integer_result_is_float(self):
        result = n_pr(5, 2)
        self.assertEqual(result, 20)
        self.assertIsInstance(result, float)

    def test_integral_float_n_is_float(self):
        result = n_pr(5.0, 2)
        self.assertEqual(result, 20)
        self.assertIsInstance(result, float)

    def test_non_integral_n(self):
        self.assertEqual(n_pr(5.5, 2), 24.75)

    def test_overflow_is_infinity(self):
        self.assertEqual(n_pr(300, 200), float("inf"))

    def test_zero_of_zero(self):
        self.assertEqual(n_pr(0, 0), 1)


if __name__ == "__main__":
    unittest.main()
