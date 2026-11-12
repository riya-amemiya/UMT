import math
import unittest

from src.math import variance


class TestVariance(unittest.TestCase):
    def test_population_by_default(self):
        self.assertEqual(variance([1, 2, 3]), 0.6666666666666666)
        self.assertEqual(variance([1, 2, 3, 4, 5]), 2)
        self.assertEqual(variance([10, 12, 23, 23, 16, 23, 21, 16]), 24)

    def test_population_when_sample_is_false(self):
        self.assertEqual(variance([1, 2, 3, 4, 5], False), 2)

    def test_sample(self):
        self.assertEqual(variance([1, 2, 3], True), 1)
        self.assertEqual(variance([1, 2, 3, 4, 5], True), 2.5)
        self.assertEqual(
            variance([10, 12, 23, 23, 16, 23, 21, 16], True), 27.428571428571427
        )

    def test_decimal_values(self):
        self.assertEqual(variance([0.1, 0.2, 0.3]), 0.006666666666666666)

    def test_identical_values(self):
        self.assertEqual(variance([5, 5, 5, 5]), 0)
        self.assertEqual(variance([5, 5, 5, 5], True), 0)

    def test_single_value_population(self):
        self.assertEqual(variance([42]), 0)

    def test_single_value_sample_is_nan(self):
        self.assertTrue(math.isnan(variance([42], True)))

    def test_empty(self):
        self.assertEqual(variance([]), 0)
        self.assertEqual(variance([], True), 0)

    def test_returns_float(self):
        self.assertIsInstance(variance([1, 2, 3], True), float)


if __name__ == "__main__":
    unittest.main()
