import unittest

from src.object import flatten_object


class TestFlattenObject(unittest.TestCase):
    def test_flattens_nested_object_with_dot(self):
        self.assertEqual(flatten_object({"a": {"b": {"c": 1}}}), {"a.b.c": 1})

    def test_preserves_top_level_keys(self):
        self.assertEqual(flatten_object({"a": 1, "b": 2}), {"a": 1, "b": 2})

    def test_supports_custom_separator(self):
        self.assertEqual(flatten_object({"a": {"b": 1}}, "/"), {"a/b": 1})

    def test_keeps_arrays_as_values(self):
        self.assertEqual(flatten_object({"a": {"b": [1, 2]}}), {"a.b": [1, 2]})

    def test_empty_plain_objects_are_leaf_values(self):
        self.assertEqual(flatten_object({"a": {}}), {"a": {}})

    def test_returns_empty_for_empty_input(self):
        self.assertEqual(flatten_object({}), {})

    def test_docstring_example(self):
        self.assertEqual(
            flatten_object({"a": {"b": {"c": 1}}, "d": 2}),
            {"a.b.c": 1, "d": 2},
        )


if __name__ == "__main__":
    unittest.main()
