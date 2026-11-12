import math
import unittest

from src.math import inverse_lerp


class TestInverseLerp(unittest.TestCase):
    def test_endpoints(self):
        self.assertEqual(inverse_lerp(10, 20, 10), 0)
        self.assertEqual(inverse_lerp(10, 20, 20), 1)

    def test_inside_range(self):
        self.assertEqual(inverse_lerp(0, 10, 5), 0.5)
        self.assertEqual(inverse_lerp(10, 20, 12.5), 0.25)

    def test_descending_range(self):
        self.assertEqual(inverse_lerp(20, 10, 17.5), 0.25)

    def test_outside_range(self):
        self.assertEqual(inverse_lerp(0, 10, 20), 2)
        self.assertEqual(inverse_lerp(0, 10, -5), -0.5)

    def test_empty_range_returns_zero(self):
        self.assertEqual(inverse_lerp(5, 5, 7), 0)
        self.assertEqual(inverse_lerp(5, 5, 5), 0)

    def test_nan(self):
        self.assertTrue(math.isnan(inverse_lerp(0, 10, float("nan"))))


if __name__ == "__main__":
    unittest.main()
