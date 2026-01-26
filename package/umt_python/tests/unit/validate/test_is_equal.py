import unittest

from src.validate import is_equal


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


if __name__ == "__main__":
    unittest.main()
