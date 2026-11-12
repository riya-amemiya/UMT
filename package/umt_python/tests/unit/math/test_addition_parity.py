import unittest

from src.math import addition


class TestAdditionParity(unittest.TestCase):
    def test_no_arguments_returns_zero(self):
        self.assertEqual(addition(), 0)

    def test_integer_fast_path_returns_float(self):
        result = addition(1, 2, 3)
        self.assertEqual(result, 6)
        self.assertIsInstance(result, float)

    def test_integral_floats_use_decimal_path(self):
        self.assertEqual(addition(2.0, 3.5), 5.5)

    def test_mixed_decimals(self):
        self.assertEqual(addition(0.1, 0.2, 0.3), 0.6)


if __name__ == "__main__":
    unittest.main()
