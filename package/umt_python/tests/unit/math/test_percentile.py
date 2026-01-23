import math
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

    def test_empty_array(self):
        result = percentile([], 50)
        self.assertTrue(math.isnan(result))

    def test_invalid_percentile_below_zero(self):
        with self.assertRaises(ValueError):
            percentile([1, 2, 3], -1)

    def test_invalid_percentile_above_hundred(self):
        with self.assertRaises(ValueError):
            percentile([1, 2, 3], 101)

    def test_docstring_example(self):
        self.assertEqual(percentile([1, 2, 3, 4, 5], 50), 3)
        self.assertEqual(percentile([1, 2, 3, 4, 5], 25), 2)
        self.assertEqual(percentile([1, 2, 3, 4, 5], 75), 4)


if __name__ == "__main__":
    unittest.main()
