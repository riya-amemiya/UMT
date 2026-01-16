import unittest

from src import to_kelvin


class TestToKelvin(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(to_kelvin(0), 273.15)
        self.assertAlmostEqual(to_kelvin(100), 373.15)


if __name__ == "__main__":
    unittest.main()
