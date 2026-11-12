import math
import unittest

from src.math.js_round import js_round


class TestJsRound(unittest.TestCase):
    def test_ties_round_toward_positive_infinity(self):
        self.assertEqual(js_round(2.5), 3)
        self.assertEqual(js_round(-2.5), -2)
        self.assertEqual(js_round(0.5), 1)
        self.assertEqual(js_round(-0.5), 0)

    def test_rounds_to_nearest(self):
        self.assertEqual(js_round(1.4), 1)
        self.assertEqual(js_round(1.6), 2)
        self.assertEqual(js_round(-1.4), -1)
        self.assertEqual(js_round(-1.6), -2)

    def test_largest_double_below_half(self):
        self.assertEqual(js_round(0.49999999999999994), 0)

    def test_large_integral_float(self):
        self.assertEqual(js_round(4503599627370497.0), 4503599627370497)

    def test_int_unchanged(self):
        self.assertEqual(js_round(7), 7)
        self.assertIsInstance(js_round(7), int)

    def test_float_returns_int(self):
        self.assertIsInstance(js_round(2.5), int)

    def test_non_finite_unchanged(self):
        self.assertTrue(math.isnan(js_round(float("nan"))))
        self.assertEqual(js_round(float("inf")), float("inf"))
        self.assertEqual(js_round(float("-inf")), float("-inf"))


if __name__ == "__main__":
    unittest.main()
