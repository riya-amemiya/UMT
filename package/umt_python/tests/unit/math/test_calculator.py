import unittest

from src.math import calculator


class TestCalculator(unittest.TestCase):
    def test_addition(self):
        self.assertEqual(calculator("1+"), "1+")
        self.assertEqual(calculator("1+1"), "2")
        self.assertEqual(calculator("1+1+1"), "3")
        self.assertEqual(calculator("1+1+1+1"), "4")
        self.assertEqual(calculator("1+1+1+1+1"), "5")

    def test_subtraction(self):
        self.assertEqual(calculator("1-"), "1-")
        self.assertEqual(calculator("1-1"), "0")
        self.assertEqual(calculator("1-1-1"), "-1")
        self.assertEqual(calculator("1-1-1-1"), "-2")
        self.assertEqual(calculator("1-1-1-1-1"), "-3")

    def test_multiplication(self):
        self.assertEqual(calculator("2*"), "2*")
        self.assertEqual(calculator("2*2"), "4")
        self.assertEqual(calculator("2*2*2"), "8")
        self.assertEqual(calculator("2*2*2*2"), "16")
        self.assertEqual(calculator("2*2*2*2*2"), "32")

    def test_division(self):
        self.assertEqual(calculator("2/"), "2/")
        self.assertEqual(calculator("2/2"), "1")
        self.assertEqual(calculator("2/2/2"), "0.5")
        self.assertEqual(calculator("2/2/2/2"), "0.25")
        self.assertEqual(calculator("2/2/2/2/2"), "0.125")

    def test_exponentiation(self):
        self.assertEqual(calculator("2^"), "2^")
        self.assertEqual(calculator("2^2"), "4")
        self.assertEqual(calculator("2^2^2"), "16")
        self.assertEqual(calculator("2^2^2^2"), "65536")
        self.assertEqual(calculator("2^2^2^2^2"), "Infinity")
        self.assertEqual(calculator("3^4"), "81")
        self.assertEqual(calculator("3^4^2"), "43046721")

    def test_parentheses(self):
        self.assertEqual(calculator("(1+1)"), "2")
        self.assertEqual(calculator("(1"), "(1")
        self.assertEqual(calculator("(1+"), "(1+")
        self.assertEqual(calculator("1)"), "1)")
        self.assertEqual(calculator("(1)"), "(1)")
        self.assertEqual(calculator("(1+1)+(1+1)"), "4")

    def test_equations(self):
        self.assertEqual(calculator("2x=(1+1)+(1+1)+(1+1)"), "3")

    def test_variables_and_currency(self):
        self.assertEqual(calculator("$10*2", {"$": 100}), "2000")
        self.assertEqual(calculator("2*$10", {"$": 100}), "2000")

    def test_docstring_examples(self):
        self.assertEqual(calculator("1+2"), "3")
        self.assertEqual(calculator("(2+3)*4"), "20")
        self.assertEqual(calculator("x=5"), "5")


if __name__ == "__main__":
    unittest.main()
