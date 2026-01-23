import unittest

from src.math import random_int


class TestRandomInt(unittest.TestCase):
    def test_basic_range(self):
        for _ in range(100):
            result = random_int(10)
            self.assertGreaterEqual(result, 0)
            self.assertLessEqual(result, 10)

    def test_min_and_max(self):
        for _ in range(100):
            result = random_int(10, 5)
            self.assertGreaterEqual(result, 5)
            self.assertLessEqual(result, 10)

    def test_same_min_max(self):
        result = random_int(5, 5)
        self.assertEqual(result, 5)

    def test_docstring_example(self):
        self.assertTrue(0 <= random_int(10) <= 10)
        self.assertTrue(5 <= random_int(10, 5) <= 10)


if __name__ == "__main__":
    unittest.main()
