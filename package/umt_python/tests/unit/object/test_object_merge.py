import unittest

from src.object import object_merge


class TestObjectMerge(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            object_merge({"a": 1}, {"b": 2}, {"c": 3}),
            {"a": 1, "b": 2, "c": 3},
        )

    def test_overwrite(self):
        self.assertEqual(
            object_merge({"a": 1}, {"a": 2}),
            {"a": 2},
        )

    def test_edge_cases(self):
        self.assertEqual(object_merge({}, {}), {})
        self.assertEqual(object_merge({"a": 1}), {"a": 1})

    def test_does_not_mutate_original(self):
        original = {"a": 1}
        result = object_merge(original, {"b": 2})
        self.assertEqual(original, {"a": 1})
        self.assertEqual(result, {"a": 1, "b": 2})

    def test_docstring_example(self):
        self.assertEqual(
            object_merge({"a": 1}, {"b": 2}, {"c": 3}),
            {"a": 1, "b": 2, "c": 3},
        )


if __name__ == "__main__":
    unittest.main()
