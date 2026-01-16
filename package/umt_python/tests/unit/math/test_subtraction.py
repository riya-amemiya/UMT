import unittest

from src.math import subtraction


class TestSubtraction(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(subtraction(5, 3), 2)
        self.assertEqual(subtraction(-5, -3), -2)
        self.assertEqual(subtraction(5, -3), 8)

    def test_floating_point(self):
        self.assertAlmostEqual(subtraction(0.3, 0.1), 0.2)
        self.assertAlmostEqual(subtraction(1, 0.1, 0.2), 0.7)

    def test_docstring_example(self):
        self.assertAlmostEqual(subtraction(0.3, 0.1), 0.2)
        self.assertAlmostEqual(subtraction(1, 0.1, 0.2), 0.7)


if __name__ == "__main__":
    unittest.main()
