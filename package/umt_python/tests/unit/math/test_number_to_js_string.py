import unittest

from src.math.number_to_js_string import number_to_js_string


class TestNumberToJsString(unittest.TestCase):
    def test_special_values(self):
        self.assertEqual(number_to_js_string(float("nan")), "NaN")
        self.assertEqual(number_to_js_string(float("inf")), "Infinity")
        self.assertEqual(number_to_js_string(float("-inf")), "-Infinity")

    def test_zero_and_negative_zero(self):
        self.assertEqual(number_to_js_string(0), "0")
        self.assertEqual(number_to_js_string(-0.0), "0")

    def test_integral_float_has_no_fraction(self):
        self.assertEqual(number_to_js_string(4.0), "4")
        self.assertEqual(number_to_js_string(-123.0), "-123")

    def test_plain_decimal(self):
        self.assertEqual(number_to_js_string(0.3), "0.3")

    def test_small_number_above_exponent_threshold(self):
        self.assertEqual(number_to_js_string(0.000015), "0.000015")
        self.assertEqual(
            number_to_js_string(1.2345678901234567e-05), "0.000012345678901234568"
        )

    def test_large_number_below_exponent_threshold(self):
        self.assertEqual(number_to_js_string(1e16), "10000000000000000")
        self.assertEqual(
            number_to_js_string(1.2345678901234567e20), "123456789012345670000"
        )

    def test_exponent_notation(self):
        self.assertEqual(number_to_js_string(1e21), "1e+21")
        self.assertEqual(number_to_js_string(1.5e-7), "1.5e-7")
        self.assertEqual(number_to_js_string(-1e-7), "-1e-7")

    def test_large_integer_uses_exponent_notation(self):
        self.assertEqual(number_to_js_string(10**21), "1e+21")


if __name__ == "__main__":
    unittest.main()
