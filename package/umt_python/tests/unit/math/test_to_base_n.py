import unittest

from src.math import to_base_n


class TestToBaseN(unittest.TestCase):
    def test_binary_conversion(self):
        self.assertEqual(to_base_n(10, 2), "1010")
        self.assertEqual(to_base_n(255, 2), "11111111")

    def test_octal_conversion(self):
        self.assertEqual(to_base_n(8, 8), "10")
        self.assertEqual(to_base_n(64, 8), "100")

    def test_hex_conversion(self):
        self.assertEqual(to_base_n(255, 16), "ff")
        self.assertEqual(to_base_n(15, 16), "f")

    def test_custom_base(self):
        self.assertEqual(to_base_n(10, 3), "101")
        self.assertEqual(to_base_n(100, 5), "400")
        self.assertEqual(to_base_n(35, 36), "z")

    def test_zero_value(self):
        self.assertEqual(to_base_n(0, 2), "0")
        self.assertEqual(to_base_n(0, 10), "0")
        self.assertEqual(to_base_n(0, 5), "0")

    def test_negative_values(self):
        self.assertEqual(to_base_n(-10, 3), "-101")
        self.assertEqual(to_base_n(-5, 5), "-10")

    def test_docstring_example(self):
        self.assertEqual(to_base_n(10), "1010")
        self.assertEqual(to_base_n(15, 16), "f")
        self.assertEqual(to_base_n(255, 16), "ff")


if __name__ == "__main__":
    unittest.main()
