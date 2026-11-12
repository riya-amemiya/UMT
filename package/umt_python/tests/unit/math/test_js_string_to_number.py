import math
import unittest

from src.math.js_string_to_number import js_string_to_number


class TestJsStringToNumber(unittest.TestCase):
    def test_empty_and_whitespace_are_zero(self):
        self.assertEqual(js_string_to_number(""), 0)
        self.assertEqual(js_string_to_number("  "), 0)

    def test_decimal_literals(self):
        self.assertEqual(js_string_to_number("12.5"), 12.5)
        self.assertEqual(js_string_to_number(" -3 "), -3)
        self.assertEqual(js_string_to_number("+.5"), 0.5)
        self.assertEqual(js_string_to_number("5."), 5)
        self.assertEqual(js_string_to_number("1e3"), 1000)
        self.assertEqual(js_string_to_number("1.5E-2"), 0.015)

    def test_infinity(self):
        self.assertEqual(js_string_to_number("Infinity"), float("inf"))
        self.assertEqual(js_string_to_number("+Infinity"), float("inf"))
        self.assertEqual(js_string_to_number("-Infinity"), float("-inf"))

    def test_radix_literals(self):
        self.assertEqual(js_string_to_number("0x10"), 16)
        self.assertEqual(js_string_to_number("0XfF"), 255)
        self.assertEqual(js_string_to_number("0o17"), 15)
        self.assertEqual(js_string_to_number("0b101"), 5)

    def test_signed_or_invalid_radix_literals_are_nan(self):
        self.assertTrue(math.isnan(js_string_to_number("-0x10")))
        self.assertTrue(math.isnan(js_string_to_number("0x")))
        self.assertTrue(math.isnan(js_string_to_number("0o8")))
        self.assertTrue(math.isnan(js_string_to_number("0b2")))
        self.assertTrue(math.isnan(js_string_to_number("0x_1")))

    def test_python_only_spellings_are_nan(self):
        self.assertTrue(math.isnan(js_string_to_number("inf")))
        self.assertTrue(math.isnan(js_string_to_number("infinity")))
        self.assertTrue(math.isnan(js_string_to_number("nan")))
        self.assertTrue(math.isnan(js_string_to_number("1_000")))
        self.assertTrue(math.isnan(js_string_to_number("\uff11\uff12")))

    def test_non_numeric_text_is_nan(self):
        self.assertTrue(math.isnan(js_string_to_number("abc")))
        self.assertTrue(math.isnan(js_string_to_number("12px")))
        self.assertTrue(math.isnan(js_string_to_number("-")))
        self.assertTrue(math.isnan(js_string_to_number(".")))
        self.assertTrue(math.isnan(js_string_to_number("1e")))

    def test_javascript_whitespace_set(self):
        self.assertEqual(js_string_to_number("\ufeff1\u00a0"), 1)
        self.assertEqual(js_string_to_number("\t\n 7 \u3000"), 7)
        self.assertTrue(math.isnan(js_string_to_number("\x1c1")))
        self.assertTrue(math.isnan(js_string_to_number("\x851")))

    def test_radix_literal_overflow_is_infinity(self):
        self.assertEqual(js_string_to_number("0x" + "f" * 300), float("inf"))


if __name__ == "__main__":
    unittest.main()
