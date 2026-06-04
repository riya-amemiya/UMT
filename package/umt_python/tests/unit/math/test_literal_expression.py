import unittest

from src.math import literal_expression


class TestLiteralExpression(unittest.TestCase):
    def test_basic_equations(self):
        self.assertEqual(literal_expression("x=1"), "1")
        self.assertEqual(literal_expression("x=0"), "0")
        self.assertEqual(literal_expression("x=-1"), "-1")

    def test_basic_equations_right_side(self):
        self.assertEqual(literal_expression("1=x"), "1")
        self.assertEqual(literal_expression("0=x"), "0")
        self.assertEqual(literal_expression("-1=x"), "-1")

    def test_mixed_numerical_and_variable_parts(self):
        self.assertEqual(literal_expression("2x=4"), "2")
        self.assertEqual(literal_expression("1x=1"), "1")
        self.assertEqual(literal_expression("3x=-6"), "-2")
        self.assertEqual(literal_expression("2x+2=4"), "1")
        self.assertEqual(literal_expression("3x-2=-6"), "-4/3")

    def test_mixed_numerical_and_variable_parts_right_side(self):
        self.assertEqual(literal_expression("4=2x"), "2")
        self.assertEqual(literal_expression("1=1x"), "1")
        self.assertEqual(literal_expression("-6=3x"), "-2")
        self.assertEqual(literal_expression("4=2x+2"), "1")
        self.assertEqual(literal_expression("-6=3x-2"), "-4/3")

    def test_simplify_when_gcd_greater_than_one(self):
        self.assertEqual(literal_expression("2x=8"), "4")

    def test_simplify_when_gcd_greater_than_one_right_side(self):
        self.assertEqual(literal_expression("8=2x"), "4")

    def test_fractions_that_cannot_be_simplified(self):
        self.assertEqual(literal_expression("6x=4"), "2/3")

    def test_invalid_equations(self):
        self.assertEqual(literal_expression("x=x"), "")


if __name__ == "__main__":
    unittest.main()
