import unittest

from src.math import quotient


class TestQuotient(unittest.TestCase):
    def test_basic_cases(self):
        result = quotient(10, 3)
        self.assertEqual(result[0], 3.0)
        self.assertEqual(result[1], 1)

    def test_edge_cases(self):
        result = quotient(9, 3)
        self.assertEqual(result[0], 3.0)
        self.assertEqual(result[1], 0)

    def test_docstring_example(self):
        self.assertEqual(quotient(5, 2), [2.0, 1.0])


if __name__ == "__main__":
    unittest.main()
