import math
import unittest

from src.math import deviation_value


class TestDeviationValueParity(unittest.TestCase):
    def test_zero_deviation_with_equal_value_is_nan(self):
        self.assertTrue(math.isnan(deviation_value(1, 1, 0)))

    def test_zero_deviation_with_nan_value_is_nan(self):
        self.assertTrue(math.isnan(deviation_value(float("nan"), 1, 0)))

    def test_zero_deviation_above_average_is_positive_infinity(self):
        self.assertEqual(deviation_value(2, 1, 0), float("inf"))

    def test_zero_deviation_below_average_is_negative_infinity(self):
        self.assertEqual(deviation_value(0, 1, 0), float("-inf"))

    def test_negative_zero_deviation_flips_sign(self):
        self.assertEqual(deviation_value(2, 1, -0.0), float("-inf"))

    def test_regular_value(self):
        self.assertEqual(deviation_value(10, 5, 2), 75)


if __name__ == "__main__":
    unittest.main()
