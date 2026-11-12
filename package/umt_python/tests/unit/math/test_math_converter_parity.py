import unittest

from src.math import math_converter


class TestMathConverterParity(unittest.TestCase):
    def test_square_is_expanded_without_float_suffix(self):
        self.assertEqual(math_converter("1250*1250"), "1500*1000+400*100+200*100+50*50")

    def test_power_operator(self):
        self.assertEqual(math_converter("1250^2"), "1500*1000+400*100+200*100+50*50")

    def test_small_remainder(self):
        self.assertEqual(math_converter("15*15"), "20*10+5*5")

    def test_decimal_operands_keep_their_fraction(self):
        self.assertEqual(math_converter("1.5*1.5"), "2*1+0.5*0.5")
        self.assertEqual(math_converter("12.5*12.5"), "15*10+2.5*2.5")

    def test_surrounding_text_is_dropped(self):
        self.assertEqual(math_converter("a12*12b"), "14*10+2*2")

    def test_different_operands_are_returned_unchanged(self):
        self.assertEqual(math_converter("12*13"), "12*13")

    def test_no_operator_is_returned_unchanged(self):
        self.assertEqual(math_converter("2+2"), "2+2")
        self.assertEqual(math_converter(""), "")

    def test_zero_primary_is_returned_unchanged(self):
        self.assertEqual(math_converter("0*0"), "0*0")

    def test_self_reproducing_expansion_terminates(self):
        self.assertEqual(math_converter("5*5"), "5*5+0*0")
        self.assertEqual(math_converter("100^100"), "100*100+0*0")


if __name__ == "__main__":
    unittest.main()
