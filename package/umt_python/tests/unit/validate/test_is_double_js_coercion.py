import unittest
from decimal import Decimal

from src.validate import is_double


class TestIsDoubleJsCoercion(unittest.TestCase):
    def test_accepts_surrounding_whitespace(self):
        self.assertTrue(is_double(" 1.5 "))

    def test_accepts_exponent_notation(self):
        self.assertTrue(is_double("15e-1"))
        self.assertFalse(is_double("1.5e1"))

    def test_rejects_integer_like_strings(self):
        self.assertFalse(is_double(""))
        self.assertFalse(is_double("0x10"))
        self.assertFalse(is_double("2.0"))

    def test_rejects_python_only_spellings(self):
        self.assertFalse(is_double("1_0.5"))
        self.assertFalse(is_double(chr(0xFF11) + ".5"))

    def test_rejects_non_finite_strings(self):
        self.assertFalse(is_double("Infinity"))
        self.assertFalse(is_double("NaN"))
        self.assertFalse(is_double("abc"))

    def test_loose_mode_keeps_non_string_coercion(self):
        self.assertTrue(is_double(Decimal("1.5")))
        self.assertFalse(is_double(object()))

    def test_strict_mode_rejects_non_numbers(self):
        self.assertFalse(is_double(Decimal("1.5"), False))
        self.assertTrue(is_double(1.5, False))


if __name__ == "__main__":
    unittest.main()
