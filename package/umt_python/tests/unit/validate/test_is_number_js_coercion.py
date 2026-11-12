import unittest
from decimal import Decimal

from src.validate import is_number


class TestIsNumberJsCoercion(unittest.TestCase):
    def test_empty_and_whitespace_strings_are_numbers(self):
        self.assertTrue(is_number(""))
        self.assertTrue(is_number("  "))

    def test_accepts_surrounding_whitespace(self):
        self.assertTrue(is_number(" 42 "))
        self.assertTrue(is_number("\t1.5\n"))

    def test_accepts_radix_literals(self):
        self.assertTrue(is_number("0x10"))
        self.assertTrue(is_number("0b11"))
        self.assertTrue(is_number("0o17"))

    def test_rejects_python_only_spellings(self):
        self.assertFalse(is_number("1_000"))
        self.assertFalse(is_number("inf"))
        self.assertFalse(is_number("nan"))
        self.assertFalse(is_number(chr(0xFF11)))

    def test_rejects_non_finite_strings(self):
        self.assertFalse(is_number("Infinity"))
        self.assertFalse(is_number("-Infinity"))
        self.assertFalse(is_number("NaN"))
        self.assertFalse(is_number("1e400"))

    def test_strict_mode_rejects_strings(self):
        self.assertFalse(is_number("", False))
        self.assertFalse(is_number("0x10", False))

    def test_loose_mode_keeps_non_string_coercion(self):
        self.assertTrue(is_number(Decimal("1.5")))
        self.assertFalse(is_number(object()))


if __name__ == "__main__":
    unittest.main()
