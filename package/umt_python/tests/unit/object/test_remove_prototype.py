import unittest

from src.object import remove_prototype


class TestRemovePrototype(unittest.TestCase):
    def test_removes_prototype_polluting_properties(self):
        source = {
            "__proto__": {"polluted": True},
            "constructor": {"prototype": {"polluted": True}},
            "prototype": {"polluted": True},
            "a": 1,
        }
        result = remove_prototype(source)
        self.assertEqual(result, {"a": 1})
        self.assertNotIn("__proto__", result)
        self.assertNotIn("constructor", result)
        self.assertNotIn("prototype", result)

    def test_preserves_other_properties(self):
        source = {
            "a": 1,
            "b": "test",
            "c": True,
            "d": [1, 2, 3],
            "e": {"nested": True},
        }
        self.assertEqual(
            remove_prototype(source),
            {"a": 1, "b": "test", "c": True, "d": [1, 2, 3], "e": {"nested": True}},
        )

    def test_does_not_modify_original(self):
        source = {"__proto__": {"polluted": True}, "a": 1}
        remove_prototype(source)
        self.assertIn("__proto__", source)
        self.assertEqual(source["a"], 1)

    def test_performs_shallow_copy(self):
        nested = {"b": 2}
        result = remove_prototype({"a": nested})
        self.assertIs(result["a"], nested)

    def test_handles_empty_object(self):
        result = remove_prototype({})
        self.assertEqual(result, {})
        self.assertEqual(len(result), 0)

    def test_only_removes_dangerous_keys_at_top_level(self):
        safe = {"__proto__": {"polluted": True}, "value": 1}
        result = remove_prototype({"safe": safe})
        self.assertIn("__proto__", result["safe"])
        self.assertIs(result["safe"], safe)

    def test_does_not_share_references_between_calls(self):
        source = {"a": 1}
        first = remove_prototype(source)
        second = remove_prototype(source)
        self.assertIsNot(first, second)

    def test_mixed_dangerous_and_safe_keys(self):
        source = {
            "__proto__": 1,
            "a": 2,
            "constructor": 3,
            "b": 4,
            "prototype": 5,
            "c": 6,
        }
        result = remove_prototype(source)
        self.assertEqual(list(result.keys()), ["a", "b", "c"])
        self.assertEqual(result, {"a": 2, "b": 4, "c": 6})


if __name__ == "__main__":
    unittest.main()
