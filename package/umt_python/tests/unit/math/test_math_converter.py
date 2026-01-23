import unittest

from src.math import math_converter


class TestMathConverter(unittest.TestCase):
    def test_square_pattern(self):
        result = math_converter("1250*1250")
        self.assertEqual(result, "1500.0*1000+400.0*100+200.0*100+50*50")

    def test_power_pattern(self):
        result = math_converter("100^100")
        self.assertIsInstance(result, str)

    def test_no_pattern_match(self):
        result = math_converter("abc")
        self.assertEqual(result, "abc")

    def test_different_operands(self):
        result = math_converter("100*200")
        self.assertEqual(result, "100*200")

    def test_small_remainder(self):
        result = math_converter("150*150")
        self.assertEqual(result, "200.0*100+50*50")

    def test_larger_remainder_with_recursion(self):
        result = math_converter("1500*1500")
        self.assertIn("*", result)
        self.assertIn("+", result)

    def test_zero_primary(self):
        result = math_converter("0*0")
        self.assertEqual(result, "0*0")

    def test_docstring_example(self):
        result = math_converter("1250*1250")
        self.assertIn("1500", result)
        self.assertIn("1000", result)


if __name__ == "__main__":
    unittest.main()
