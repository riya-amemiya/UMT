import unittest

from src.array import chunk


class TestChunk(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(
            chunk([1, 2, 3, 4, 5, 6, 7, 8, 9], 3), [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        )
        self.assertEqual(chunk([1, 2, 3, 4, 5], 2), [[1, 2], [3, 4], [5]])

    def test_edge_cases(self):
        self.assertEqual(chunk([], 3), [])
        self.assertEqual(chunk([1, 2, 3], 5), [[1, 2, 3]])
        self.assertEqual(chunk([1, 2, 3], 1), [[1], [2], [3]])

    def test_docstring_example(self):
        self.assertEqual(
            chunk([1, 2, 3, 4, 5, 6, 7, 8, 9], 3), [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
        )
        self.assertEqual(chunk([1, 2, 3, 4, 5], 2), [[1, 2], [3, 4], [5]])

    def test_does_not_mutate_input(self):
        original = [1, 2, 3, 4, 5]
        chunk(original, 2)
        self.assertEqual(original, [1, 2, 3, 4, 5])


if __name__ == "__main__":
    unittest.main()
