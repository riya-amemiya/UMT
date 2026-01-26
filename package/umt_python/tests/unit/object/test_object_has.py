import unittest

from src.object import object_has


class TestObjectHas(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(object_has({"a": {"b": 1}}, "a.b"))
        self.assertTrue(object_has({"a": {"b": 1}}, ["a", "b"]))
        self.assertFalse(object_has({"a": {"b": 1}}, "a.c"))

    def test_single_level(self):
        self.assertTrue(object_has({"a": 1}, "a"))
        self.assertFalse(object_has({"a": 1}, "b"))

    def test_nested_path(self):
        obj = {"a": {"b": {"c": 1}}}
        self.assertTrue(object_has(obj, "a.b.c"))
        self.assertFalse(object_has(obj, "a.b.d"))

    def test_edge_cases(self):
        self.assertFalse(object_has({}, "a"))

    def test_docstring_example(self):
        self.assertTrue(object_has({"a": {"b": 1}}, "a.b"))
        self.assertTrue(object_has({"a": {"b": 1}}, ["a", "b"]))
        self.assertFalse(object_has({"a": {"b": 1}}, "a.c"))


if __name__ == "__main__":
    unittest.main()
