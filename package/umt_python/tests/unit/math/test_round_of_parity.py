import math
import unittest

from src.math import round_of


class TestRoundOfParity(unittest.TestCase):
    def test_positive_ties_round_up(self):
        self.assertEqual(round_of(2.5), 3)
        self.assertEqual(round_of(0.5), 1)
        self.assertEqual(round_of(0.125, 2), 0.13)

    def test_negative_ties_round_toward_positive_infinity(self):
        self.assertEqual(round_of(-2.5), -2)
        self.assertEqual(round_of(-1.5), -1)
        self.assertEqual(round_of(-0.5), 0)

    def test_value_just_below_half_rounds_down(self):
        self.assertEqual(round_of(0.49999999999999994), 0)

    def test_negative_precision(self):
        self.assertEqual(round_of(1234.5, -2), 1200)

    def test_non_finite_values_pass_through(self):
        self.assertTrue(math.isnan(round_of(float("nan"), 2)))
        self.assertEqual(round_of(float("inf"), 2), float("inf"))

    def test_overflowing_precision_is_nan(self):
        self.assertTrue(math.isnan(round_of(1.5, 400)))
        self.assertTrue(math.isnan(round_of(float("inf"), 400)))

    def test_underflowing_precision_is_nan(self):
        self.assertTrue(math.isnan(round_of(1.5, -400)))
        self.assertTrue(math.isnan(round_of(float("inf"), -400)))


if __name__ == "__main__":
    unittest.main()
