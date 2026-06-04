import unittest

from src.math import calculator_core


class TestCalculatorCore(unittest.TestCase):
    def test_simple_addition(self):
        self.assertEqual(calculator_core("1+1"), "2")
        self.assertEqual(calculator_core("10+20"), "30")
        self.assertEqual(calculator_core("0+0"), "0")

    def test_simple_subtraction(self):
        self.assertEqual(calculator_core("5-3"), "2")
        self.assertEqual(calculator_core("10-20"), "-10")
        self.assertEqual(calculator_core("0-0"), "0")

    def test_simple_multiplication(self):
        self.assertEqual(calculator_core("3*4"), "12")
        self.assertEqual(calculator_core("5*0"), "0")
        self.assertEqual(calculator_core("1*1"), "1")

    def test_simple_division(self):
        self.assertEqual(calculator_core("8/2"), "4")
        self.assertEqual(calculator_core("1/1"), "1")
        self.assertTrue(calculator_core("10/3").startswith("3.33333"))

    def test_exponentiation(self):
        self.assertEqual(calculator_core("2^3"), "8")
        self.assertEqual(calculator_core("5^2"), "25")
        self.assertEqual(calculator_core("2^0"), "1")

    def test_double_minus(self):
        self.assertEqual(calculator_core("5--3"), "8")
        self.assertEqual(calculator_core("10--5"), "15")

    def test_double_plus(self):
        self.assertEqual(calculator_core("5++3"), "8")
        self.assertEqual(calculator_core("10++5"), "15")

    def test_mixed_signs(self):
        self.assertEqual(calculator_core("5+-3"), "2")
        self.assertEqual(calculator_core("5-+3"), "2")

    def test_simple_parentheses(self):
        self.assertEqual(calculator_core("(2+3)"), "5")
        self.assertEqual(calculator_core("(10-5)"), "5")
        self.assertEqual(calculator_core("(3*4)"), "12")
        self.assertEqual(calculator_core("(8/2)"), "4")

    def test_invalid_parentheses(self):
        self.assertEqual(calculator_core("(2+"), "(2+")
        self.assertEqual(calculator_core("2+)"), "2+)")

    def test_nested_parentheses_partially(self):
        self.assertIsInstance(calculator_core("((2+3))"), str)

    def test_operator_precedence_behavior(self):
        self.assertIsInstance(calculator_core("2+3*4"), str)
        self.assertIsInstance(calculator_core("2*3*4"), str)

    def test_basic_currency_conversion(self):
        self.assertEqual(calculator_core("$10*2", {"$": 100}), "2000")

    def test_multiple_currencies(self):
        self.assertIsInstance(calculator_core("$10+E5", {"$": 100, "E": 110}), str)

    def test_without_currency_conversion(self):
        self.assertEqual(calculator_core("10+20"), "30")
        self.assertEqual(calculator_core("10*20", {}), "200")

    def test_decimal_numbers(self):
        self.assertEqual(calculator_core("2.5+1.5"), "4")
        self.assertEqual(calculator_core("3.14*2"), "6.28")
        self.assertEqual(calculator_core("10.5/2.1"), "5")

    def test_precision_issues(self):
        self.assertEqual(calculator_core("0.1+0.2"), "0.3")

    def test_invalid_input(self):
        self.assertEqual(calculator_core(""), "")
        self.assertEqual(calculator_core("abc"), "abc")
        self.assertEqual(calculator_core("*/"), "*/")

    def test_division_by_zero(self):
        self.assertEqual(calculator_core("5/0"), "5/0")
        self.assertEqual(calculator_core("0/0"), "0/0")

    def test_numbers_only(self):
        self.assertEqual(calculator_core("42"), "42")
        self.assertEqual(calculator_core("0"), "0")
        self.assertEqual(calculator_core("-5"), "-5")

    def test_incomplete_expressions(self):
        self.assertEqual(calculator_core("1+"), "1+")
        self.assertEqual(calculator_core("2*"), "2*")
        self.assertEqual(calculator_core("5/"), "5/")
        self.assertEqual(calculator_core("3^"), "3^")

    def test_negative_numbers(self):
        self.assertEqual(calculator_core("-5+3"), "-2")
        self.assertEqual(calculator_core("-10/2"), "-5")
        self.assertEqual(calculator_core("3-5"), "-2")
        self.assertEqual(calculator_core("1-10"), "-9")

    def test_expressions_that_dont_match_patterns(self):
        self.assertEqual(calculator_core("1 + 1"), "1 + 1")
        self.assertEqual(calculator_core("a+b"), "a+b")

    def test_very_long_numbers(self):
        self.assertEqual(calculator_core("999999999+1"), "1000000000")
        self.assertEqual(calculator_core("123456789*2"), "246913578")

    def test_currency_with_no_numbers(self):
        self.assertEqual(calculator_core("Q*2", {"Q": 3}), "Q*2")


if __name__ == "__main__":
    unittest.main()
