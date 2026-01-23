import unittest

from src.array import shuffle_2d_array


class TestShuffle2dArray(unittest.TestCase):
    def test_preserves_row_structure(self):
        """Test that row lengths are preserved after shuffling."""
        arr = [[1, 2], [3, 4, 5], [6]]
        result = shuffle_2d_array(arr)
        self.assertEqual([len(row) for row in result], [2, 3, 1])

    def test_preserves_all_elements(self):
        """Test that all elements are preserved after shuffling."""
        arr = [[1, 2], [3, 4], [5, 6]]
        result = shuffle_2d_array(arr)

        original_flat = sorted([item for row in arr for item in row])
        result_flat = sorted([item for row in result for item in row])
        self.assertEqual(original_flat, result_flat)

    def test_empty_array(self):
        """Test with empty 2D array."""
        result = shuffle_2d_array([])
        self.assertEqual(result, [])

    def test_empty_sub_arrays(self):
        """Test with empty sub-arrays."""
        arr = [[], [1, 2], []]
        result = shuffle_2d_array(arr)
        self.assertEqual([len(row) for row in result], [0, 2, 0])

    def test_single_element_arrays(self):
        """Test with single element arrays."""
        arr = [[1], [2], [3]]
        result = shuffle_2d_array(arr)
        self.assertEqual([len(row) for row in result], [1, 1, 1])

        original_flat = sorted([item for row in arr for item in row])
        result_flat = sorted([item for row in result for item in row])
        self.assertEqual(original_flat, result_flat)

    def test_single_row(self):
        """Test with a single row."""
        arr = [[1, 2, 3, 4, 5]]
        result = shuffle_2d_array(arr)
        self.assertEqual(len(result), 1)
        self.assertEqual(sorted(result[0]), sorted(arr[0]))

    def test_does_not_mutate_original(self):
        """Test that original array is not mutated."""
        arr = [[1, 2], [3, 4]]
        original_copy = [[1, 2], [3, 4]]
        shuffle_2d_array(arr)
        self.assertEqual(arr, original_copy)

    def test_various_types(self):
        """Test with various element types."""
        arr = [["a", "b"], [1, 2], [True, False]]
        result = shuffle_2d_array(arr)
        self.assertEqual([len(row) for row in result], [2, 2, 2])

    def test_docstring_example(self):
        """Test example from docstring."""
        arr = [[1, 2], [3, 4], [5, 6]]
        result = shuffle_2d_array(arr)
        self.assertEqual([len(row) for row in result], [len(row) for row in arr])


if __name__ == "__main__":
    unittest.main()
