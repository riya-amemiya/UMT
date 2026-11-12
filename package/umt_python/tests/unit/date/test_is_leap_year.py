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


class TestIsLeapYearNumericInputs(unittest.TestCase):
    def test_integral_float_is_treated_as_integer(self):
        self.assertTrue(is_leap_year(2000.0))
        self.assertFalse(is_leap_year(2100.0))

    def test_non_finite_values_are_not_leap_years(self):
        self.assertFalse(is_leap_year(float("nan")))
        self.assertFalse(is_leap_year(float("inf")))

    def test_bool_is_rejected(self):
        self.assertFalse(is_leap_year(True))
        self.assertFalse(is_leap_year(False))

    def test_zero_and_negative_years(self):
        self.assertTrue(is_leap_year(0))
        self.assertTrue(is_leap_year(-4))
        self.assertFalse(is_leap_year(-1))


if __name__ == "__main__":
    unittest.main()
