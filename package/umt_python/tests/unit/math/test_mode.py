import unittest

from src.math import mode


class TestMode(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(mode([1, 2, 2, 3, 3, 3]), [3])
        self.assertEqual(mode([1, 2, 2, 3, 3]), [2, 3])
        self.assertEqual(mode([1, 2, 3]), [1, 2, 3])

    def test_edge_cases(self):
        self.assertEqual(mode([]), [])
        self.assertEqual(mode([5]), [5])

    def test_docstring_example(self):
        self.assertEqual(mode([1, 2, 2, 3, 3, 3]), [3])
        self.assertEqual(mode([1, 2, 2, 3, 3]), [2, 3])
        self.assertEqual(mode([1, 2, 3]), [1, 2, 3])


if __name__ == "__main__":
    unittest.main()
