import unittest

from src.validate import is_equal


class TestIsEqualValueSemantics(unittest.TestCase):
    def test_equal_integers_built_separately_are_equal(self):
        self.assertTrue(is_equal(int("1000000"), int("1000000")))
        self.assertTrue(is_equal(2**80, 2**80))
        self.assertFalse(is_equal(int("1000000"), int("1000001")))

    def test_equal_strings_built_separately_are_equal(self):
        self.assertTrue(is_equal("".join(["te", "st"]), "test"))
        self.assertFalse(is_equal("".join(["te", "st"]), "tesT"))

    def test_equal_floats_built_separately_are_equal(self):
        self.assertTrue(is_equal(float("1.5"), float("1.5")))
        self.assertFalse(is_equal(float("1.5"), float("1.6")))

    def test_same_signed_zeros_are_equal(self):
        self.assertTrue(is_equal(0.0, 0.0))
        self.assertTrue(is_equal(-0.0, -0.0))
        self.assertFalse(is_equal(0.0, -0.0))

    def test_nan_is_only_equal_to_nan(self):
        nan = float("nan")
        self.assertTrue(is_equal(nan, nan))
        self.assertTrue(is_equal(float("nan"), float("nan")))
        self.assertFalse(is_equal(float("nan"), 1.0))

    def test_booleans_are_distinct_from_integers(self):
        self.assertTrue(is_equal(True, True))
        self.assertFalse(is_equal(True, 1))
        self.assertFalse(is_equal(0, False))
        self.assertFalse(is_equal(True, False))

    def test_integers_are_distinct_from_floats(self):
        self.assertFalse(is_equal(1, 1.0))
        self.assertFalse(is_equal(1.0, 1))

    def test_none_is_only_equal_to_none(self):
        self.assertTrue(is_equal(None, None))
        self.assertFalse(is_equal(None, 0))
        self.assertFalse(is_equal("", None))

    def test_different_types_are_not_equal(self):
        self.assertFalse(is_equal(1, "1"))
        self.assertFalse(is_equal("1", 1))

    def test_containers_compare_by_identity(self):
        items = [1, 2, 3]
        self.assertTrue(is_equal(items, items))
        self.assertFalse(is_equal(items, [1, 2, 3]))
        self.assertFalse(is_equal((1, 2), tuple([1, 2])))


if __name__ == "__main__":
    unittest.main()
