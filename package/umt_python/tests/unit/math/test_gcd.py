import unittest

from src.math import gcd


class TestGcd(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(gcd(12, 18), 6)
        self.assertEqual(gcd(12, 18, 24), 6)

    def test_floating_point(self):
        self.assertEqual(gcd(0.5, 0.25), 0.25)

    def test_edge_cases(self):
        self.assertEqual(gcd(0, 5), 5)
        self.assertEqual(gcd(5, 0), 5)

    def test_docstring_example(self):
        self.assertEqual(gcd(12, 18), 6)
        self.assertEqual(gcd(12, 18, 24), 6)
        self.assertEqual(gcd(0.5, 0.25), 0.25)


if __name__ == "__main__":
    unittest.main()
