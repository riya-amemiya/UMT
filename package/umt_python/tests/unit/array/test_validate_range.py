import unittest

from src.array import validate_range
from src.array.validate_range import ValidatedSortRange


class TestValidateRange(unittest.TestCase):
    def test_valid_range(self):
        """Test with valid range within bounds."""
        result = validate_range([1, 2, 3, 4, 5], 0, 4)
        self.assertEqual(result.start_index, 0)
        self.assertEqual(result.end_index, 4)
        self.assertTrue(result.should_sort)

    def test_empty_array(self):
        """Test with empty array."""
        result = validate_range([], 0, 0)
        self.assertEqual(result.start_index, 0)
        self.assertEqual(result.end_index, -1)
        self.assertFalse(result.should_sort)

    def test_negative_start_index(self):
        """Test with negative start index (should clamp to 0)."""
        result = validate_range([1, 2, 3, 4, 5], -1, 10)
        self.assertEqual(result.start_index, 0)
        self.assertEqual(result.end_index, 4)
        self.assertTrue(result.should_sort)

    def test_end_index_beyond_bounds(self):
        """Test with end index beyond array bounds."""
        result = validate_range([1, 2, 3, 4, 5], 0, 100)
        self.assertEqual(result.start_index, 0)
        self.assertEqual(result.end_index, 4)
        self.assertTrue(result.should_sort)

    def test_start_index_beyond_bounds(self):
        """Test with start index beyond array bounds."""
        result = validate_range([1, 2, 3], 5, 10)
        self.assertEqual(result.start_index, 2)
        self.assertEqual(result.end_index, 2)
        self.assertTrue(result.should_sort)

    def test_both_indices_negative(self):
        """Test with both indices negative."""
        result = validate_range([1, 2, 3, 4, 5], -5, -1)
        self.assertEqual(result.start_index, 0)
        self.assertEqual(result.end_index, 0)
        self.assertTrue(result.should_sort)

    def test_partial_range(self):
        """Test with partial range."""
        result = validate_range([1, 2, 3, 4, 5], 1, 3)
        self.assertEqual(result.start_index, 1)
        self.assertEqual(result.end_index, 3)
        self.assertTrue(result.should_sort)

    def test_single_element_range(self):
        """Test with single element range."""
        result = validate_range([1, 2, 3, 4, 5], 2, 2)
        self.assertEqual(result.start_index, 2)
        self.assertEqual(result.end_index, 2)
        self.assertTrue(result.should_sort)

    def test_single_element_array(self):
        """Test with single element array."""
        result = validate_range([1], 0, 0)
        self.assertEqual(result.start_index, 0)
        self.assertEqual(result.end_index, 0)
        self.assertTrue(result.should_sort)

    def test_validated_sort_range_dataclass(self):
        """Test ValidatedSortRange dataclass structure."""
        result = ValidatedSortRange(start_index=1, end_index=3, should_sort=True)
        self.assertEqual(result.start_index, 1)
        self.assertEqual(result.end_index, 3)
        self.assertTrue(result.should_sort)

    def test_docstring_examples(self):
        """Test examples from docstring."""
        result1 = validate_range([1, 2, 3, 4, 5], 0, 4)
        self.assertEqual(
            result1, ValidatedSortRange(start_index=0, end_index=4, should_sort=True)
        )

        result2 = validate_range([1, 2, 3, 4, 5], -1, 10)
        self.assertEqual(
            result2, ValidatedSortRange(start_index=0, end_index=4, should_sort=True)
        )

        result3 = validate_range([], 0, 0)
        self.assertEqual(
            result3, ValidatedSortRange(start_index=0, end_index=-1, should_sort=False)
        )

        result4 = validate_range([1, 2, 3], 5, 10)
        self.assertEqual(
            result4, ValidatedSortRange(start_index=2, end_index=2, should_sort=True)
        )


if __name__ == "__main__":
    unittest.main()
