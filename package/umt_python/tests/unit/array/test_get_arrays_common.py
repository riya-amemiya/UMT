import math
import unittest

from src.array import get_arrays_common


class TestGetArraysCommon(unittest.TestCase):
    def test_single_array(self):
        """Test with only one array (no additional arrays)."""
        result = get_arrays_common([1, 2, 3])
        self.assertEqual(result, [1, 2, 3])

    def test_basic_cases(self):
        """Test basic common element extraction."""
        result = get_arrays_common([1, 2, 3], [2, 3, 4], [2, 5, 3])
        self.assertEqual(result, [2, 3])

    def test_no_common_elements(self):
        """Test when there are no common elements."""
        result = get_arrays_common([1, 2], [3, 4], [5, 6])
        self.assertEqual(result, [])

    def test_all_common_elements(self):
        """Test when all elements are common."""
        result = get_arrays_common([1, 2, 3], [1, 2, 3], [1, 2, 3])
        self.assertEqual(result, [1, 2, 3])

    def test_nan_values_common(self):
        """Test NaN values that are common across all arrays."""
        result = get_arrays_common(
            [1, float("nan"), 3],
            [float("nan"), 2, 3],
            [float("nan"), 5, 3],
        )

        self.assertEqual(len(result), 2)
        self.assertTrue(any(isinstance(x, float) and math.isnan(x) for x in result))
        self.assertIn(3, result)

    def test_nan_values_not_common(self):
        """Test NaN values that are not common across all arrays."""
        result = get_arrays_common(
            [1, float("nan"), 3],
            [1, 2, 3],
            [1, 5, 3],
        )

        self.assertFalse(any(isinstance(x, float) and math.isnan(x) for x in result))
        self.assertEqual(result, [1, 3])

    def test_duplicate_nan_values(self):
        """Test that duplicate NaN values are deduplicated."""
        result = get_arrays_common(
            [float("nan"), float("nan"), 1],
            [float("nan"), 2, 1],
            [float("nan"), 5, 1],
        )

        nan_count = sum(1 for x in result if isinstance(x, float) and math.isnan(x))
        self.assertEqual(nan_count, 1)

    def test_duplicate_values(self):
        """Test that duplicate non-NaN values are deduplicated."""
        result = get_arrays_common([1, 1, 2, 2], [1, 2], [1, 2])
        self.assertEqual(result, [1, 2])

    def test_empty_first_array(self):
        """Test with empty first array."""
        result = get_arrays_common([], [1, 2], [2, 3])
        self.assertEqual(result, [])

    def test_docstring_example(self):
        """Test example from docstring."""
        result = get_arrays_common([1, 2, 3], [2, 3, 4], [2, 5, 3])
        self.assertEqual(result, [2, 3])

    def test_unhashable_elements(self):
        """Test with unhashable elements (lists)."""
        result = get_arrays_common(
            [[1], [2], [3]],
            [[2], [3], [4]],
            [[2], [5], [3]],
        )
        self.assertEqual(result, [[2], [3]])


if __name__ == "__main__":
    unittest.main()
