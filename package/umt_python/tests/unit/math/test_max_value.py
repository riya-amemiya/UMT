import unittest

from src.math import max_value


class TestMaxValue(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(max_value(1, 2, 3), 3)
        self.assertEqual(max_value(-1, -2, -3), -1)

    def test_edge_cases(self):
        self.assertEqual(max_value(5), 5)
        self.assertEqual(max_value(0, 0, 0), 0)


if __name__ == "__main__":
    unittest.main()
