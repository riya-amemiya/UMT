import math
import unittest

from src.validate import is_perfect_square


class TestIsPerfectSquareFloat(unittest.TestCase):
    def test_integer_valued_floats(self):
        self.assertTrue(is_perfect_square(16.0))
        self.assertTrue(is_perfect_square(0.0))
        self.assertTrue(is_perfect_square(-0.0))
        self.assertFalse(is_perfect_square(20.0))

    def test_non_integer_floats(self):
        self.assertFalse(is_perfect_square(2.25))
        self.assertFalse(is_perfect_square(0.25))

    def test_negative_floats(self):
        self.assertFalse(is_perfect_square(-16.0))
        self.assertFalse(is_perfect_square(-math.inf))

    def test_large_floats_follow_double_precision_square_root(self):
        self.assertTrue(is_perfect_square(1e300))
        self.assertTrue(is_perfect_square(4.000000000000001))

    def test_non_finite_floats(self):
        self.assertTrue(is_perfect_square(math.inf))
        self.assertFalse(is_perfect_square(math.nan))

    def test_integers_stay_exact(self):
        self.assertTrue(is_perfect_square((10**20 + 1) ** 2))
        self.assertFalse(is_perfect_square((10**20 + 1) ** 2 + 1))
        self.assertFalse(is_perfect_square(-4))


if __name__ == "__main__":
    unittest.main()
