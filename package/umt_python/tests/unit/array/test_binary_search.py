import unittest

from src.array import binary_search


class TestBinarySearch(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 3), 2)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 1), 0)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 5), 4)

    def test_not_found(self):
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 6), -1)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 0), -1)

    def test_edge_cases(self):
        self.assertEqual(binary_search([], 1), -1)
        self.assertEqual(binary_search([5], 5), 0)
        self.assertEqual(binary_search([5], 3), -1)

    def test_docstring_example(self):
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 3), 2)
        self.assertEqual(binary_search([1, 2, 3, 4, 5], 6), -1)


if __name__ == "__main__":
    unittest.main()
