import unittest

from src.object import object_omit


class TestObjectOmit(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            object_omit({"a": 1, "b": 2, "c": 3}, "b", "c"),
            {"a": 1},
        )

    def test_missing_keys(self):
        self.assertEqual(
            object_omit({"a": 1, "b": 2}, "c"),
            {"a": 1, "b": 2},
        )

    def test_edge_cases(self):
        self.assertEqual(object_omit({}, "a"), {})
        self.assertEqual(object_omit({"a": 1, "b": 2}), {"a": 1, "b": 2})

    def test_does_not_mutate_original(self):
        original = {"a": 1, "b": 2}
        result = object_omit(original, "b")
        self.assertEqual(original, {"a": 1, "b": 2})
        self.assertEqual(result, {"a": 1})

    def test_docstring_example(self):
        self.assertEqual(
            object_omit({"a": 1, "b": 2, "c": 3}, "b", "c"),
            {"a": 1},
        )


if __name__ == "__main__":
    unittest.main()
