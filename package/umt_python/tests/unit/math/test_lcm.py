import unittest

from src.math import lcm


class TestLcm(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(lcm(2, 3), 6)
        self.assertEqual(lcm(4, 6), 12)

    def test_edge_cases(self):
        self.assertEqual(lcm(0, 5), 0)
        self.assertEqual(lcm(5, 0), 0)

    def test_docstring_example(self):
        self.assertEqual(lcm(2, 3), 6)
        self.assertEqual(lcm(4, 6), 12)


if __name__ == "__main__":
    unittest.main()
