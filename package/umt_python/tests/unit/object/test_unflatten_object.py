import unittest

from src.object import unflatten_object


class TestUnflattenObject(unittest.TestCase):
    def test_rebuilds_nested_object_from_dotted_paths(self):
        self.assertEqual(unflatten_object({"a.b.c": 1}), {"a": {"b": {"c": 1}}})

    def test_merges_multiple_paths_into_shared_parents(self):
        self.assertEqual(
            unflatten_object({"a.b": 1, "a.c": 2}),
            {"a": {"b": 1, "c": 2}},
        )

    def test_supports_custom_separator(self):
        self.assertEqual(unflatten_object({"a/b": 1}, "/"), {"a": {"b": 1}})

    def test_preserves_top_level_scalars(self):
        self.assertEqual(unflatten_object({"a": 1, "b": 2}), {"a": 1, "b": 2})

    def test_returns_empty_for_empty_input(self):
        self.assertEqual(unflatten_object({}), {})


if __name__ == "__main__":
    unittest.main()
