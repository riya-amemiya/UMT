import unittest

from src.math import calculator_core


class TestCalculatorCoreParity(unittest.TestCase):
    def test_python_only_infinity_spelling_is_returned_unchanged(self):
        self.assertEqual(calculator_core("inf"), "inf")

    def test_underscore_separator_is_returned_unchanged(self):
        self.assertEqual(calculator_core("1_0"), "1_0")

    def test_hexadecimal_literal_is_evaluated(self):
        self.assertEqual(calculator_core("0x10"), "16")

    def test_infinity_literal(self):
        self.assertEqual(calculator_core("Infinity"), "Infinity")


if __name__ == "__main__":
    unittest.main()
