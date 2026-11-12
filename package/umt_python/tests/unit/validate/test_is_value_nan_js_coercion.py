import unittest

from src.validate import is_value_nan


class TestIsValueNanJsCoercion(unittest.TestCase):
    def test_loose_mode_returns_true_for_non_numeric_strings(self):
        self.assertTrue(is_value_nan("a", True))
        self.assertTrue(is_value_nan("abc", True))
        self.assertTrue(is_value_nan("not a number", True))
        self.assertTrue(is_value_nan("NaN", True))

    def test_loose_mode_returns_true_for_python_only_spellings(self):
        self.assertTrue(is_value_nan("inf", True))
        self.assertTrue(is_value_nan("1_000", True))

    def test_loose_mode_returns_false_for_numeric_strings(self):
        self.assertFalse(is_value_nan("0", True))
        self.assertFalse(is_value_nan("-1", True))
        self.assertFalse(is_value_nan("Infinity", True))
        self.assertFalse(is_value_nan("0x10", True))
        self.assertFalse(is_value_nan(" 1.5 ", True))

    def test_loose_mode_returns_false_for_empty_and_whitespace_strings(self):
        self.assertFalse(is_value_nan("", True))
        self.assertFalse(is_value_nan(" ", True))

    def test_loose_mode_keeps_non_string_coercion(self):
        self.assertTrue(is_value_nan(float("nan"), True))
        self.assertFalse(is_value_nan(1, True))
        self.assertFalse(is_value_nan(None, True))

    def test_strict_mode_returns_false_for_strings(self):
        self.assertFalse(is_value_nan("abc"))
        self.assertFalse(is_value_nan("NaN"))


if __name__ == "__main__":
    unittest.main()
