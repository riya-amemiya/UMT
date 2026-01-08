import unittest

from src.validate import (
    is_array,
    is_dictionary_object,
    is_equal,
    is_not_empty,
    is_number,
    is_string,
    is_value_nan,
)
from src.math import is_perfect_square


class TestIsNumber(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_number(0.1))
        self.assertTrue(is_number(42))
        self.assertTrue(is_number(-5))
        self.assertTrue(is_number(0))

    def test_loose_mode(self):
        self.assertTrue(is_number("0.1"))
        self.assertTrue(is_number("42"))
        self.assertFalse(is_number("0.1", False))

    def test_invalid_values(self):
        self.assertFalse(is_number(None))
        self.assertFalse(is_number([]))
        self.assertFalse(is_number({}))
        self.assertFalse(is_number(True))
        self.assertFalse(is_number("abc"))

    def test_edge_cases(self):
        self.assertFalse(is_number(float("inf")))
        self.assertFalse(is_number(float("nan")))

    def test_docstring_example(self):
        self.assertTrue(is_number(0.1))
        self.assertTrue(is_number("0.1"))
        self.assertFalse(is_number("0.1", False))


class TestIsString(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_string("test"))
        self.assertTrue(is_string(""))
        self.assertTrue(is_string("hello world"))

    def test_invalid_values(self):
        self.assertFalse(is_string(123))
        self.assertFalse(is_string(None))
        self.assertFalse(is_string([]))
        self.assertFalse(is_string({}))

    def test_docstring_example(self):
        self.assertTrue(is_string("test"))
        self.assertFalse(is_string(123))


class TestIsArray(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_array([1, 2, 3]))
        self.assertTrue(is_array([]))
        self.assertTrue(is_array(["a", "b", "c"]))

    def test_invalid_values(self):
        self.assertFalse(is_array({}))
        self.assertFalse(is_array("string"))
        self.assertFalse(is_array(None))
        self.assertFalse(is_array((1, 2, 3)))

    def test_docstring_example(self):
        self.assertTrue(is_array([1, 2, 3]))
        self.assertFalse(is_array({}))


class TestIsDictionaryObject(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_dictionary_object({}))
        self.assertTrue(is_dictionary_object({"a": 1}))

    def test_invalid_values(self):
        self.assertFalse(is_dictionary_object([]))
        self.assertFalse(is_dictionary_object("string"))
        self.assertFalse(is_dictionary_object(None))


class TestIsNotEmpty(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_not_empty({"a": 1}))
        self.assertTrue(is_not_empty({"key": "value"}))

    def test_empty_values(self):
        self.assertFalse(is_not_empty({}))

    def test_docstring_example(self):
        self.assertFalse(is_not_empty({}))
        self.assertTrue(is_not_empty({"a": 1}))


class TestIsEqual(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_equal(1, 1))
        self.assertTrue(is_equal("test", "test"))
        self.assertFalse(is_equal(1, 2))
        self.assertFalse(is_equal("hello", "world"))

    def test_nan_equality(self):
        self.assertTrue(is_equal(float("nan"), float("nan")))

    def test_zero_equality(self):
        self.assertFalse(is_equal(-0.0, 0.0))

    def test_identity(self):
        obj = {"a": 1}
        self.assertTrue(is_equal(obj, obj))
        self.assertFalse(is_equal(obj, {"a": 1}))

    def test_docstring_example(self):
        self.assertTrue(is_equal(1, 1))
        self.assertTrue(is_equal("test", "test"))
        self.assertTrue(is_equal(float("nan"), float("nan")))
        self.assertFalse(is_equal(-0.0, 0.0))


class TestIsPerfectSquare(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_perfect_square(4))
        self.assertTrue(is_perfect_square(9))
        self.assertTrue(is_perfect_square(16))
        self.assertTrue(is_perfect_square(1))

    def test_non_perfect_squares(self):
        self.assertFalse(is_perfect_square(2))
        self.assertFalse(is_perfect_square(3))
        self.assertFalse(is_perfect_square(5))

    def test_edge_cases(self):
        self.assertTrue(is_perfect_square(0))
        self.assertFalse(is_perfect_square(-4))


class TestIsValueNan(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_value_nan(float("nan")))
        self.assertFalse(is_value_nan(1))
        self.assertFalse(is_value_nan(0))

    def test_edge_cases(self):
        self.assertFalse(is_value_nan(float("inf")))
        self.assertFalse(is_value_nan(None))
        self.assertFalse(is_value_nan("nan"))


if __name__ == "__main__":
    unittest.main()
