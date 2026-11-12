import unittest
from datetime import datetime

from src.string import apply_formatter, default_formatters


class TestDefaultFormatters(unittest.TestCase):
    def test_upper(self):
        self.assertEqual(default_formatters["upper"]("alice"), "ALICE")

    def test_lower(self):
        self.assertEqual(default_formatters["lower"]("ALICE"), "alice")

    def test_currency_usd(self):
        self.assertEqual(
            default_formatters["currency"](1000, "en-US", "USD"), "$1,000.00"
        )

    def test_currency_jpy(self):
        self.assertEqual(default_formatters["currency"](1000, "ja-JP", "JPY"), "¥1,000")

    def test_number_precision(self):
        self.assertEqual(
            default_formatters["number"](1234.5, "en-US", "2", "2"), "1,234.50"
        )

    def test_plural_singular(self):
        self.assertEqual(default_formatters["plural"](1, "item", "items"), "item")

    def test_plural_plural(self):
        self.assertEqual(default_formatters["plural"](2, "item", "items"), "items")

    def test_pad(self):
        self.assertEqual(default_formatters["pad"](5, "3", "0"), "005")

    def test_date_iso(self):
        date = datetime(2024, 1, 2, 3, 4, 5, 678000)
        self.assertEqual(
            default_formatters["date"](date, "en-US", "iso"),
            "2024-01-02T03:04:05.678Z",
        )

    def test_via_apply_formatter(self):
        self.assertEqual(apply_formatter("alice", "upper", default_formatters), "ALICE")
        self.assertEqual(
            apply_formatter(1000, "currency(ja-JP,JPY)", default_formatters),
            "¥1,000",
        )
        self.assertEqual(
            apply_formatter(1, "plural(item,items)", default_formatters), "item"
        )


class TestPadFormatterLength(unittest.TestCase):
    def test_non_numeric_length_adds_no_padding(self):
        self.assertEqual(default_formatters["pad"](5, "abc"), "5")

    def test_infinite_length_adds_no_padding(self):
        self.assertEqual(default_formatters["pad"](5, "Infinity"), "5")

    def test_negative_length_adds_no_padding(self):
        self.assertEqual(default_formatters["pad"](5, "-1"), "5")

    def test_default_length_and_char(self):
        self.assertEqual(default_formatters["pad"](5), "05")

    def test_space_pad_char(self):
        self.assertEqual(default_formatters["pad"](5, "3", " "), "  5")

    def test_radix_literal_length(self):
        self.assertEqual(default_formatters["pad"](5, "0x5"), "00005")

    def test_python_only_numeric_spelling_adds_no_padding(self):
        self.assertEqual(default_formatters["pad"](5, "1_0"), "5")
        self.assertEqual(default_formatters["pad"](5, "inf"), "5")


class TestPluralFormatterMissingArguments(unittest.TestCase):
    def test_missing_plural_form(self):
        self.assertEqual(default_formatters["plural"](1, "item"), "item")
        self.assertEqual(default_formatters["plural"](2, "item"), "undefined")

    def test_missing_both_forms(self):
        self.assertEqual(default_formatters["plural"](1), "undefined")

    def test_python_only_numeric_spelling_is_not_one(self):
        self.assertEqual(default_formatters["plural"]("1_0", "a", "b"), "b")
        self.assertEqual(default_formatters["plural"]("0x1", "a", "b"), "a")


if __name__ == "__main__":
    unittest.main()
