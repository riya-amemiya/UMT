import unittest

from src.math import get_decimal_length


class TestGetDecimalLengthParity(unittest.TestCase):
    def test_integral_float_has_no_decimals(self):
        self.assertEqual(get_decimal_length(1.0), 0)

    def test_small_number_printed_without_exponent(self):
        self.assertEqual(get_decimal_length(0.000015), 6)

    def test_exponent_notation_has_no_decimals(self):
        self.assertEqual(get_decimal_length(1e-7), 0)
        self.assertEqual(get_decimal_length(1e21), 0)

    def test_exponent_notation_with_mantissa_decimals(self):
        self.assertEqual(get_decimal_length(1.5e-7), 4)

    def test_negative_number(self):
        self.assertEqual(get_decimal_length(-1.5), 1)


if __name__ == "__main__":
    unittest.main()
