import unittest

from src.object import object_pick_deep


class TestObjectPickDeep(unittest.TestCase):
    def test_basic_cases(self):
        obj = {"a": 1, "b": 2, "c": 3}
        self.assertEqual(object_pick_deep(obj, "a", "c"), {"a": 1, "c": 3})

    def test_deep_pick(self):
        obj = {"a": {"b": {"c": 1, "d": 2}, "e": 3}, "f": 4}
        self.assertEqual(
            object_pick_deep(obj, "a.b.c", "f"),
            {"a": {"b": {"c": 1}}, "f": 4},
        )

    def test_nested_pick_multiple(self):
        obj = {"a": {"b": 1, "c": 2}, "d": {"e": 3}}
        self.assertEqual(
            object_pick_deep(obj, "a.b", "d.e"),
            {"a": {"b": 1}, "d": {"e": 3}},
        )

    def test_non_existent_key(self):
        obj = {"a": {"b": 1}}
        self.assertEqual(object_pick_deep(obj, "x.y.z"), {})
        self.assertEqual(object_pick_deep(obj, "a.x"), {"a": {}})

    def test_empty_object(self):
        self.assertEqual(object_pick_deep({}, "a", "b"), {})

    def test_no_keys(self):
        obj = {"a": 1}
        self.assertEqual(object_pick_deep(obj), {})

    def test_partial_path_exists(self):
        obj = {"a": {"b": 1}}

        result = object_pick_deep(obj, "a.c")
        self.assertEqual(result, {"a": {}})

    def test_non_dict_in_path(self):
        obj = {"a": 1}
        self.assertEqual(object_pick_deep(obj, "a.b"), {"a": {}})

    def test_picking_with_existing_target_key(self):
        obj = {"a": {"b": 1, "c": 2}}
        self.assertEqual(
            object_pick_deep(obj, "a.b", "a.c"),
            {"a": {"b": 1, "c": 2}},
        )

    def test_docstring_example(self):
        obj = {"a": {"b": {"c": 1, "d": 2}, "e": 3}, "f": 4}
        self.assertEqual(
            object_pick_deep(obj, "a.b.c", "f"),
            {"a": {"b": {"c": 1}}, "f": 4},
        )


if __name__ == "__main__":
    unittest.main()
