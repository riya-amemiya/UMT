import unittest

from src.math import is_double


class TestIsDouble(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_double(1.5))
        self.assertFalse(is_double(1))

    def test_with_loose_flag(self):
        self.assertTrue(is_double(0.1))
        self.assertTrue(is_double("0.1"))
        self.assertFalse(is_double("0.1", False))

    def test_invalid_string_loose(self):
        self.assertFalse(is_double("not a number"))
        self.assertFalse(is_double("abc"))

    def test_invalid_type_loose(self):
        self.assertFalse(is_double(None))
        self.assertFalse(is_double([1.5]))
        self.assertFalse(is_double({"value": 1.5}))

    def test_docstring_example(self):
        self.assertTrue(is_double(0.1))
        self.assertTrue(is_double("0.1"))
        self.assertFalse(is_double("0.1", False))


if __name__ == "__main__":
    unittest.main()
