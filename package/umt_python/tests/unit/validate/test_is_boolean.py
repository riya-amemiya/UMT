import unittest

from src.validate import is_boolean


class TestIsBoolean(unittest.TestCase):
    def test_boolean_values(self):
        self.assertTrue(is_boolean(True))
        self.assertTrue(is_boolean(False))

    def test_non_boolean_values(self):
        self.assertFalse(is_boolean(0))
        self.assertFalse(is_boolean(1))
        self.assertFalse(is_boolean(None))
        self.assertFalse(is_boolean("true"))
        self.assertFalse(is_boolean([]))
        self.assertFalse(is_boolean({}))

    def test_docstring_example(self):
        self.assertTrue(is_boolean(True))
        self.assertTrue(is_boolean(False))
        self.assertFalse(is_boolean(0))
        self.assertFalse(is_boolean("true"))


if __name__ == "__main__":
    unittest.main()
