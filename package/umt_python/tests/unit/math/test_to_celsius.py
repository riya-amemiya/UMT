import unittest

from src.math import to_celsius


class TestToCelsius(unittest.TestCase):
    def test_convert_kelvin_to_celsius(self):
        self.assertEqual(to_celsius(273.15), 0)
        self.assertEqual(to_celsius(300), 26.85)
        self.assertEqual(to_celsius(373.15), 100)
        self.assertEqual(to_celsius(0), -273.15)

    def test_handle_non_standard_temperatures(self):
        self.assertEqual(to_celsius(32), -241.15)
        self.assertEqual(to_celsius(1000), 726.85)

    def test_handle_negative_kelvin_values(self):
        self.assertEqual(to_celsius(-100), -373.15)


if __name__ == "__main__":
    unittest.main()
