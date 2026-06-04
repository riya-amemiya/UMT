import unittest

from src.object import object_set


class TestObjectSet(unittest.TestCase):
    def test_sets_top_level_property(self):
        target = {}
        object_set(target, "a", 1)
        self.assertEqual(target, {"a": 1})

    def test_creates_nested_objects_when_missing(self):
        target = {}
        object_set(target, "a.b.c", 1)
        self.assertEqual(target, {"a": {"b": {"c": 1}}})

    def test_respects_array_path_input(self):
        target = {}
        object_set(target, ["a", "b"], 2)
        self.assertEqual(target, {"a": {"b": 2}})

    def test_overwrites_existing_leaf_values(self):
        target = {"a": {"b": 1}}
        object_set(target, "a.b", 2)
        self.assertEqual(target, {"a": {"b": 2}})

    def test_replaces_non_object_intermediate_values(self):
        target = {"a": 5}
        object_set(target, "a.b", 1)
        self.assertEqual(target, {"a": {"b": 1}})

    def test_returns_same_object_reference(self):
        target = {}
        self.assertIs(object_set(target, "a", 1), target)

    def test_returns_object_unchanged_when_path_empty(self):
        target = {"a": 1}
        object_set(target, [], 2)
        self.assertEqual(target, {"a": 1})


if __name__ == "__main__":
    unittest.main()
