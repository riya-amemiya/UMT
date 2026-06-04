import unittest

from src.math import convert_currency


class TestConvertCurrency(unittest.TestCase):
    def test_no_conversion_rates(self):
        self.assertEqual(convert_currency("¥1000"), "¥1000")

    def test_currency_symbol_does_not_match(self):
        self.assertEqual(convert_currency("¥1000", {"$": 1.1}), "¥1000")

    def test_convert_when_symbol_matches_and_rate_is_number(self):
        self.assertEqual(convert_currency("¥1000", {"¥": 1.1}), "1100")

    def test_rate_is_not_a_number(self):
        self.assertEqual(convert_currency("¥1000", {"¥": "invalid"}), "¥1000")

    def test_conversion_result_is_nan(self):
        self.assertEqual(convert_currency("¥NaN", {"¥": 1.1}), "¥NaN")

    def test_docstring_examples(self):
        self.assertEqual(convert_currency("¥100", {"¥": 0.01}), "1")
        self.assertEqual(convert_currency("$50", {"$": 1.2}), "60")
        self.assertEqual(convert_currency("€200", {"€": 1.1}), "220")


if __name__ == "__main__":
    unittest.main()
