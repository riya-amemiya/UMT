import math
import unittest

from src.array.ordered_unique import ordered_unique


class TestOrderedUnique(unittest.TestCase):
    def test_keeps_first_occurrence_in_order(self):
        self.assertEqual(ordered_unique([3, 1, 3, 2, 1]), [3, 1, 2])

    def test_accepts_any_iterable(self):
        self.assertEqual(ordered_unique(iter([1, 1, 2])), [1, 2])

    def test_empty_input(self):
        self.assertEqual(ordered_unique([]), [])

    def test_keeps_the_first_equal_value_unchanged(self):
        result = ordered_unique([-0.0, 0.0])
        self.assertEqual(len(result), 1)
        self.assertEqual(math.copysign(1.0, result[0]), -1.0)

    def test_same_nan_object_is_deduplicated(self):
        nan = float("nan")
        self.assertEqual(len(ordered_unique([nan, nan])), 1)

    def test_uses_key(self):
        self.assertEqual(
            ordered_unique(["apple", "avocado", "banana"], key=lambda s: s[0]),
            ["apple", "banana"],
        )

    def test_reports_duplicates(self):
        duplicates: list[int] = []
        result = ordered_unique([1, 2, 1, 2, 1], on_duplicate=duplicates.append)
        self.assertEqual(result, [1, 2])
        self.assertEqual(duplicates, [1, 2, 1])

    def test_reports_duplicates_by_key(self):
        duplicates: list[float] = []
        ordered_unique([1.1, 1.9, 2.5], key=math.floor, on_duplicate=duplicates.append)
        self.assertEqual(duplicates, [1.9])

    def test_raises_for_unhashable_values(self):
        with self.assertRaises(TypeError):
            ordered_unique([[1], [1]])


if __name__ == "__main__":
    unittest.main()
