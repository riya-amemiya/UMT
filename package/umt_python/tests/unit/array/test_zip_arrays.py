import unittest

from src.array import zip_arrays


class TestZipArrays(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(zip_arrays([1, 2], ["a", "b"]), [[1, "a"], [2, "b"]])
        self.assertEqual(zip_arrays([1, 2, 3], ["a", "b"]), [[1, "a"], [2, "b"]])

    def test_edge_cases(self):
        self.assertEqual(zip_arrays(), [])
        self.assertEqual(zip_arrays([1, 2, 3]), [[1], [2], [3]])

    def test_docstring_example(self):
        self.assertEqual(zip_arrays([1, 2], ["a", "b"]), [[1, "a"], [2, "b"]])
        self.assertEqual(zip_arrays([1, 2, 3], ["a", "b"]), [[1, "a"], [2, "b"]])


if __name__ == "__main__":
    unittest.main()
