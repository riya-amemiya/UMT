import math
import unittest

from src.math import flexible_number_conversion


class TestFlexibleNumberConversion(unittest.TestCase):
    def test_integer_input(self):
        self.assertEqual(flexible_number_conversion(123), 123)

    def test_float_input(self):
        self.assertEqual(flexible_number_conversion(78.9), 78.9)

    def test_string_integer(self):
        self.assertEqual(flexible_number_conversion("456"), 456.0)

    def test_string_float(self):
        self.assertEqual(flexible_number_conversion("78.9"), 78.9)

    def test_scientific_notation(self):
        self.assertEqual(flexible_number_conversion("3.14e2"), 314.0)

    def test_hex_notation(self):
        self.assertEqual(flexible_number_conversion("0xFF"), 255.0)

    def test_octal_notation(self):
        self.assertEqual(flexible_number_conversion("0o10"), 8.0)

    def test_binary_notation(self):
        self.assertEqual(flexible_number_conversion("0b1010"), 10.0)

    def test_leading_number(self):
        self.assertEqual(flexible_number_conversion("42px"), 42.0)

    def test_empty_string(self):
        self.assertEqual(flexible_number_conversion(""), 0)

    def test_none(self):
        self.assertEqual(flexible_number_conversion(None), 0)

    def test_dict_input(self):
        result = flexible_number_conversion({"key": "value"})
        self.assertTrue(math.isnan(result))

    def test_list_input(self):
        result = flexible_number_conversion([1, 2, 3])
        self.assertTrue(math.isnan(result))

    def test_invalid_string(self):
        result = flexible_number_conversion("not a number")
        self.assertTrue(math.isnan(result))

    def test_infinity(self):
        self.assertEqual(flexible_number_conversion("infinity"), float("inf"))
        self.assertEqual(flexible_number_conversion("+infinity"), float("inf"))
        self.assertEqual(flexible_number_conversion("-infinity"), float("-inf"))

    def test_invalid_hex_extracts_leading_zero(self):
        result = flexible_number_conversion("0xZZZ")
        self.assertEqual(result, 0)

    def test_invalid_octal_extracts_leading_zero(self):
        result = flexible_number_conversion("0o999")
        self.assertEqual(result, 0)

    def test_invalid_binary_extracts_leading_zero(self):
        result = flexible_number_conversion("0b222")
        self.assertEqual(result, 0)

    def test_partial_number_dash_only(self):
        result = flexible_number_conversion("-abc")
        self.assertTrue(math.isnan(result))

    def test_partial_number_dot_only(self):
        result = flexible_number_conversion(".abc")
        self.assertTrue(math.isnan(result))

    def test_multiple_dots_invalid_float(self):
        result = flexible_number_conversion("1.2.3abc")
        self.assertTrue(math.isnan(result))

    def test_docstring_example(self):
        self.assertEqual(flexible_number_conversion(123), 123)
        self.assertEqual(flexible_number_conversion("456"), 456.0)
        self.assertEqual(flexible_number_conversion("78.9"), 78.9)
        self.assertEqual(flexible_number_conversion("3.14e2"), 314.0)
        self.assertEqual(flexible_number_conversion("0xFF"), 255.0)
        self.assertEqual(flexible_number_conversion("42px"), 42.0)
        self.assertEqual(flexible_number_conversion(""), 0)
        self.assertTrue(math.isnan(flexible_number_conversion("not a number")))


if __name__ == "__main__":
    unittest.main()
