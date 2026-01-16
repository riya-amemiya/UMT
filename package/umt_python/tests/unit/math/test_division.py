import math
import unittest

from src.math import division


class TestDivision(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(division(6, 3), 2.0)
        self.assertAlmostEqual(division(0.1, 0.2), 0.5)

    def test_with_remainder(self):
        result = division(10, 3, False)
        self.assertEqual(result[0], 3)
        self.assertEqual(result[1], 1)

    def test_division_by_zero(self):
        self.assertTrue(math.isnan(division(1, 0)))
        result = division(1, 0, False)
        self.assertTrue(math.isnan(result[0]))
        self.assertTrue(math.isnan(result[1]))

    def test_docstring_example(self):
        self.assertAlmostEqual(division(0.1, 0.2), 0.5)
        result = division(10, 3, False)
        self.assertEqual(result[0], 3)
        self.assertEqual(result[1], 1)


if __name__ == "__main__":
    unittest.main()
