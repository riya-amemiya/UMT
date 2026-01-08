import unittest

from src.object import (
    object_has,
    object_is_empty,
    object_merge,
    object_omit,
    object_pick,
)


class TestObjectHas(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(object_has({"a": {"b": 1}}, "a.b"))
        self.assertTrue(object_has({"a": {"b": 1}}, ["a", "b"]))
        self.assertFalse(object_has({"a": {"b": 1}}, "a.c"))

    def test_single_level(self):
        self.assertTrue(object_has({"a": 1}, "a"))
        self.assertFalse(object_has({"a": 1}, "b"))

    def test_nested_path(self):
        obj = {"a": {"b": {"c": 1}}}
        self.assertTrue(object_has(obj, "a.b.c"))
        self.assertFalse(object_has(obj, "a.b.d"))

    def test_edge_cases(self):
        self.assertFalse(object_has({}, "a"))

    def test_docstring_example(self):
        self.assertTrue(object_has({"a": {"b": 1}}, "a.b"))
        self.assertTrue(object_has({"a": {"b": 1}}, ["a", "b"]))
        self.assertFalse(object_has({"a": {"b": 1}}, "a.c"))


class TestObjectIsEmpty(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(object_is_empty({}))
        self.assertFalse(object_is_empty({"a": 1}))

    def test_edge_cases(self):
        self.assertFalse(object_is_empty({"key": None}))
        self.assertFalse(object_is_empty({"key": ""}))

    def test_docstring_example(self):
        self.assertTrue(object_is_empty({}))
        self.assertFalse(object_is_empty({"a": 1}))


class TestObjectMerge(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            object_merge({"a": 1}, {"b": 2}, {"c": 3}),
            {"a": 1, "b": 2, "c": 3},
        )

    def test_overwrite(self):
        self.assertEqual(
            object_merge({"a": 1}, {"a": 2}),
            {"a": 2},
        )

    def test_edge_cases(self):
        self.assertEqual(object_merge({}, {}), {})
        self.assertEqual(object_merge({"a": 1}), {"a": 1})

    def test_does_not_mutate_original(self):
        original = {"a": 1}
        result = object_merge(original, {"b": 2})
        self.assertEqual(original, {"a": 1})
        self.assertEqual(result, {"a": 1, "b": 2})

    def test_docstring_example(self):
        self.assertEqual(
            object_merge({"a": 1}, {"b": 2}, {"c": 3}),
            {"a": 1, "b": 2, "c": 3},
        )


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


class TestObjectOmit(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            object_omit({"a": 1, "b": 2, "c": 3}, "b", "c"),
            {"a": 1},
        )

    def test_missing_keys(self):
        self.assertEqual(
            object_omit({"a": 1, "b": 2}, "c"),
            {"a": 1, "b": 2},
        )

    def test_edge_cases(self):
        self.assertEqual(object_omit({}, "a"), {})
        self.assertEqual(object_omit({"a": 1, "b": 2}), {"a": 1, "b": 2})

    def test_does_not_mutate_original(self):
        original = {"a": 1, "b": 2}
        result = object_omit(original, "b")
        self.assertEqual(original, {"a": 1, "b": 2})
        self.assertEqual(result, {"a": 1})

    def test_docstring_example(self):
        self.assertEqual(
            object_omit({"a": 1, "b": 2, "c": 3}, "b", "c"),
            {"a": 1},
        )


if __name__ == "__main__":
    unittest.main()
