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


if __name__ == "__main__":
    unittest.main()
