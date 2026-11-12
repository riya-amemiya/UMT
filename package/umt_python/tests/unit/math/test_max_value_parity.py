import math
import unittest

from src.math import max_value


class TestMaxValueParity(unittest.TestCase):
    def test_no_arguments_is_negative_infinity(self):
        self.assertEqual(max_value(), float("-inf"))

    def test_nan_argument_is_nan(self):
        self.assertTrue(math.isnan(max_value(1, float("nan"), 3)))

    def test_duplicates(self):
        self.assertEqual(max_value(1.5, 1.5, 0), 1.5)


if __name__ == "__main__":
    unittest.main()
