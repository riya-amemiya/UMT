import unittest

from src.array import get_arrays_diff


class TestGetArraysDiff(unittest.TestCase):
    def test_basic_cases(self):
        result = get_arrays_diff([1, 2, 3], [2, 3, 4])
        self.assertEqual(sorted(result), [1, 4])

    def test_edge_cases(self):
        self.assertEqual(get_arrays_diff([1, 2, 3], [1, 2, 3]), [])
        result = get_arrays_diff([1, 2], [3, 4])
        self.assertEqual(sorted(result), [1, 2, 3, 4])

    def test_docstring_example(self):
        result = get_arrays_diff([1, 2, 3], [2, 3, 4])
        self.assertEqual(sorted(result), [1, 4])


if __name__ == "__main__":
    unittest.main()
