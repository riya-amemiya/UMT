import unittest
from datetime import datetime, timedelta

from src.date import sub_duration


class TestSubDuration(unittest.TestCase):
    def test_subtracts_milliseconds(self):
        base = datetime(2025, 1, 1, 0, 0, 0)
        result = sub_duration(base, 1000, "ms")
        self.assertEqual(base - result, timedelta(milliseconds=1000))

    def test_subtracts_months_and_clamps_to_last_day(self):
        base = datetime(2025, 3, 31)
        result = sub_duration(base, 1, "M")
        self.assertEqual(result.month, 2)
        self.assertEqual(result.day, 28)

    def test_subtracts_years_across_leap_year(self):
        base = datetime(2025, 2, 28)
        result = sub_duration(base, 1, "y")
        self.assertEqual(result.year, 2024)
        self.assertEqual(result.month, 2)
        self.assertEqual(result.day, 28)

    def test_subtracts_seconds(self):
        base = datetime(2025, 6, 1, 12, 0, 0)
        result = sub_duration(base, 30, "s")
        self.assertEqual(base - result, timedelta(seconds=30))

    def test_subtracts_minutes(self):
        base = datetime(2025, 6, 1, 12, 0, 0)
        result = sub_duration(base, 5, "m")
        self.assertEqual(base - result, timedelta(minutes=5))

    def test_subtracts_hours(self):
        base = datetime(2025, 6, 1, 12, 0, 0)
        result = sub_duration(base, 1, "h")
        self.assertEqual(base - result, timedelta(hours=1))

    def test_subtracts_days(self):
        base = datetime(2025, 6, 1, 12, 0, 0)
        result = sub_duration(base, 1, "d")
        self.assertEqual(base - result, timedelta(days=1))

    def test_subtracts_weeks(self):
        base = datetime(2025, 6, 15, 12, 0, 0)
        result = sub_duration(base, 1, "w")
        self.assertEqual(base - result, timedelta(days=7))


if __name__ == "__main__":
    unittest.main()
