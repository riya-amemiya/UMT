import math
import unittest

from src.math import correlation_coefficient


class TestCorrelationCoefficient(unittest.TestCase):
    def test_basic_cases(self):
        x_values = [1, 2, 3, 4, 5]
        y_values = [2, 4, 6, 8, 10]
        self.assertAlmostEqual(correlation_coefficient(x_values, y_values), 1.0)

    def test_negative_correlation(self):
        x_values = [1, 2, 3, 4, 5]
        y_values = [10, 8, 6, 4, 2]
        self.assertAlmostEqual(correlation_coefficient(x_values, y_values), -1.0)

    def test_different_lengths_raises_error(self):
        with self.assertRaises(ValueError):
            correlation_coefficient([1, 2, 3], [1, 2])

    def test_empty_arrays(self):
        result = correlation_coefficient([], [])
        self.assertTrue(math.isnan(result))

    def test_single_element_arrays(self):
        result = correlation_coefficient([1], [2])
        self.assertTrue(math.isnan(result))

    def test_constant_values(self):
        result = correlation_coefficient([5, 5, 5], [5, 5, 5])
        self.assertTrue(math.isnan(result))

    def test_docstring_example(self):
        self.assertEqual(
            correlation_coefficient([1, 2, 3, 4, 5], [2, 4, 6, 8, 10]), 1.0
        )
        self.assertEqual(
            correlation_coefficient([1, 2, 3, 4, 5], [5, 4, 3, 2, 1]), -1.0
        )


if __name__ == "__main__":
    unittest.main()
