import unittest

from src.validate import is_value_nan


class TestIsValueNan(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_value_nan(float("nan")))
        self.assertFalse(is_value_nan(1))
        self.assertFalse(is_value_nan(0))

    def test_edge_cases(self):
        self.assertFalse(is_value_nan(float("inf")))
        self.assertFalse(is_value_nan(None))
        self.assertFalse(is_value_nan("nan"))

    def test_loose_mode_with_string_nan(self):
        self.assertTrue(is_value_nan("NaN", loose=True))
        self.assertTrue(is_value_nan("nan", loose=True))

    def test_loose_mode_with_numeric_string(self):
        self.assertFalse(is_value_nan("123", loose=True))
        self.assertFalse(is_value_nan("1.5", loose=True))

    def test_loose_mode_with_float_nan(self):
        self.assertTrue(is_value_nan(float("nan"), loose=True))

    def test_loose_mode_with_invalid_value(self):
        self.assertFalse(is_value_nan("not a number", loose=True))
        self.assertFalse(is_value_nan(None, loose=True))
        self.assertFalse(is_value_nan([], loose=True))
        self.assertFalse(is_value_nan({}, loose=True))

    def test_docstring_examples(self):
        self.assertFalse(is_value_nan(1))
        self.assertFalse(is_value_nan("NaN"))
        self.assertTrue(is_value_nan("NaN", True))
        self.assertTrue(is_value_nan(float("nan")))


if __name__ == "__main__":
    unittest.main()
