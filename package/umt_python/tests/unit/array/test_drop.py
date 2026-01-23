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


if __name__ == "__main__":
    unittest.main()
