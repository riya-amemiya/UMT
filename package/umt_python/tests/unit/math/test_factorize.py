import unittest

from src.math import factorize


class TestFactorize(unittest.TestCase):
    def test_basic_cases(self):
        result = factorize(12)
        self.assertEqual(result, [2, 2, 3])

    def test_edge_cases(self):
        self.assertEqual(factorize(1), [])
        self.assertEqual(factorize(7), [7])

    def test_docstring_example(self):
        self.assertEqual(factorize(12), [2, 2, 3])


if __name__ == "__main__":
    unittest.main()
