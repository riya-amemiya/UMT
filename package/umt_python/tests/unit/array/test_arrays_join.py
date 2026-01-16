import unittest

from src.array import arrays_join


class TestArraysJoin(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(arrays_join([1, 2, 3], [2, 3, 4]), [1, 2, 3, 4])
        self.assertEqual(arrays_join([1, 2], [2, 3], [3, 4]), [1, 2, 3, 4])

    def test_edge_cases(self):
        self.assertEqual(arrays_join([], []), [])
        self.assertEqual(arrays_join([1, 2, 3]), [1, 2, 3])

    def test_docstring_example(self):
        self.assertEqual(arrays_join([1, 2, 3], [2, 3, 4]), [1, 2, 3, 4])
        self.assertEqual(arrays_join([1, 2], [2, 3], [3, 4]), [1, 2, 3, 4])


if __name__ == "__main__":
    unittest.main()
