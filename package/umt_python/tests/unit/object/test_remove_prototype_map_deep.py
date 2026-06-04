import unittest

from src.object import remove_prototype_map_deep


class TestRemovePrototypeMapDeep(unittest.TestCase):
    def test_removes_polluting_properties_recursively_from_each(self):
        objects = [
            {"safe": {"__proto__": {"polluted": True}, "value": 1}},
            {
                "list": [{"prototype": {"polluted": True}, "ok": 1}],
                "meta": {"constructor": {"prototype": {"polluted": True}}, "keep": 2},
            },
        ]
        result = remove_prototype_map_deep(objects)
        self.assertEqual(
            result,
            [
                {"safe": {"value": 1}},
                {"list": [{"ok": 1}], "meta": {"keep": 2}},
            ],
        )
        self.assertNotIn("__proto__", result[0]["safe"])
        self.assertNotIn("prototype", result[1]["list"][0])
        self.assertNotIn("constructor", result[1]["meta"])

    def test_returns_empty_array_for_empty_input(self):
        self.assertEqual(remove_prototype_map_deep([]), [])

    def test_returns_new_list_instance(self):
        objects = [{"a": 1}]
        result = remove_prototype_map_deep(objects)
        self.assertIsNot(result, objects)
        self.assertIsInstance(result, list)

    def test_preserves_element_order(self):
        objects = [
            {"id": 1, "value": {"x": 1}},
            {"id": 2, "value": {"x": 2}},
            {"id": 3, "value": {"x": 3}},
        ]
        result = remove_prototype_map_deep(objects)
        self.assertEqual([item["id"] for item in result], [1, 2, 3])

    def test_does_not_mutate_input_objects(self):
        objects = [{"outer": {"__proto__": {"polluted": True}, "ok": 1}}]
        remove_prototype_map_deep(objects)
        self.assertIn("__proto__", objects[0]["outer"])
        self.assertEqual(objects[0]["outer"]["ok"], 1)

    def test_sanitizes_dangerous_keys_nested_inside_arrays(self):
        objects = [
            {
                "items": [
                    {"__proto__": {"polluted": True}, "a": 1},
                    {"constructor": {"polluted": True}, "b": 2},
                ]
            }
        ]
        result = remove_prototype_map_deep(objects)
        self.assertEqual(result[0]["items"], [{"a": 1}, {"b": 2}])

    def test_mixes_sanitized_and_already_safe_objects(self):
        objects = [
            {"clean": {"a": 1}},
            {"dirty": {"__proto__": {"polluted": True}, "b": 2}},
        ]
        result = remove_prototype_map_deep(objects)
        self.assertEqual(result, [{"clean": {"a": 1}}, {"dirty": {"b": 2}}])


if __name__ == "__main__":
    unittest.main()
