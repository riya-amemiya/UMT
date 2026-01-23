import unittest

from src.math import math_separator


class TestMathSeparator(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(math_separator(1250), (1000, 250))
        self.assertEqual(math_separator(5), (5, 0))

    def test_single_digit(self):
        self.assertEqual(math_separator(5), (5, 0))
        self.assertEqual(math_separator(9), (9, 0))

    def test_two_digit(self):
        self.assertEqual(math_separator(25), (10, 15))
        self.assertEqual(math_separator(99), (10, 89))

    def test_three_digit(self):
        self.assertEqual(math_separator(125), (100, 25))
        self.assertEqual(math_separator(500), (100, 400))

    def test_invalid_input(self):
        result = math_separator("not a number")
        self.assertEqual(result, (0, 0))

    def test_decimal_input(self):
        result = math_separator(12.5)
        self.assertEqual(result[0], 10.0)

    def test_string_number(self):
        result = math_separator("1250")
        self.assertEqual(result, (1000, 250))

    def test_docstring_example(self):
        self.assertEqual(math_separator(1250), (1000, 250))
        self.assertEqual(math_separator(5), (5, 0))


if __name__ == "__main__":
    unittest.main()
