import math
import unittest

from src.array import array_sum


class TestArraySum(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(array_sum([1, 2, 3]), 6)
        self.assertEqual(array_sum([1.5, 2.5, 3.0]), 7.0)

    def test_edge_cases(self):
        self.assertEqual(array_sum([]), 0)
        self.assertEqual(array_sum([5]), 5)

    def test_docstring_example(self):
        self.assertEqual(array_sum([1, 2, 3]), 6)
        self.assertEqual(array_sum([1.5, 2.5, 3.0]), 7.0)

    def test_decimal_numbers_without_floating_point_errors(self):
        self.assertEqual(array_sum([0.1, 0.2]), 0.3)
        self.assertEqual(array_sum([0.1, 0.2, 0.3]), 0.6)
        self.assertEqual(array_sum([1.1, 2.2, 3.3]), 6.6)
        self.assertEqual(array_sum([0.1, 0.02, 0.003]), 0.123)
        self.assertEqual(array_sum([0.0001, 0.0002, 0.0003]), 0.0006)

    def test_mixed_integer_and_decimal_numbers(self):
        self.assertEqual(array_sum([1, 2.5, 3.7]), 7.2)
        self.assertEqual(array_sum([-1.5, 2, -3.7]), -3.2)

    def test_integer_valued_floats_use_plain_sum(self):
        self.assertEqual(array_sum([1.0, 2.0, 3.0]), 6.0)
        self.assertEqual(array_sum([5e-324, 5e-324]), 1e-323)

    def test_non_finite_values(self):
        self.assertTrue(math.isnan(array_sum([float("nan"), 1])))
        self.assertTrue(math.isnan(array_sum([float("nan"), 0.5])))
        self.assertEqual(array_sum([float("inf"), 0.5]), float("inf"))

    def test_rounds_scaled_values_exactly(self):
        self.assertEqual(array_sum([450359962737049.7, 0.1]), 450359962737049.8)
        self.assertEqual(array_sum([-0.25, 0.1]), -0.15)


if __name__ == "__main__":
    unittest.main()
