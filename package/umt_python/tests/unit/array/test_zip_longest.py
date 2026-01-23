import unittest

from src.array import zip_longest


class TestZipLongest(unittest.TestCase):
    def test_empty_input(self):
        """Test with no arrays provided."""
        result = zip_longest()
        self.assertEqual(result, [])

    def test_basic_cases(self):
        """Test basic zip_longest behavior."""
        result = zip_longest([1, 2], ["a"])
        self.assertEqual(result, [[1, "a"], [2, None]])

        result = zip_longest([1], ["a", "b"])
        self.assertEqual(result, [[1, "a"], [None, "b"]])

    def test_equal_length_arrays(self):
        """Test with arrays of equal length."""
        result = zip_longest([1, 2, 3], ["a", "b", "c"])
        self.assertEqual(result, [[1, "a"], [2, "b"], [3, "c"]])

    def test_three_arrays(self):
        """Test with three arrays of different lengths."""
        result = zip_longest([1, 2], ["a", "b", "c"], [True])
        expected = [[1, "a", True], [2, "b", None], [None, "c", None]]
        self.assertEqual(result, expected)

    def test_single_array(self):
        """Test with a single array."""
        result = zip_longest([1, 2, 3])
        self.assertEqual(result, [[1], [2], [3]])

    def test_empty_array_in_input(self):
        """Test with empty array in input."""
        result = zip_longest([], [1, 2])
        self.assertEqual(result, [[None, 1], [None, 2]])

    def test_all_empty_arrays(self):
        """Test with all empty arrays."""
        result = zip_longest([], [])
        self.assertEqual(result, [])

    def test_docstring_examples(self):
        """Test examples from docstring."""
        self.assertEqual(zip_longest([1, 2], ["a"]), [[1, "a"], [2, None]])
        self.assertEqual(zip_longest([1], ["a", "b"]), [[1, "a"], [None, "b"]])


if __name__ == "__main__":
    unittest.main()
