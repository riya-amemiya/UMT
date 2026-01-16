import unittest

from src.array import pop


class TestPop(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(pop([1, 2, 3]), 3)
        self.assertEqual(pop(["a", "b"]), "b")

    def test_edge_cases(self):
        self.assertIsNone(pop([]))

    def test_docstring_example(self):
        self.assertEqual(pop([1, 2, 3]), 3)
        self.assertIsNone(pop([]))
        self.assertEqual(pop(["a", "b"]), "b")


if __name__ == "__main__":
    unittest.main()
