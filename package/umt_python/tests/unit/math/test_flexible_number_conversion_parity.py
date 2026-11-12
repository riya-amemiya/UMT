import math
import unittest

from src.math import flexible_number_conversion


class TestFlexibleNumberConversionParity(unittest.TestCase):
    def test_booleans_are_nan(self):
        self.assertTrue(math.isnan(flexible_number_conversion(True)))
        self.assertTrue(math.isnan(flexible_number_conversion(False)))

    def test_non_dict_objects_are_nan(self):
        self.assertTrue(math.isnan(flexible_number_conversion((1, 2))))
        self.assertTrue(math.isnan(flexible_number_conversion(object())))

    def test_nan_input_stays_nan(self):
        self.assertTrue(math.isnan(flexible_number_conversion(float("nan"))))

    def test_number_input_is_returned_unchanged(self):
        self.assertEqual(flexible_number_conversion(float("inf")), float("inf"))
        self.assertEqual(flexible_number_conversion(-1.5), -1.5)

    def test_invalid_radix_literals_are_nan(self):
        self.assertTrue(math.isnan(flexible_number_conversion("0xZZ")))
        self.assertTrue(math.isnan(flexible_number_conversion("0x")))
        self.assertTrue(math.isnan(flexible_number_conversion("0o9")))
        self.assertTrue(math.isnan(flexible_number_conversion("0b2")))

    def test_valid_radix_literals(self):
        self.assertEqual(flexible_number_conversion("0XfF"), 255)
        self.assertEqual(flexible_number_conversion("0o17"), 15)
        self.assertEqual(flexible_number_conversion("0b101"), 5)

    def test_leading_plus_sign(self):
        self.assertEqual(flexible_number_conversion("+42px"), 42)

    def test_second_decimal_point_ends_the_number(self):
        self.assertEqual(flexible_number_conversion("1.5.5"), 1.5)

    def test_exponent_before_trailing_text(self):
        self.assertEqual(flexible_number_conversion("1e3x"), 1000)
        self.assertEqual(flexible_number_conversion("1e+2z"), 100)

    def test_dangling_exponent_is_ignored(self):
        self.assertEqual(flexible_number_conversion("1e"), 1)

    def test_underscore_ends_the_number(self):
        self.assertEqual(flexible_number_conversion("1_000"), 1)

    def test_python_only_spellings_are_nan(self):
        self.assertTrue(math.isnan(flexible_number_conversion("inf")))
        self.assertTrue(math.isnan(flexible_number_conversion("nan")))
        self.assertTrue(math.isnan(flexible_number_conversion("infinityx")))

    def test_fullwidth_digits_are_nan(self):
        self.assertTrue(math.isnan(flexible_number_conversion("\uff11\uff12")))

    def test_whitespace_only_is_nan(self):
        self.assertTrue(math.isnan(flexible_number_conversion("   ")))

    def test_leading_dot_and_trailing_dot(self):
        self.assertEqual(flexible_number_conversion("-.5"), -0.5)
        self.assertEqual(flexible_number_conversion("5."), 5)

    def test_infinity_spellings(self):
        self.assertEqual(flexible_number_conversion(" +Infinity "), float("inf"))
        self.assertEqual(flexible_number_conversion("-INFINITY"), float("-inf"))
        self.assertEqual(flexible_number_conversion("infinity"), float("inf"))

    def test_javascript_whitespace_set(self):
        self.assertEqual(flexible_number_conversion("\ufeff1"), 1)
        self.assertTrue(math.isnan(flexible_number_conversion("\x1c1")))


if __name__ == "__main__":
    unittest.main()
