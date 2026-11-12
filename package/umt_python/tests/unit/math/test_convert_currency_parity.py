import unittest

from src.math import convert_currency


class TestConvertCurrencyParity(unittest.TestCase):
    def test_hexadecimal_amount(self):
        self.assertEqual(convert_currency("$0x10", {"$": 2}), "32")

    def test_exponent_amount(self):
        self.assertEqual(convert_currency("$1e2", {"$": 2}), "200")

    def test_python_only_infinity_spelling_is_not_converted(self):
        self.assertEqual(convert_currency("$inf", {"$": 2}), "$inf")

    def test_underscore_separator_is_not_converted(self):
        self.assertEqual(convert_currency("$1_000", {"$": 2}), "$1_000")

    def test_empty_amount_converts_to_zero(self):
        self.assertEqual(convert_currency("$", {"$": 2}), "0")


if __name__ == "__main__":
    unittest.main()
