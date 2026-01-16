import unittest

from src.validate import is_array


class TestIsArray(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_array([1, 2, 3]))
        self.assertTrue(is_array([]))
        self.assertTrue(is_array(["a", "b", "c"]))

    def test_invalid_values(self):
        self.assertFalse(is_array({}))
        self.assertFalse(is_array("string"))
        self.assertFalse(is_array(None))
        self.assertFalse(is_array((1, 2, 3)))

    def test_docstring_example(self):
        self.assertTrue(is_array([1, 2, 3]))
        self.assertFalse(is_array({}))


if __name__ == "__main__":
    unittest.main()
