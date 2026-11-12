import unittest

from src.array import drop


class TestDrop(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(drop([1, 2, 3, 4, 5], 2), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "left"), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "right"), [1, 2, 3])

    def test_between_direction(self):
        self.assertEqual(drop([1, 2, 3, 4, 5], 1, "between"), [1, 2, 4, 5])

    def test_edge_cases(self):
        self.assertEqual(drop([1, 2, 3], -1), [1, 2, 3])
        self.assertEqual(drop([1, 2, 3], 5), [])

    def test_docstring_example(self):
        self.assertEqual(drop([1, 2, 3, 4, 5], 2), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "left"), [3, 4, 5])
        self.assertEqual(drop([1, 2, 3, 4, 5], 2, "right"), [1, 2, 3])
        self.assertEqual(drop([1, 2, 3, 4, 5], 1, "between"), [1, 2, 4, 5])

    def test_invalid_direction_is_treated_as_left(self):
        self.assertEqual(drop([1, 2, 3], 1, "invalid"), [2, 3])  # type: ignore[arg-type]

    def test_between_with_even_count(self):
        self.assertEqual(drop([1, 2, 3, 4, 5, 6], 2, "between"), [1, 2, 5, 6])

    def test_zero_count(self):
        self.assertEqual(drop([1, 2, 3], 0, "left"), [1, 2, 3])
        self.assertEqual(drop([1, 2, 3], 0, "right"), [1, 2, 3])
        self.assertEqual(drop([1, 2, 3], 0, "between"), [1, 2, 3])


if __name__ == "__main__":
    unittest.main()
