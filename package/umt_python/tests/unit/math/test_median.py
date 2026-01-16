import unittest

from src.math import median


class TestMedian(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(median([1, 3, 3, 6, 7, 8, 9]), 6)
        self.assertEqual(median([1, 2, 3, 4]), 2.5)

    def test_edge_cases(self):
        self.assertEqual(median([5]), 5)
        self.assertEqual(median([1, 2]), 1.5)

    def test_docstring_example(self):
        self.assertEqual(median([1, 3, 3, 6, 7, 8, 9]), 6)
        self.assertEqual(median([1, 2, 3, 4]), 2.5)


if __name__ == "__main__":
    unittest.main()
