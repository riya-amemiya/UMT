import unittest

from src.validate import is_dictionary_object


class TestIsDictionaryObject(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_dictionary_object({}))
        self.assertTrue(is_dictionary_object({"a": 1}))

    def test_invalid_values(self):
        self.assertFalse(is_dictionary_object([]))
        self.assertFalse(is_dictionary_object("string"))
        self.assertFalse(is_dictionary_object(None))


if __name__ == "__main__":
    unittest.main()
