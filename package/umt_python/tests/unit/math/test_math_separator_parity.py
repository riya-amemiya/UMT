import unittest

from src.math import math_separator


class TestMathSeparatorParity(unittest.TestCase):
    def test_exponent_string_uses_its_text_length(self):
        self.assertEqual(math_separator("1e3"), (100, 900))

    def test_decimal_number(self):
        self.assertEqual(math_separator(12.5), (10, 2.5))

    def test_decimal_string_with_single_digit(self):
        self.assertEqual(math_separator("0.5"), (0, 0.5))

    def test_integral_float(self):
        self.assertEqual(math_separator(1250.0), (1000, 250))

    def test_negative_number_counts_the_sign(self):
        self.assertEqual(math_separator(-125), (1000, -1125))

    def test_trailing_dot_string(self):
        self.assertEqual(math_separator("12."), (10, 2))

    def test_leading_dot_string(self):
        primary, remainder = math_separator(".5")
        self.assertEqual(primary, 0.1)
        self.assertEqual(remainder, 0.4)

    def test_zero(self):
        self.assertEqual(math_separator(0), (0, 0))

    def test_invalid_input(self):
        self.assertEqual(math_separator("abc"), (0, 0))


if __name__ == "__main__":
    unittest.main()
