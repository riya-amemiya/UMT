import unittest

from src.array import range_array


class TestRangeArray(unittest.TestCase):
    def test_basic_cases(self):
        """Test basic range generation."""
        self.assertEqual(range_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(range_array(2, 10, 2), [2, 4, 6, 8])

    def test_single_argument(self):
        """Test with only start argument (range from 0 to start)."""
        self.assertEqual(range_array(3), [0, 1, 2])
        self.assertEqual(range_array(0), [])

    def test_two_arguments(self):
        """Test with start and end arguments."""
        self.assertEqual(range_array(1, 5), [1, 2, 3, 4])
        self.assertEqual(range_array(0, 3), [0, 1, 2])

    def test_positive_step(self):
        """Test with positive step values."""
        self.assertEqual(range_array(0, 10, 2), [0, 2, 4, 6, 8])
        self.assertEqual(range_array(1, 6, 3), [1, 4])

    def test_negative_step(self):
        """Test with negative step values."""
        self.assertEqual(range_array(5, 0, -1), [5, 4, 3, 2, 1])
        self.assertEqual(range_array(10, 0, -2), [10, 8, 6, 4, 2])

    def test_zero_step(self):
        """Test with zero step (should return empty array)."""
        self.assertEqual(range_array(0, 5, 0), [])
        self.assertEqual(range_array(5, 0, 0), [])

    def test_invalid_range_positive_step(self):
        """Test with invalid range for positive step (start >= end)."""
        self.assertEqual(range_array(5, 5, 1), [])
        self.assertEqual(range_array(10, 5, 1), [])

    def test_invalid_range_negative_step(self):
        """Test with invalid range for negative step (start <= end)."""
        self.assertEqual(range_array(5, 5, -1), [])
        self.assertEqual(range_array(0, 5, -1), [])

    def test_float_values(self):
        """Test with float values."""
        result = range_array(0.5, 3.5, 1)
        self.assertEqual(result, [0.5, 1.5, 2.5])

    def test_negative_start_end(self):
        """Test with negative start and end values."""
        self.assertEqual(range_array(-5, 0, 1), [-5, -4, -3, -2, -1])
        self.assertEqual(range_array(0, -5, -1), [0, -1, -2, -3, -4])

    def test_docstring_examples(self):
        """Test examples from docstring."""
        self.assertEqual(range_array(5), [0, 1, 2, 3, 4])
        self.assertEqual(range_array(2, 10, 2), [2, 4, 6, 8])


if __name__ == "__main__":
    unittest.main()
