import unittest
from typing import cast

from src.object import object_update


class TestObjectUpdate(unittest.TestCase):
    def test_updates_top_level_property(self):
        target = {"a": 1}
        object_update(target, "a", lambda value: cast("int", value) + 1)
        self.assertEqual(target, {"a": 2})

    def test_updates_nested_property(self):
        target = {"a": {"b": 1}}
        object_update(target, "a.b", lambda value: cast("int", value) * 10)
        self.assertEqual(target, {"a": {"b": 10}})

    def test_accepts_list_path(self):
        target = {"a": {"b": "x"}}
        object_update(target, ["a", "b"], lambda value: f"{value}y")
        self.assertEqual(target, {"a": {"b": "xy"}})

    def test_passes_none_and_creates_intermediates_when_missing(self):
        target: dict[str, object] = {}
        received: list[object] = []

        def updater(value: object) -> object:
            received.append(value)
            return 0

        object_update(target, "a.b", updater)
        self.assertEqual(received, [None])
        self.assertEqual(target, {"a": {"b": 0}})

    def test_returns_same_object_reference(self):
        target = {"a": 1}
        self.assertIs(object_update(target, "a", lambda _value: 2), target)

    def test_does_not_call_updater_for_empty_path_list(self):
        target = {"a": 1}
        received: list[object] = []
        result = object_update(target, [], lambda value: received.append(value))
        self.assertIs(result, target)
        self.assertEqual(received, [])
        self.assertEqual(target, {"a": 1})

    def test_replaces_non_object_intermediate_values(self):
        target: dict[str, object] = {"a": 5}
        object_update(target, "a.b", lambda _value: 1)
        self.assertEqual(target, {"a": {"b": 1}})


if __name__ == "__main__":
    unittest.main()
