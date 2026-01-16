import unittest

from src.math import multiplication


class TestMultiplication(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(multiplication(2, 3), 6)
        self.assertEqual(multiplication(2, 3, 4), 24)
        self.assertEqual(multiplication(-2, 3), -6)

    def test_floating_point(self):
        self.assertAlmostEqual(multiplication(0.1, 0.2), 0.02)
        self.assertAlmostEqual(multiplication(0.1, 0.2, 0.3), 0.006)

    def test_docstring_example(self):
        self.assertAlmostEqual(multiplication(0.1, 0.2, 0.3), 0.006)
        self.assertEqual(multiplication(2, 3, 4), 24)


if __name__ == "__main__":
    unittest.main()
