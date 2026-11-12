import math
import unittest

from src.math import percentile, quantile


class TestQuantile(unittest.TestCase):
    def test_quartiles(self):
        self.assertEqual(quantile([1, 2, 3, 4, 5], 0.25), 2)
        self.assertEqual(quantile([1, 2, 3, 4, 5], 0.5), 3)
        self.assertEqual(quantile([1, 2, 3, 4, 5], 0.75), 4)

    def test_minimum_and_maximum(self):
        self.assertEqual(quantile([5, 1, 4, 2, 3], 0), 1)
        self.assertEqual(quantile([5, 1, 4, 2, 3], 1), 5)

    def test_interpolation(self):
        self.assertEqual(quantile([1, 2, 3, 4], 0.5), 2.5)
        self.assertEqual(quantile([10, 20], 0.1), 11)

    def test_matches_percentile(self):
        values = [index * 1.5 for index in range(101)]
        self.assertEqual(quantile(values, 0.29), percentile(values, 29))
        self.assertEqual(quantile(values, 0.07), percentile(values, 7))

    def test_empty(self):
        self.assertTrue(math.isnan(quantile([], 0.5)))

    def test_out_of_range(self):
        with self.assertRaises(ValueError):
            quantile([1, 2, 3], 1.5)


if __name__ == "__main__":
    unittest.main()
