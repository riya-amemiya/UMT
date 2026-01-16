import unittest

from src.math import factorial


class TestFactorial(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(factorial(5), 120)
        self.assertEqual(factorial(0), 1)
        self.assertEqual(factorial(1), 1)

    def test_edge_cases(self):
        self.assertEqual(factorial(10), 3628800)
        self.assertEqual(factorial(-1), 1)

    def test_docstring_example(self):
        self.assertEqual(factorial(5), 120)
        self.assertEqual(factorial(0), 1)


if __name__ == "__main__":
    unittest.main()
