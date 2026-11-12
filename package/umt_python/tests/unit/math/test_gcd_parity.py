import math
import unittest

from src.math import gcd


class TestGcdParity(unittest.TestCase):
    def test_decimal_inputs_scale_by_decimal_places(self):
        self.assertEqual(gcd(0.1, 0.3), 0.1)
        self.assertEqual(gcd(1.2, 1.8, 2.4), 0.6)
        self.assertEqual(gcd(1.5, 2.5), 0.5)

    def test_zero_first_argument_ignores_additional_numbers(self):
        self.assertEqual(gcd(0, 6, 4), 6)

    def test_zero_second_argument_ignores_additional_numbers(self):
        self.assertEqual(gcd(6, 0, 4), 6)

    def test_both_zero(self):
        self.assertEqual(gcd(0, 0), 0)

    def test_negative_inputs(self):
        self.assertEqual(gcd(-12, 18), 6)

    def test_integral_floats(self):
        self.assertEqual(gcd(12.0, 18.0), 6)

    def test_large_integers_stay_exact(self):
        self.assertEqual(gcd(10**20 + 2, 4), 2)

    def test_coprime(self):
        self.assertEqual(gcd(7, 13), 1)

    def test_scaled_decimals_use_floating_point_products(self):
        self.assertEqual(gcd(0.3333333333333333, 4294967295), 1e-16)

    def test_zero_with_non_finite_returns_the_non_finite_value(self):
        self.assertTrue(math.isnan(gcd(0, float("nan"))))
        self.assertTrue(math.isnan(gcd(float("nan"), 0)))
        self.assertEqual(gcd(0, float("-inf")), float("inf"))
        self.assertEqual(gcd(float("inf"), 0), float("inf"))

    def test_non_finite_with_non_zero_raises(self):
        with self.assertRaises(ValueError):
            gcd(float("nan"), 5)
        with self.assertRaises(ValueError):
            gcd(5, float("nan"))


if __name__ == "__main__":
    unittest.main()
