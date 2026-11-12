import math
import unittest

from src.math import standard_deviation


class TestStandardDeviation(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(standard_deviation([1, 2, 3]), 0.816497, places=6)
        self.assertAlmostEqual(
            standard_deviation([10, 12, 23, 23, 16, 23, 21, 16]), 4.89898, places=5
        )

    def test_edge_cases(self):
        self.assertEqual(standard_deviation([5, 5, 5]), 0)

    def test_empty(self):
        self.assertEqual(standard_deviation([]), 0)
        self.assertEqual(standard_deviation([], True), 0)

    def test_population_when_sample_is_false(self):
        self.assertEqual(
            standard_deviation([1, 2, 3], False), standard_deviation([1, 2, 3])
        )

    def test_sample(self):
        self.assertEqual(standard_deviation([1, 2, 3], True), 1)
        self.assertEqual(
            standard_deviation([2, 4, 4, 4, 5, 5, 7, 9], True), math.sqrt(32 / 7)
        )
        self.assertEqual(standard_deviation([5, 5, 5], True), 0)

    def test_sample_single_value_is_nan(self):
        self.assertTrue(math.isnan(standard_deviation([42], True)))

    def test_docstring_example(self):
        self.assertAlmostEqual(
            round(standard_deviation([1, 2, 3]), 6), 0.816497, places=6
        )


if __name__ == "__main__":
    unittest.main()
