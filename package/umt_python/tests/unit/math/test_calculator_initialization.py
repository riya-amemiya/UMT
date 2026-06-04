import unittest

from src.math import calculator_initialization


class TestCalculatorInitialization(unittest.TestCase):
    def test_initialize_with_exchange_rates(self):
        calculate = calculator_initialization({"$": 100})
        self.assertEqual(calculate("$1"), "100")
        self.assertEqual(calculate("100 + $1"), "200")

    def test_multiple_exchange_rates(self):
        calculate = calculator_initialization({"$": 100, "EUR": 120})
        self.assertEqual(calculate("$1"), "100")
        self.assertEqual(calculate("EUR1"), "120")
        self.assertEqual(calculate("$1 + EUR1"), "220")

    def test_docstring_examples(self):
        self.assertEqual(calculator_initialization({"$": 100})("$1"), "100")
        self.assertEqual(calculator_initialization({"EUR": 1.2})("EUR5 + 10"), "16")


if __name__ == "__main__":
    unittest.main()
