import unittest

from src.array import unique


class TestUnique(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(unique([1, 2, 2, 3, 3, 3]), [1, 2, 3])
        self.assertEqual(unique(["a", "b", "a", "c"]), ["a", "b", "c"])

    def test_edge_cases(self):
        self.assertEqual(unique([]), [])
        self.assertEqual(unique([1, 1, 1, 1]), [1])
        self.assertEqual(unique([1]), [1])

    def test_preserves_order(self):
        self.assertEqual(unique([3, 1, 2, 1, 3, 2]), [3, 1, 2])

    def test_docstring_example(self):
        self.assertEqual(unique([1, 2, 2, 3, 3, 3]), [1, 2, 3])
        self.assertEqual(unique(["a", "b", "a", "c"]), ["a", "b", "c"])


if __name__ == "__main__":
    unittest.main()
