import unittest
from datetime import datetime, timezone

from src.object import object_unset


class TestObjectUnset(unittest.TestCase):
    def test_removes_top_level_property(self):
        self.assertEqual(object_unset({"a": 1, "b": 2}, "a"), {"b": 2})

    def test_removes_nested_property(self):
        self.assertEqual(object_unset({"a": {"b": 1, "c": 2}}, "a.b"), {"a": {"c": 2}})

    def test_accepts_list_path(self):
        self.assertEqual(
            object_unset({"a": {"b": 1, "c": 2}}, ["a", "c"]), {"a": {"b": 1}}
        )

    def test_does_not_mutate_input(self):
        source = {"a": {"b": 1, "c": 2}, "d": {"e": 3}}
        result = object_unset(source, "a.b")
        self.assertEqual(source, {"a": {"b": 1, "c": 2}, "d": {"e": 3}})
        self.assertIsNot(result, source)
        self.assertIsNot(result["a"], source["a"])
        self.assertIs(result["d"], source["d"])

    def test_returns_shallow_copy_when_path_missing(self):
        source = {"a": {"b": 1}}
        result = object_unset(source, "a.x.y")
        self.assertEqual(result, source)
        self.assertIsNot(result, source)

    def test_returns_shallow_copy_when_path_list_empty(self):
        source = {"a": 1}
        result = object_unset(source, [])
        self.assertEqual(result, {"a": 1})
        self.assertIsNot(result, source)

    def test_removes_list_element_and_shifts(self):
        source = {"a": [1, 2, 3]}
        self.assertEqual(object_unset(source, "a.1"), {"a": [1, 3]})
        self.assertEqual(source["a"], [1, 2, 3])

    def test_traverses_through_list_elements(self):
        self.assertEqual(
            object_unset({"a": [{"b": 1, "c": 2}]}, "a.0.b"), {"a": [{"c": 2}]}
        )

    def test_ignores_non_canonical_list_indexes(self):
        self.assertEqual(object_unset({"a": [1, 2]}, "a.01"), {"a": [1, 2]})
        self.assertEqual(object_unset({"a": [1, 2]}, "a.length"), {"a": [1, 2]})
        self.assertEqual(object_unset({"a": [1, 2]}, f"a.{chr(0x661)}"), {"a": [1, 2]})

    def test_ignores_out_of_range_list_indexes(self):
        self.assertEqual(object_unset({"a": [1, 2]}, "a.5"), {"a": [1, 2]})

    def test_stops_at_primitive_list_elements(self):
        source = {"a": [1, 2]}
        result = object_unset(source, ["a", "0", "x"])
        self.assertEqual(result, {"a": [1, 2]})
        self.assertIsNot(result["a"], source["a"])

    def test_stops_at_primitive_values(self):
        self.assertEqual(object_unset({"a": "text"}, "a.b"), {"a": "text"})

    def test_stops_at_non_container_objects(self):
        date = datetime(2020, 1, 1, tzinfo=timezone.utc)
        result = object_unset({"a": date}, "a.year")
        self.assertIs(result["a"], date)


if __name__ == "__main__":
    unittest.main()
