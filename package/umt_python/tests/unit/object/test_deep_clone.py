import re
import unittest
from datetime import datetime

from src.object import deep_clone


class TestDeepClone(unittest.TestCase):
    def test_simple_object(self):
        original = {"a": 1, "b": "hello"}
        cloned = deep_clone(original)
        self.assertEqual(cloned, original)
        self.assertIsNot(cloned, original)

    def test_nested_objects(self):
        original = {"a": {"b": {"c": 3}}}
        cloned = deep_clone(original)
        self.assertEqual(cloned, original)
        self.assertIsNot(cloned["a"], original["a"])
        self.assertIsNot(cloned["a"]["b"], original["a"]["b"])

    def test_lists(self):
        original = [1, [2, [3]]]
        cloned = deep_clone(original)
        self.assertEqual(cloned, original)
        self.assertIsNot(cloned, original)
        self.assertIsNot(cloned[1], original[1])

    def test_datetime(self):
        dt = datetime(2024, 1, 1)
        original = {"date": dt}
        cloned = deep_clone(original)
        self.assertEqual(cloned["date"], original["date"])
        self.assertIsNot(cloned["date"], original["date"])

    def test_set(self):
        original = {"items": {1, 2, 3}}
        cloned = deep_clone(original)
        self.assertEqual(cloned["items"], original["items"])
        self.assertIsNot(cloned["items"], original["items"])

    def test_primitives(self):
        self.assertEqual(deep_clone(42), 42)
        self.assertEqual(deep_clone("hello"), "hello")
        self.assertEqual(deep_clone(True), True)
        self.assertIsNone(deep_clone(None))

    def test_mutation_isolation(self):
        original = {"a": 1, "b": {"c": 2}}
        cloned = deep_clone(original)
        cloned["b"]["c"] = 99
        self.assertEqual(original["b"]["c"], 2)

    def test_regex_pattern(self):
        original = {"pattern": re.compile(r"test", re.IGNORECASE)}
        cloned = deep_clone(original)
        self.assertEqual(cloned["pattern"].pattern, "test")
        self.assertEqual(cloned["pattern"].flags, original["pattern"].flags)
        self.assertIsNot(cloned["pattern"], original["pattern"])

    def test_tuple(self):
        original = (1, (2, 3))
        cloned = deep_clone(original)
        self.assertEqual(cloned, original)

    def test_frozenset(self):
        original = frozenset([1, 2, 3])
        cloned = deep_clone(original)
        self.assertEqual(cloned, original)

    def test_bytes(self):
        self.assertEqual(deep_clone(b"hello"), b"hello")

    def test_empty_containers(self):
        self.assertEqual(deep_clone({}), {})
        self.assertEqual(deep_clone([]), [])
        self.assertEqual(deep_clone(set()), set())

    def test_mixed_nested_structure(self):
        original = {
            "list": [1, {"nested": True}],
            "set": {10, 20},
            "tuple": (3, 4),
        }
        cloned = deep_clone(original)
        self.assertEqual(cloned, original)
        self.assertIsNot(cloned["list"], original["list"])
        self.assertIsNot(cloned["list"][1], original["list"][1])

    def test_docstring_example(self):
        original = {"a": 1, "b": {"c": 2}}
        cloned = deep_clone(original)
        cloned["b"]["c"] = 99
        self.assertEqual(original["b"]["c"], 2)


if __name__ == "__main__":
    unittest.main()
