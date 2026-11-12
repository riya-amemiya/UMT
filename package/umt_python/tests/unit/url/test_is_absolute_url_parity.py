import unittest

from src.url import is_absolute_url

LONG_S = "\N{LATIN SMALL LETTER LONG S}"
KELVIN = "\N{KELVIN SIGN}"
DOTLESS_I = "\N{LATIN SMALL LETTER DOTLESS I}"
DOTTED_I = "\N{LATIN CAPITAL LETTER I WITH DOT ABOVE}"


class TestIsAbsoluteUrlParity(unittest.TestCase):
    def test_scheme_letters_are_ascii_only(self):
        self.assertFalse(is_absolute_url(LONG_S + ":"))
        self.assertFalse(is_absolute_url(KELVIN + ":x"))
        self.assertFalse(is_absolute_url(DOTTED_I + ":"))
        self.assertFalse(is_absolute_url("a" + DOTLESS_I + ":"))
        self.assertFalse(is_absolute_url("a" + DOTTED_I + ":"))
        self.assertFalse(is_absolute_url("a" + LONG_S + ":"))
        self.assertFalse(is_absolute_url("a" + KELVIN + ":"))

    def test_scheme_digits_are_ascii_only(self):
        self.assertFalse(is_absolute_url("a\N{ARABIC-INDIC DIGIT THREE}:"))
        self.assertTrue(is_absolute_url("A1+.-:"))

    def test_minimal_and_degenerate_inputs(self):
        self.assertTrue(is_absolute_url("a:"))
        self.assertFalse(is_absolute_url(":"))
        self.assertFalse(is_absolute_url("a\n:"))
        self.assertFalse(is_absolute_url("\nhttp:"))


if __name__ == "__main__":
    unittest.main()
