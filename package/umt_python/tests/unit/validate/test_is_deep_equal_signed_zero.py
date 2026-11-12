import unittest

from src.validate import is_deep_equal
from src.validate.is_deep_equal import IsDeepEqualOptions


class TestIsDeepEqualSignedZero(unittest.TestCase):
    def test_distinguishes_signed_zeros(self):
        self.assertFalse(is_deep_equal(-0.0, 0.0))
        self.assertTrue(is_deep_equal(0.0, 0.0))
        self.assertTrue(is_deep_equal(-0.0, -0.0))

    def test_distinguishes_signed_zeros_inside_containers(self):
        self.assertFalse(is_deep_equal([0.0], [-0.0]))
        self.assertFalse(is_deep_equal({"a": 0.0}, {"a": -0.0}))
        self.assertTrue(is_deep_equal([-0.0], [-0.0]))

    def test_distinguishes_signed_zeros_without_strict_order(self):
        options = IsDeepEqualOptions(strict_order=False)
        self.assertFalse(is_deep_equal([0.0, 1.0], [1.0, -0.0], options))
        self.assertTrue(is_deep_equal([-0.0, 1.0], [1.0, -0.0], options))
        self.assertTrue(is_deep_equal([0.0, 1.0], [1.0, 0.0], options))

    def test_keeps_zeros_distinct_from_ones_without_strict_order(self):
        options = IsDeepEqualOptions(strict_order=False)
        self.assertFalse(is_deep_equal([0.0], [1.0], options))
        self.assertFalse(is_deep_equal([-0.0], [-1.0], options))
        self.assertFalse(is_deep_equal([0.0, -1.0], [-0.0, 1.0], options))

    def test_treats_nan_as_equal_to_nan(self):
        options = IsDeepEqualOptions(strict_order=False)
        self.assertTrue(is_deep_equal(float("nan"), float("nan")))
        self.assertTrue(
            is_deep_equal([float("nan"), 1.5], [1.5, float("nan")], options)
        )

    def test_compares_other_floats_by_value(self):
        self.assertTrue(is_deep_equal(float("1.5"), float("1.5")))
        self.assertFalse(is_deep_equal(1.5, 2.5))

    def test_compares_integers_strings_and_booleans_by_value(self):
        self.assertTrue(is_deep_equal(int("1000000"), int("1000000")))
        self.assertTrue(is_deep_equal("".join(["a", "b"]), "ab"))
        self.assertTrue(is_deep_equal(True, True))
        self.assertFalse(is_deep_equal(True, False))
        self.assertFalse(is_deep_equal(True, 1))


if __name__ == "__main__":
    unittest.main()
