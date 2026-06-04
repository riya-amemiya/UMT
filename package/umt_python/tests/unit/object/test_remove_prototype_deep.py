import re
import unittest

from src.object import remove_prototype_deep


class TestRemovePrototypeDeep(unittest.TestCase):
    def test_removes_polluting_properties_recursively(self):
        source = {
            "safe": {"nested": {"__proto__": {"polluted": True}, "value": 1}},
            "list": [
                {"constructor": {"prototype": {"polluted": True}}, "ok": 1},
                {"prototype": {"polluted": True}, "safe": 2},
            ],
        }
        result = remove_prototype_deep(source)
        self.assertEqual(
            result,
            {"safe": {"nested": {"value": 1}}, "list": [{"ok": 1}, {"safe": 2}]},
        )
        self.assertNotIn("__proto__", result["safe"]["nested"])
        self.assertNotIn("constructor", result["list"][0])
        self.assertNotIn("prototype", result["list"][1])

    def test_handles_arrays_containing_arrays_and_primitives(self):
        source = {
            "matrix": [
                [{"__proto__": {"polluted": True}, "a": 1}],
                ["text", 42, True, None],
            ]
        }
        self.assertEqual(
            remove_prototype_deep(source),
            {"matrix": [[{"a": 1}], ["text", 42, True, None]]},
        )

    def test_does_not_modify_original(self):
        source = {
            "safe": {"__proto__": {"polluted": True}, "value": 1},
            "list": [{"prototype": {"polluted": True}, "ok": 1}],
        }
        result = remove_prototype_deep(source)
        self.assertEqual(
            result,
            {"safe": {"value": 1}, "list": [{"ok": 1}]},
        )
        self.assertIn("__proto__", source["safe"])
        self.assertIn("prototype", source["list"][0])

    def test_handles_empty_object(self):
        self.assertEqual(remove_prototype_deep({}), {})

    def test_preserves_primitive_values(self):
        source = {"a": 1, "b": "text", "c": True, "d": None}
        self.assertEqual(remove_prototype_deep(source), source)

    def test_passes_non_plain_objects_by_reference(self):
        pattern = re.compile("foo")
        a_set = {1, 2}
        source = {"set": a_set, "regex": pattern}
        result = remove_prototype_deep(source)
        self.assertIs(result["set"], a_set)
        self.assertIs(result["regex"], pattern)

    def test_sanitizes_dangerous_keys_inside_arrays(self):
        source = {
            "list": [
                {"__proto__": {"polluted": True}, "value": 1},
                {"constructor": {"polluted": True}, "value": 2},
            ]
        }
        result = remove_prototype_deep(source)
        self.assertEqual(result["list"][0], {"value": 1})
        self.assertEqual(result["list"][1], {"value": 2})

    def test_sanitizes_dangerous_keys_at_multiple_depths(self):
        source = {"__proto__": 1, "a": {"__proto__": 2, "b": {"__proto__": 3, "c": 4}}}
        self.assertEqual(remove_prototype_deep(source), {"a": {"b": {"c": 4}}})

    def test_returns_new_array_reference(self):
        inner = [1, 2, 3]
        result = remove_prototype_deep({"list": inner})
        self.assertEqual(result["list"], inner)
        self.assertIsNot(result["list"], inner)

    def test_returns_new_plain_object_reference(self):
        nested = {"b": 1}
        result = remove_prototype_deep({"a": nested})
        self.assertEqual(result["a"], nested)
        self.assertIsNot(result["a"], nested)

    def test_recurses_through_nested_arrays(self):
        source = {"matrix": [[[{"__proto__": {"polluted": True}, "deep": 1}]]]}
        self.assertEqual(
            remove_prototype_deep(source),
            {"matrix": [[[{"deep": 1}]]]},
        )


if __name__ == "__main__":
    unittest.main()
