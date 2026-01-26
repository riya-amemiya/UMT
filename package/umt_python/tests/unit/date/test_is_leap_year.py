import unittest

from src.date import is_leap_year


class TestIsLeapYear(unittest.TestCase):
    def test_basic_cases(self):
        self.assertTrue(is_leap_year(2000))
        self.assertTrue(is_leap_year(2020))
        self.assertFalse(is_leap_year(2100))
        self.assertFalse(is_leap_year(2023))

    def test_edge_cases(self):
        self.assertTrue(is_leap_year(2024))
        self.assertFalse(is_leap_year(1900))
        self.assertTrue(is_leap_year(1600))

    def test_invalid_input(self):
        self.assertFalse(is_leap_year(2020.5))

    def test_docstring_example(self):
        self.assertTrue(is_leap_year(2000))
        self.assertTrue(is_leap_year(2020))
        self.assertFalse(is_leap_year(2100))
        self.assertFalse(is_leap_year(2023))


if __name__ == "__main__":
    unittest.main()
