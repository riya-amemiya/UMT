import unittest

from src import to_celsius


class TestToCelsius(unittest.TestCase):
    def test_basic_cases(self):
        self.assertAlmostEqual(to_celsius(273.15), 0)
        self.assertAlmostEqual(to_celsius(373.15), 100)

    def test_docstring_example(self):
        self.assertAlmostEqual(to_celsius(300), 26.85)
        self.assertEqual(to_celsius(273.15), 0.0)
        self.assertEqual(to_celsius(0), -273.15)


if __name__ == "__main__":
    unittest.main()
