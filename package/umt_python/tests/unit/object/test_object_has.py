import unittest

from src.object import object_has


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


class TestObjectHasTraversal(unittest.TestCase):
    def test_list_index_segments(self):
        wrapper = {"list": [10, 20, 30]}
        self.assertTrue(object_has(wrapper, "list.0"))
        self.assertTrue(object_has(wrapper, ["list", "2"]))
        self.assertFalse(object_has(wrapper, "list.3"))
        self.assertFalse(object_has(wrapper, "list.push"))

    def test_list_rejects_non_canonical_index(self):
        wrapper = {"list": [10, 20, 30]}
        self.assertFalse(object_has(wrapper, "list.01"))
        self.assertFalse(object_has(wrapper, "list.-1"))
        self.assertFalse(object_has(wrapper, "list.\u0661"))
        self.assertFalse(object_has(wrapper, ["list", ""]))

    def test_traverses_objects_inside_lists(self):
        wrapper = {"list": [{"a": 1}]}
        self.assertTrue(object_has(wrapper, "list.0.a"))
        self.assertFalse(object_has(wrapper, "list.0.b"))

    def test_stepping_off_a_primitive_returns_false(self):
        self.assertFalse(object_has({"a": 1}, "a.b"))
        self.assertFalse(object_has({"a": "hello"}, "a.ell"))
        self.assertFalse(object_has({"a": None}, "a.b"))

    def test_does_not_copy_or_mutate_the_source(self):
        source = {"a": {"b": 1}}
        object_has(source, "a.b")
        self.assertEqual(source, {"a": {"b": 1}})

    def test_empty_list_path_returns_true(self):
        self.assertTrue(object_has({"a": 1}, []))


if __name__ == "__main__":
    unittest.main()
