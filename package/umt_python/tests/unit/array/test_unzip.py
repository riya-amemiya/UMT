import unittest

from src.array import unzip, zip_arrays


class TestUnzip(unittest.TestCase):
    def test_splits_tuples(self):
        self.assertEqual(
            unzip([[1, "a"], [2, "b"], [3, "c"]]), [[1, 2, 3], ["a", "b", "c"]]
        )

    def test_is_inverse_of_zip_arrays(self):
        numbers = [1, 2, 3]
        letters = ["a", "b", "c"]
        flags = [True, False, True]
        self.assertEqual(
            unzip(zip_arrays(numbers, letters, flags)), [numbers, letters, flags]
        )

    def test_truncates_to_shortest_tuple(self):
        self.assertEqual(
            unzip([(1, "a", True), (2, "b"), (3, "c", False)]),
            [[1, 2, 3], ["a", "b", "c"]],
        )

    def test_empty_input(self):
        self.assertEqual(unzip([]), [])

    def test_empty_tuple(self):
        self.assertEqual(unzip([[1], []]), [])

    def test_keeps_none(self):
        self.assertEqual(unzip([[None, 1], [2, None]]), [[None, 2], [1, None]])


if __name__ == "__main__":
    unittest.main()
