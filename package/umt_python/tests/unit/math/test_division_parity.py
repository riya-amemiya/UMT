import math
import unittest

from src.math import division


class TestDivisionParity(unittest.TestCase):
    def test_remainder_is_not_rescaled(self):
        self.assertEqual(division(0.5, 0.2, False), [2, 1])
        self.assertEqual(division(0.7, 0.2, False), [3, 1])

    def test_exponent_notation_dividend(self):
        self.assertEqual(division(1e-7, 2), 5e-8)

    def test_large_exponent_dividend(self):
        self.assertEqual(division(1e21, 4), 2.5e20)

    def test_integral_float_dividend(self):
        self.assertEqual(division(5.0, 2), 2.5)

    def test_negative_operands(self):
        self.assertEqual(division(-10, 3, False), [-3, 1])
        self.assertEqual(division(-1, -4), 0.25)

    def test_nan_dividend_returns_nan(self):
        self.assertTrue(math.isnan(division(float("nan"), 2)))

    def test_nan_dividend_with_remainder(self):
        result = division(float("nan"), 2, False)
        self.assertTrue(math.isnan(result[0]))
        self.assertTrue(math.isnan(result[1]))

    def test_infinite_dividend(self):
        self.assertEqual(division(float("inf"), 2), float("inf"))

    def test_digits_beyond_safe_integer_range_round_like_doubles(self):
        self.assertEqual(division(0.12345678901234566, 7, False), [0, 6])
        self.assertEqual(division(0.12345678901234566, 7), 0.017636684144620808)


if __name__ == "__main__":
    unittest.main()
