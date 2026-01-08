import unittest
from datetime import datetime, timezone, timedelta
from unittest.mock import patch

from src.date import (
    birthday,
    date_now,
    date_range,
    day_of_week,
    is_leap_year,
)


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


class TestDayOfWeek(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(day_of_week(2000, 1, 1), 5)
        self.assertEqual(day_of_week(2025, 1, 1), 2)

    def test_edge_cases(self):
        self.assertEqual(day_of_week(2024, 2, 29), 3)

    def test_docstring_example(self):
        self.assertEqual(day_of_week(2000, 1, 1), 5)


class TestDateRange(unittest.TestCase):
    def test_basic_cases(self):
        start = datetime(2025, 1, 1)
        end = datetime(2025, 1, 3)
        result = date_range(start, end)
        self.assertEqual(len(result), 3)
        self.assertEqual(result[0], datetime(2025, 1, 1))
        self.assertEqual(result[1], datetime(2025, 1, 2))
        self.assertEqual(result[2], datetime(2025, 1, 3))

    def test_single_day(self):
        start = datetime(2025, 1, 1)
        end = datetime(2025, 1, 1)
        result = date_range(start, end)
        self.assertEqual(len(result), 1)

    def test_edge_cases(self):
        start = datetime(2025, 1, 5)
        end = datetime(2025, 1, 1)
        result = date_range(start, end)
        self.assertEqual(len(result), 0)

    def test_docstring_example(self):
        result = date_range(datetime(2025, 1, 1), datetime(2025, 1, 3))
        self.assertEqual(len(result), 3)


class TestDateNow(unittest.TestCase):
    def test_basic_cases(self):
        result = date_now()
        self.assertIsNotNone(result)
        self.assertIsInstance(result, datetime)

    def test_timezone_offset(self):
        result_jst = date_now(9)
        result_utc = date_now(0)
        diff = result_jst.utcoffset() - result_utc.utcoffset()
        self.assertEqual(diff, timedelta(hours=9))


class TestBirthday(unittest.TestCase):
    def test_basic_cases(self):
        age = birthday(2000, 1, 1)
        self.assertIsInstance(age, int)
        self.assertGreaterEqual(age, 0)

    def test_edge_cases(self):
        future_age = birthday(2100, 1, 1)
        self.assertEqual(future_age, 0)

    def test_returns_integer(self):
        age = birthday(1990, 6, 15)
        self.assertIsInstance(age, int)


if __name__ == "__main__":
    unittest.main()
