import math
import unittest

from src.math import lerp


class TestLerp(unittest.TestCase):
    def test_endpoints(self):
        self.assertEqual(lerp(10, 20, 0), 10)
        self.assertEqual(lerp(10, 20, 1), 20)

    def test_interpolation(self):
        self.assertEqual(lerp(0, 10, 0.5), 5)
        self.assertEqual(lerp(10, 20, 0.25), 12.5)
        self.assertEqual(lerp(-10, 10, 0.75), 5)

    def test_descending_range(self):
        self.assertEqual(lerp(20, 10, 0.25), 17.5)

    def test_extrapolation(self):
        self.assertEqual(lerp(0, 10, 2), 20)
        self.assertEqual(lerp(0, 10, -0.5), -5)

    def test_empty_range(self):
        self.assertEqual(lerp(5, 5, 0.3), 5)

    def test_nan(self):
        self.assertTrue(math.isnan(lerp(0, 10, float("nan"))))


if __name__ == "__main__":
    unittest.main()
