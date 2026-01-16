import unittest

from src.math import get_decimal_length


class TestGetDecimalLength(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(get_decimal_length(1.234), 3)
        self.assertEqual(get_decimal_length(10), 0)
        self.assertEqual(get_decimal_length(0.1), 1)

    def test_edge_cases(self):
        self.assertEqual(get_decimal_length(0), 0)
        self.assertEqual(get_decimal_length(100), 0)

    def test_docstring_example(self):
        self.assertEqual(get_decimal_length(1.23), 2)
        self.assertEqual(get_decimal_length(100), 0)


if __name__ == "__main__":
    unittest.main()
