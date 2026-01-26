import math
import unittest

from src.math import reduce_fraction


class TestReduceFraction(unittest.TestCase):
    def test_basic_cases(self):
        result = reduce_fraction(4, 8)
        self.assertEqual(result["x"], 1.0)
        self.assertEqual(result["y"], 2.0)
        self.assertEqual(result["gcd"], 4.0)

    def test_edge_cases(self):
        result = reduce_fraction(2, 4)
        self.assertEqual(result["x"], 1)
        self.assertEqual(result["y"], 2)
        self.assertEqual(result["gcd"], 2)

    def test_zero_denominator(self):
        result = reduce_fraction(5, 0)
        self.assertTrue(math.isnan(result["x"]))
        self.assertTrue(math.isnan(result["y"]))
        self.assertTrue(math.isnan(result["gcd"]))

    def test_zero_numerator(self):
        result = reduce_fraction(0, 5)
        self.assertEqual(result["x"], 0)
        self.assertEqual(result["y"], 1)
        self.assertEqual(result["gcd"], 5)

    def test_docstring_example(self):
        result = reduce_fraction(2, 4)
        self.assertEqual(result, {"x": 1, "y": 2, "gcd": 2})


if __name__ == "__main__":
    unittest.main()
