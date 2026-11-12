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


class TestObjectMergeDeepSources(unittest.TestCase):
    def test_no_sources_returns_the_target_itself(self):
        target = {"a": 1}
        self.assertIs(object_merge_deep(target), target)

    def test_many_sources_do_not_mutate_inputs(self):
        target = {"a": {"x": 1}}
        first = {"a": {"y": 2}}
        second = {"a": {"z": 3}, "b": 4}
        result = object_merge_deep(target, first, second)
        self.assertEqual(result, {"a": {"x": 1, "y": 2, "z": 3}, "b": 4})
        self.assertEqual(target, {"a": {"x": 1}})
        self.assertEqual(first, {"a": {"y": 2}})
        self.assertEqual(second, {"a": {"z": 3}, "b": 4})

    def test_non_plain_source_replaces_the_accumulated_result(self):
        self.assertEqual(object_merge_deep({"a": 1}, 5), 5)
        self.assertEqual(object_merge_deep({"a": 1}, 5, {"b": 2}), {"b": 2})

    def test_source_after_non_plain_source_is_not_mutated(self):
        replacement = {"b": 2}
        result = object_merge_deep({"a": 1}, 5, replacement, {"c": 3})
        self.assertEqual(result, {"b": 2, "c": 3})
        self.assertEqual(replacement, {"b": 2})

    def test_non_plain_target_is_replaced_by_source(self):
        self.assertEqual(object_merge_deep(5, {"a": 1}), {"a": 1})


if __name__ == "__main__":
    unittest.main()
