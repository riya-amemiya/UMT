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


if __name__ == "__main__":
    unittest.main()
