import unittest

from src.validate import is_string


class TestIsString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_string("test"))
        self.assertTrue(is_string(""))
        self.assertTrue(is_string("hello world"))

    def test_invalid_values(self):
        self.assertFalse(is_string(123))
        self.assertFalse(is_string(None))
        self.assertFalse(is_string([]))
        self.assertFalse(is_string({}))

    def test_docstring_example(self):
        self.assertTrue(is_string("test"))
        self.assertFalse(is_string(123))


if __name__ == "__main__":
    unittest.main()
