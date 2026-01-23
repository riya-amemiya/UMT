import unittest

from src.validate import is_deep_equal
from src.validate.is_deep_equal import IsDeepEqualOptions


class TestIsDeepEqual(unittest.TestCase):
    def test_basic_identity(self):
        obj = {"a": 1}
        self.assertTrue(is_deep_equal(obj, obj))

    def test_none_values(self):
        self.assertTrue(is_deep_equal(None, None))
        self.assertFalse(is_deep_equal(None, 1))
        self.assertFalse(is_deep_equal(1, None))

    def test_different_types(self):
        self.assertFalse(is_deep_equal(1, "1"))
        self.assertFalse(is_deep_equal([1], (1,)))
        self.assertFalse(is_deep_equal({"a": 1}, [("a", 1)]))

    def test_primitive_values(self):
        self.assertTrue(is_deep_equal(1, 1))
        self.assertTrue(is_deep_equal("test", "test"))
        self.assertTrue(is_deep_equal(True, True))
        self.assertFalse(is_deep_equal(1, 2))
        self.assertFalse(is_deep_equal("a", "b"))

    def test_nan_values(self):
        self.assertTrue(is_deep_equal(float("nan"), float("nan")))
        self.assertFalse(is_deep_equal(float("nan"), 1.0))

    def test_float_equality(self):
        self.assertTrue(is_deep_equal(1.5, 1.5))
        self.assertFalse(is_deep_equal(1.5, 1.6))

    def test_circular_reference(self):
        list_a = [1, 2]
        list_a.append(list_a)
        list_b = [1, 2]
        list_b.append(list_b)
        self.assertTrue(is_deep_equal(list_a, list_b))

    def test_list_equal_strict_order(self):
        self.assertTrue(is_deep_equal([1, 2, 3], [1, 2, 3]))
        self.assertFalse(is_deep_equal([1, 2, 3], [3, 2, 1]))
        self.assertFalse(is_deep_equal([1, 2], [1, 2, 3]))

    def test_list_equal_non_strict_order(self):
        options = IsDeepEqualOptions(strict_order=False)
        self.assertTrue(is_deep_equal([1, 2, 3], [3, 2, 1], options))
        self.assertTrue(is_deep_equal([1, 2, 3], [2, 1, 3], options))
        self.assertFalse(is_deep_equal([1, 2, 3], [1, 2, 4], options))
        self.assertFalse(is_deep_equal([1, 2], [1, 2, 3], options))

    def test_list_nested_non_strict_order(self):
        options = IsDeepEqualOptions(strict_order=False)
        self.assertTrue(
            is_deep_equal(
                [{"a": 1}, {"b": 2}],
                [{"b": 2}, {"a": 1}],
                options,
            )
        )

    def test_set_equal(self):
        self.assertTrue(is_deep_equal({1, 2, 3}, {3, 2, 1}))
        self.assertTrue(is_deep_equal({1, 2}, {1, 2}))
        self.assertFalse(is_deep_equal({1, 2}, {1, 2, 3}))
        self.assertFalse(is_deep_equal({1, 2, 3}, {1, 2, 4}))

    def test_dict_equal(self):
        self.assertTrue(is_deep_equal({"a": 1, "b": 2}, {"b": 2, "a": 1}))
        self.assertTrue(is_deep_equal({"a": {"b": 1}}, {"a": {"b": 1}}))
        self.assertFalse(is_deep_equal({"a": 1}, {"a": 2}))
        self.assertFalse(is_deep_equal({"a": 1}, {"b": 1}))
        self.assertFalse(is_deep_equal({"a": 1}, {"a": 1, "b": 2}))

    def test_nested_dict(self):
        self.assertTrue(
            is_deep_equal(
                {"a": {"b": {"c": 1}}},
                {"a": {"b": {"c": 1}}},
            )
        )
        self.assertFalse(
            is_deep_equal(
                {"a": {"b": {"c": 1}}},
                {"a": {"b": {"c": 2}}},
            )
        )

    def test_bytes_equal(self):
        self.assertTrue(is_deep_equal(b"hello", b"hello"))
        self.assertFalse(is_deep_equal(b"hello", b"world"))
        self.assertTrue(is_deep_equal(bytearray(b"test"), bytearray(b"test")))
        self.assertFalse(is_deep_equal(bytearray(b"test"), bytearray(b"diff")))

    def test_fallback_equality(self):
        class Point:
            def __init__(self, x, y):
                self.x = x
                self.y = y

            def __eq__(self, other):
                if isinstance(other, Point):
                    return self.x == other.x and self.y == other.y
                return False

        p1 = Point(1, 2)
        p2 = Point(1, 2)
        p3 = Point(3, 4)
        self.assertTrue(is_deep_equal(p1, p2))
        self.assertFalse(is_deep_equal(p1, p3))

    def test_default_options(self):
        self.assertTrue(is_deep_equal({"a": 1}, {"a": 1}))

    def test_docstring_examples(self):
        self.assertTrue(is_deep_equal({"a": 1, "b": [2, 3]}, {"b": [2, 3], "a": 1}))
        self.assertFalse(is_deep_equal([1, 2, 3], [3, 2, 1]))
        self.assertTrue(
            is_deep_equal([1, 2, 3], [3, 2, 1], IsDeepEqualOptions(strict_order=False))
        )
        self.assertTrue(is_deep_equal({1, 2}, {2, 1}))


if __name__ == "__main__":
    unittest.main()
