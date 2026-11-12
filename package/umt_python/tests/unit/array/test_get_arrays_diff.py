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

    def test_preserves_insertion_order(self):
        self.assertEqual(
            get_arrays_diff(["pear", "apple", "fig"], ["apple", "kiwi"]),
            ["pear", "fig", "kiwi"],
        )
        self.assertEqual(get_arrays_diff([3, 1, 2], [9, 1]), [3, 2, 9])

    def test_single_array_is_deduplicated(self):
        self.assertEqual(get_arrays_diff([2, 1, 2]), [2, 1])

    def test_value_repeated_after_removal_stays_removed(self):
        self.assertEqual(get_arrays_diff([1], [2, 2, 2]), [1])
        self.assertEqual(get_arrays_diff([1, 2], [2], [2, 3]), [1, 3])


if __name__ == "__main__":
    unittest.main()
