import math
import unittest

from src.math import quotient


class TestQuotientParity(unittest.TestCase):
    def test_negative_dividend_truncates_toward_zero(self):
        self.assertEqual(quotient(-5, 2), [-2, -1])

    def test_negative_divisor_truncates_toward_zero(self):
        self.assertEqual(quotient(5, -2), [-2, 1])

    def test_decimal_dividend(self):
        self.assertEqual(quotient(5.5, 2), [2, 1.5])

    def test_zero_divisor_is_nan(self):
        result = quotient(5, 0)
        self.assertTrue(math.isnan(result[0]))
        self.assertTrue(math.isnan(result[1]))

    def test_infinite_dividend_is_nan(self):
        result = quotient(float("inf"), 2)
        self.assertTrue(math.isnan(result[0]))
        self.assertTrue(math.isnan(result[1]))

    def test_nan_dividend_with_zero_divisor_is_nan(self):
        result = quotient(float("nan"), 0)
        self.assertTrue(math.isnan(result[0]))
        self.assertTrue(math.isnan(result[1]))

    def test_infinite_divisor_keeps_dividend_as_remainder(self):
        self.assertEqual(quotient(5, float("inf")), [0, 5])


if __name__ == "__main__":
    unittest.main()
