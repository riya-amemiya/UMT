import unittest
import math
from src.validate import is_deep_equal
from src.validate.is_deep_equal import IsDeepEqualOptions


class TestIsDeepEqualExtra(unittest.TestCase):
    def test_list_strict_order_false_optimized(self):
        options = IsDeepEqualOptions(strict_order=False)

        # Integers (Hashable)
        self.assertTrue(is_deep_equal([1, 2, 3], [3, 2, 1], options))
        self.assertFalse(is_deep_equal([1, 2, 2], [1, 2, 3], options))
        self.assertTrue(is_deep_equal([1, 2, 2], [2, 1, 2], options))

        # Mixed types (Hashable)
        self.assertFalse(is_deep_equal([1], [1.0], options))
        self.assertTrue(is_deep_equal([1, "a"], ["a", 1], options))
        self.assertFalse(is_deep_equal([1, "a"], ["b", 1], options))

        # NaNs (Hashable typically, but tricky)
        nan = float("nan")
        self.assertTrue(is_deep_equal([nan], [nan], options))
        self.assertTrue(is_deep_equal([1, nan], [nan, 1], options))
        self.assertFalse(is_deep_equal([1, nan], [1, 2], options))

        # Multiple NaNs
        self.assertTrue(is_deep_equal([nan, nan], [nan, nan], options))

        # Tuples (Hashable)
        self.assertTrue(is_deep_equal([(1, 2), (3, 4)], [(3, 4), (1, 2)], options))

        # Unhashable fallback (Lists of lists)
        self.assertTrue(is_deep_equal([[1], [2]], [[2], [1]], options))

    def test_bug_reproduction_case_hashable(self):
        # This was the bug pattern: repeated items matching incorrectly against distinct items
        options = IsDeepEqualOptions(strict_order=False)
        # a = [1, 1], b = [2, 3] -> Should be False
        # If bug existed for hashables, it might have returned True if we used the bad logic.
        # But for integers, the old logic used `x == y` which was correct, but `visited` was skipped?
        # Actually `isinstance` check returns early. So integers were SAFE from the visited bug in original code too.
        # But let's verify.
        self.assertFalse(is_deep_equal([1, 1], [2, 3], options))


if __name__ == "__main__":
    unittest.main()
