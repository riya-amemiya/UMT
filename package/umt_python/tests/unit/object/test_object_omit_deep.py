import unittest

from src.object import object_omit_deep


class TestObjectOmitDeep(unittest.TestCase):
    def test_removes_simple_keys(self):
        self.assertEqual(object_omit_deep({"a": 1, "b": 2, "c": 3}, "a", "c"), {"b": 2})

    def test_removes_nested_keys(self):
        obj = {"a": {"b": {"c": 1, "d": 2}, "e": 3}, "f": 4}
        self.assertEqual(
            object_omit_deep(obj, "a.b.c", "f"), {"a": {"b": {"d": 2}, "e": 3}}
        )

    def test_does_not_mutate_input(self):
        obj = {"a": {"b": 1, "c": 2}, "d": {"e": 3}}
        result = object_omit_deep(obj, "a.b")
        self.assertEqual(obj, {"a": {"b": 1, "c": 2}, "d": {"e": 3}})
        self.assertIs(result["d"], obj["d"])

    def test_returns_shallow_copy_when_no_keys(self):
        obj = {"a": 1}
        result = object_omit_deep(obj)
        self.assertEqual(result, {"a": 1})
        self.assertIsNot(result, obj)

    def test_ignores_missing_keys(self):
        self.assertEqual(object_omit_deep({"a": {"b": 1}}, "a.x", "z"), {"a": {"b": 1}})

    def test_removes_whole_nested_object(self):
        self.assertEqual(object_omit_deep({"a": {"b": 1}, "c": 2}, "a"), {"c": 2})

    def test_removes_keys_from_dicts_inside_lists(self):
        self.assertEqual(
            object_omit_deep({"a": [{"b": 1, "c": 2}]}, "a.0.b"), {"a": [{"c": 2}]}
        )


if __name__ == "__main__":
    unittest.main()
