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

    def test_docstring_example(self):
        self.assertAlmostEqual(
            round(standard_deviation([1, 2, 3]), 6), 0.816497, places=6
        )


if __name__ == "__main__":
    unittest.main()
