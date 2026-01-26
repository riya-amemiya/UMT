import unittest

from src.object import object_pick


class TestObjectPick(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            object_pick({"id": 1, "name": "Alice", "age": 30}, "id", "name"),
            {"id": 1, "name": "Alice"},
        )

    def test_missing_keys(self):
        self.assertEqual(
            object_pick({"a": 1, "b": 2}, "a", "c"),
            {"a": 1},
        )

    def test_edge_cases(self):
        self.assertEqual(object_pick({}, "a"), {})
        self.assertEqual(object_pick({"a": 1, "b": 2}), {})

    def test_docstring_example(self):
        self.assertEqual(
            object_pick({"id": 1, "name": "Alice", "age": 30}, "id", "name"),
            {"id": 1, "name": "Alice"},
        )


if __name__ == "__main__":
    unittest.main()
