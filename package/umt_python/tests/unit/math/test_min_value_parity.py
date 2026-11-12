import math
import unittest

from src.math import min_value


class TestMinValueParity(unittest.TestCase):
    def test_no_arguments_is_positive_infinity(self):
        self.assertEqual(min_value(), float("inf"))

    def test_nan_argument_is_nan(self):
        self.assertTrue(math.isnan(min_value(1, float("nan"), 3)))

    def test_duplicates(self):
        self.assertEqual(min_value(1.5, 1.5, 3), 1.5)


if __name__ == "__main__":
    unittest.main()
