import math
import unittest

from src.math import clamp


class TestClampParity(unittest.TestCase):
    def test_nan_value(self):
        self.assertTrue(math.isnan(clamp(float("nan"), 0, 10)))

    def test_nan_minimum(self):
        self.assertTrue(math.isnan(clamp(5, float("nan"), 10)))

    def test_nan_maximum(self):
        self.assertTrue(math.isnan(clamp(5, 0, float("nan"))))

    def test_inverted_bounds(self):
        self.assertEqual(clamp(5, 10, 0), 0)


if __name__ == "__main__":
    unittest.main()
