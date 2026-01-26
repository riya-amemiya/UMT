import unittest

from src.math import multiples


class TestMultiples(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(multiples(2, 5), [2, 4, 6, 8, 10])

    def test_edge_cases(self):
        self.assertEqual(multiples(3, 3), [3, 6, 9])


if __name__ == "__main__":
    unittest.main()
