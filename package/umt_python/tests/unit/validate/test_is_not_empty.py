import unittest

from src.validate import is_not_empty


class TestIsNotEmpty(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_not_empty({"a": 1}))
        self.assertTrue(is_not_empty({"key": "value"}))

    def test_empty_values(self):
        self.assertFalse(is_not_empty({}))

    def test_non_dict_values(self):
        self.assertFalse(is_not_empty([1, 2, 3]))
        self.assertFalse(is_not_empty("hello"))
        self.assertFalse(is_not_empty(123))
        self.assertFalse(is_not_empty(None))
        self.assertFalse(is_not_empty([]))
        self.assertFalse(is_not_empty(""))
        self.assertFalse(is_not_empty(set()))
        self.assertFalse(is_not_empty(tuple()))

    def test_docstring_example(self):
        self.assertFalse(is_not_empty({}))
        self.assertTrue(is_not_empty({"a": 1}))


if __name__ == "__main__":
    unittest.main()
