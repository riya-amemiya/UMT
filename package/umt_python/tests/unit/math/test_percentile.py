import unittest

from src.math import percentile


class TestPercentile(unittest.TestCase):
    def test_basic_cases(self):
        data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
        self.assertAlmostEqual(percentile(data, 50), 5.5)

    def test_edge_cases(self):
        data = [1, 2, 3, 4, 5]
        self.assertEqual(percentile(data, 0), 1)
        self.assertEqual(percentile(data, 100), 5)


if __name__ == "__main__":
    unittest.main()
