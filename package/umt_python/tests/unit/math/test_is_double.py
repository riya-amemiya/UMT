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

    def test_docstring_example(self):
        self.assertTrue(is_double(0.1))
        self.assertTrue(is_double("0.1"))
        self.assertFalse(is_double("0.1", False))


if __name__ == "__main__":
    unittest.main()
