import unittest

from src.object import object_merge_deep


class TestObjectMergeDeep(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            object_merge_deep({"a": 1}, {"b": 2}),
            {"a": 1, "b": 2},
        )

    def test_deep_merge(self):
        self.assertEqual(
            object_merge_deep({"a": {"b": 1}}, {"a": {"c": 2}}),
            {"a": {"b": 1, "c": 2}},
        )

    def test_no_sources(self):
        target = {"a": 1}
        self.assertEqual(object_merge_deep(target), target)

    def test_nested_dict_merge(self):
        self.assertEqual(
            object_merge_deep(
                {"a": {"b": {"c": 1}}},
                {"a": {"b": {"d": 2}}},
            ),
            {"a": {"b": {"c": 1, "d": 2}}},
        )

    def test_non_dict_value_overwrite(self):
        self.assertEqual(
            object_merge_deep({"a": {"b": 1}}, {"a": {"b": 2}}),
            {"a": {"b": 2}},
        )
        self.assertEqual(
            object_merge_deep({"a": 1}, {"a": {"b": 2}}),
            {"a": {"b": 2}},
        )
        self.assertEqual(
            object_merge_deep({"a": {"b": 1}}, {"a": 2}),
            {"a": 2},
        )

    def test_multiple_sources(self):
        self.assertEqual(
            object_merge_deep(
                {"a": {"b": 1}},
                {"a": {"c": 2}},
                {"a": {"d": 3}},
            ),
            {"a": {"b": 1, "c": 2, "d": 3}},
        )

    def test_non_dict_target_or_source(self):
        self.assertEqual(
            object_merge_deep([1, 2], {"a": 1}),
            {"a": 1},
        )
        self.assertEqual(
            object_merge_deep({"a": 1}, [1, 2]),
            [1, 2],
        )

    def test_does_not_mutate_original(self):
        original = {"a": {"b": 1}}
        result = object_merge_deep(original, {"a": {"c": 2}})
        self.assertEqual(original, {"a": {"b": 1}})
        self.assertEqual(result, {"a": {"b": 1, "c": 2}})

    def test_docstring_example(self):
        self.assertEqual(
            object_merge_deep({"a": {"b": 1}}, {"a": {"c": 2}}),
            {"a": {"b": 1, "c": 2}},
        )


if __name__ == "__main__":
    unittest.main()
