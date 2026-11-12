import unittest

from src.object import object_get


class TestObjectGet(unittest.TestCase):
    def test_reads_top_level_property(self):
        self.assertEqual(object_get({"a": 1}, "a"), 1)

    def test_reads_nested_property_by_dotted_path(self):
        self.assertEqual(object_get({"a": {"b": {"c": 42}}}, "a.b.c"), 42)

    def test_reads_nested_property_by_list_path(self):
        self.assertEqual(object_get({"a": {"b": {"c": 42}}}, ["a", "b", "c"]), 42)

    def test_returns_default_when_segment_missing(self):
        self.assertEqual(object_get({"a": {"b": 1}}, "a.x", "fallback"), "fallback")

    def test_returns_default_when_traversing_through_none(self):
        self.assertEqual(object_get({"a": None}, "a.b", "fallback"), "fallback")

    def test_returns_none_when_no_default_and_missing(self):
        self.assertIsNone(object_get({"a": 1}, "x"))

    def test_returns_explicit_none_value(self):
        self.assertIsNone(object_get({"a": None}, "a", "fallback"))

    def test_handles_none_root(self):
        self.assertEqual(object_get(None, "a.b", 7), 7)

    def test_returns_default_for_non_dict_intermediate(self):
        self.assertEqual(object_get({"a": 1}, "a.b", "fallback"), "fallback")

    def test_docstring_example(self):
        self.assertEqual(object_get({"a": {"b": {"c": 1}}}, "a.b.c"), 1)
        self.assertEqual(object_get({"a": {"b": 1}}, ["a", "x"], 0), 0)
        self.assertEqual(object_get({"a": None}, "a.b", 7), 7)


class TestObjectGetListTraversal(unittest.TestCase):
    def test_reads_list_elements_by_index_segment(self):
        self.assertEqual(object_get({"a": [10, 20]}, "a.1"), 20)
        self.assertEqual(object_get({"a": [{"b": 5}]}, ["a", "0", "b"]), 5)

    def test_returns_default_for_out_of_range_index(self):
        self.assertEqual(object_get({"a": [10, 20]}, "a.2", "fallback"), "fallback")

    def test_returns_default_for_non_canonical_index(self):
        self.assertEqual(object_get({"a": [10, 20]}, "a.01", "fallback"), "fallback")
        self.assertEqual(object_get({"a": [10, 20]}, "a.-1", "fallback"), "fallback")
        self.assertEqual(object_get({"a": [10, 20]}, "a.x", "fallback"), "fallback")
        self.assertEqual(
            object_get({"a": [10, 20]}, "a.\u0661", "fallback"), "fallback"
        )

    def test_returns_default_when_stepping_off_a_primitive(self):
        self.assertEqual(object_get({"a": 1}, "a.b", "fallback"), "fallback")


if __name__ == "__main__":
    unittest.main()
