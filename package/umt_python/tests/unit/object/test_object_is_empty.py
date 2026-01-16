import unittest

from src.object import object_is_empty


class TestObjectIsEmpty(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(object_is_empty({}))
        self.assertFalse(object_is_empty({"a": 1}))

    def test_edge_cases(self):
        self.assertFalse(object_is_empty({"key": None}))
        self.assertFalse(object_is_empty({"key": ""}))

    def test_docstring_example(self):
        self.assertTrue(object_is_empty({}))
        self.assertFalse(object_is_empty({"a": 1}))


if __name__ == "__main__":
    unittest.main()
