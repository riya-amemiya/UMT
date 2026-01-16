import unittest

from src.math import min_value


class TestMinValue(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(min_value(1, 2, 3), 1)
        self.assertEqual(min_value(-1, -2, -3), -3)

    def test_edge_cases(self):
        self.assertEqual(min_value(5), 5)
        self.assertEqual(min_value(0, 0, 0), 0)


if __name__ == "__main__":
    unittest.main()
