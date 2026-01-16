import unittest

from src.array import first


class TestFirst(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(first([1, 2, 3]), 1)
        self.assertEqual(first(["a", "b", "c"]), "a")

    def test_edge_cases(self):
        self.assertIsNone(first([]))

    def test_docstring_example(self):
        self.assertEqual(first([1, 2, 3]), 1)
        self.assertIsNone(first([]))
        self.assertEqual(first(["a", "b", "c"]), "a")


if __name__ == "__main__":
    unittest.main()
