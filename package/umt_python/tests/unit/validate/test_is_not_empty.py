import unittest

from src.validate import is_not_empty


class TestIsNotEmpty(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_not_empty({"a": 1}))
        self.assertTrue(is_not_empty({"key": "value"}))

    def test_empty_values(self):
        self.assertFalse(is_not_empty({}))

    def test_docstring_example(self):
        self.assertFalse(is_not_empty({}))
        self.assertTrue(is_not_empty({"a": 1}))


if __name__ == "__main__":
    unittest.main()
