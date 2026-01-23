import unittest

from src.array import random_select


class TestRandomSelect(unittest.TestCase):
    def test_basic_selection(self):
        """Test basic random selection."""
        arr = [1, 2, 3, 4, 5]
        result = random_select(arr, 2)
        self.assertEqual(len(result), 2)
        for item in result:
            self.assertIn(item, arr)

    def test_no_duplicates_default(self):
        """Test that by default, no duplicates are selected."""
        arr = [1, 2, 3, 4, 5]
        result = random_select(arr, 5)
        self.assertEqual(len(result), 5)
        self.assertEqual(len(set(result)), 5)

    def test_count_exceeds_array_length_no_duplicates(self):
        """Test when count exceeds array length with no duplicates."""
        arr = [1, 2, 3]
        result = random_select(arr, 10)

        self.assertEqual(len(result), 3)
        self.assertEqual(len(set(result)), 3)

    def test_allow_duplicates(self):
        """Test selection with duplicates allowed."""
        arr = [1, 2, 3]
        result = random_select(arr, 10, allow_duplicates=True)
        self.assertEqual(len(result), 10)
        for item in result:
            self.assertIn(item, arr)

    def test_single_element_with_duplicates(self):
        """Test selecting multiple from single element with duplicates."""
        arr = [42]
        result = random_select(arr, 5, allow_duplicates=True)
        self.assertEqual(len(result), 5)
        self.assertEqual(result, [42, 42, 42, 42, 42])

    def test_select_zero(self):
        """Test selecting zero elements."""
        arr = [1, 2, 3, 4, 5]
        result = random_select(arr, 0)
        self.assertEqual(result, [])

    def test_select_from_single_element(self):
        """Test selecting from single element array."""
        arr = [42]
        result = random_select(arr, 1)
        self.assertEqual(result, [42])

    def test_elements_from_original_array(self):
        """Test that all selected elements are from original array."""
        arr = ["a", "b", "c", "d", "e"]
        result = random_select(arr, 3)
        for item in result:
            self.assertIn(item, arr)

    def test_docstring_example(self):
        """Test example from docstring."""
        result = random_select([1, 2, 3, 4, 5], 2)
        self.assertEqual(len(result), 2)


if __name__ == "__main__":
    unittest.main()
