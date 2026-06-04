import unittest

from src.object import remove_prototype_map


class TestRemovePrototypeMap(unittest.TestCase):
    def test_removes_polluting_properties_from_each_object(self):
        nested = {"keep": True}
        objects = [
            {"__proto__": {"polluted": True}, "a": 1},
            {
                "constructor": {"prototype": {"polluted": True}},
                "b": 2,
                "nested": nested,
            },
            {"prototype": {"polluted": True}, "c": 3},
        ]
        result = remove_prototype_map(objects)
        self.assertEqual(result, [{"a": 1}, {"b": 2, "nested": nested}, {"c": 3}])
        self.assertIsNot(result, objects)
        self.assertIsNot(result[0], objects[0])
        self.assertIs(result[1]["nested"], nested)

    def test_returns_empty_array_for_empty_input(self):
        self.assertEqual(remove_prototype_map([]), [])

    def test_returns_new_list_instance(self):
        objects = [{"a": 1}]
        result = remove_prototype_map(objects)
        self.assertIsNot(result, objects)
        self.assertIsInstance(result, list)

    def test_preserves_element_order(self):
        objects = [{"a": 1}, {"a": 2}, {"a": 3}, {"a": 4}]
        result = remove_prototype_map(objects)
        self.assertEqual([item["a"] for item in result], [1, 2, 3, 4])

    def test_only_sanitizes_top_level_of_each_element(self):
        nested_malicious = {"__proto__": {"polluted": True}, "deep": 1}
        objects = [{"nested": nested_malicious}]
        result = remove_prototype_map(objects)
        self.assertIn("__proto__", result[0]["nested"])
        self.assertIs(result[0]["nested"], nested_malicious)

    def test_does_not_mutate_input_objects(self):
        objects = [
            {"__proto__": {"polluted": True}, "a": 1},
            {"constructor": {"polluted": True}, "b": 2},
        ]
        remove_prototype_map(objects)
        self.assertIn("__proto__", objects[0])
        self.assertIn("constructor", objects[1])


if __name__ == "__main__":
    unittest.main()
