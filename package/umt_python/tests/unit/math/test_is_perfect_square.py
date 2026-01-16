import unittest

from src.math import is_perfect_square


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


if __name__ == "__main__":
    unittest.main()
