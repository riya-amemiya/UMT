import unittest

from src.math import to_kelvin


class TestToKelvin(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(to_kelvin(0), 273.15)
        self.assertEqual(to_kelvin(26.85), 300)
        self.assertEqual(to_kelvin(100), 373.15)

    def test_handle_negative_celsius_values(self):
        self.assertEqual(to_kelvin(-40), 233.15)
        self.assertEqual(to_kelvin(-273.15), 0)

    def test_handle_extreme_temperatures(self):
        self.assertEqual(to_kelvin(1000), 1273.15)
        self.assertEqual(to_kelvin(-300), -26.85)

    def test_handle_decimal_values(self):
        self.assertEqual(to_kelvin(25.5), 298.65)


if __name__ == "__main__":
    unittest.main()
