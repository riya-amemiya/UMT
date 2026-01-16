import unittest

from src.math import addition


class TestAddition(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(addition(2, 3), 5)
        self.assertEqual(addition(-2, -3), -5)
        self.assertEqual(addition(2, -3), -1)
        self.assertEqual(addition(1, 2, 3), 6)
        self.assertEqual(addition(-1, -2, -3), -6)
        self.assertEqual(addition(2, -3, 1), 0)

    def test_floating_point(self):
        self.assertAlmostEqual(addition(0.1, 0.2), 0.3)
        self.assertAlmostEqual(addition(-0.1, -0.2), -0.3)
        self.assertAlmostEqual(addition(0.1, -0.2), -0.1)
        self.assertAlmostEqual(addition(2, 0.3), 2.3)
        self.assertAlmostEqual(addition(-2, -0.3), -2.3)
        self.assertAlmostEqual(addition(2, -0.3), 1.7)
        self.assertAlmostEqual(addition(-2, 0.3), -1.7)
        self.assertAlmostEqual(addition(0.1, 0.2, 0.3), 0.6)
        self.assertAlmostEqual(addition(-2, 0.5, 1.5), 0)

    def test_docstring_example(self):
        self.assertAlmostEqual(addition(0.1, 0.2), 0.3)
        self.assertEqual(addition(1, 2, 3), 6)


if __name__ == "__main__":
    unittest.main()
