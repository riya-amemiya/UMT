import unittest

from src.math import average


class TestAverage(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(average([1, 2, 3]), 2)
        self.assertEqual(average([10, 20]), 15)

    def test_edge_cases(self):
        self.assertEqual(average([]), 0)
        self.assertEqual(average([5]), 5)

    def test_docstring_example(self):
        self.assertEqual(average([1, 2, 3]), 2)
        self.assertEqual(average([10, 20]), 15)
        self.assertEqual(average([]), 0)


if __name__ == "__main__":
    unittest.main()
