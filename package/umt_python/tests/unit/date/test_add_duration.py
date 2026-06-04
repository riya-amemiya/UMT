import unittest
from datetime import datetime, timedelta

from src.date import add_duration


class TestAddDuration(unittest.TestCase):
    def test_adds_milliseconds(self):
        base = datetime(2025, 1, 1, 0, 0, 0)
        result = add_duration(base, 500, "ms")
        self.assertEqual(result - base, timedelta(milliseconds=500))

    def test_adds_seconds(self):
        base = datetime(2025, 1, 1, 0, 0, 0)
        result = add_duration(base, 30, "s")
        self.assertEqual(result - base, timedelta(seconds=30))

    def test_adds_minutes(self):
        base = datetime(2025, 1, 1, 0, 0, 0)
        result = add_duration(base, 5, "m")
        self.assertEqual(result - base, timedelta(minutes=5))

    def test_adds_hours(self):
        base = datetime(2025, 1, 1, 0, 0, 0)
        result = add_duration(base, 2, "h")
        self.assertEqual(result - base, timedelta(hours=2))

    def test_adds_days(self):
        base = datetime(2025, 1, 1, 0, 0, 0)
        result = add_duration(base, 7, "d")
        self.assertEqual(result - base, timedelta(days=7))

    def test_adds_weeks(self):
        base = datetime(2025, 1, 1, 0, 0, 0)
        result = add_duration(base, 2, "w")
        self.assertEqual(result - base, timedelta(days=14))

    def test_adds_months_and_clamps_to_last_day(self):
        base = datetime(2025, 1, 31)
        result = add_duration(base, 1, "M")
        self.assertEqual(result.year, 2025)
        self.assertEqual(result.month, 2)
        self.assertEqual(result.day, 28)

    def test_adds_months_across_year_boundary(self):
        base = datetime(2025, 12, 15)
        result = add_duration(base, 2, "M")
        self.assertEqual(result.year, 2026)
        self.assertEqual(result.month, 2)
        self.assertEqual(result.day, 15)

    def test_subtracts_via_negative_amount(self):
        base = datetime(2025, 3, 15)
        result = add_duration(base, -1, "M")
        self.assertEqual(result.month, 2)

    def test_adds_years_preserving_month_and_day(self):
        base = datetime(2024, 2, 29)
        result = add_duration(base, 1, "y")
        self.assertEqual(result.year, 2025)
        self.assertEqual(result.month, 2)
        self.assertEqual(result.day, 28)

    def test_adds_years_across_leap_year_boundary(self):
        base = datetime(2024, 2, 29)
        result = add_duration(base, 4, "y")
        self.assertEqual(result.year, 2028)
        self.assertEqual(result.month, 2)
        self.assertEqual(result.day, 29)

    def test_does_not_mutate_the_input_date(self):
        base = datetime(2025, 1, 1)
        snapshot = datetime(2025, 1, 1)
        add_duration(base, 5, "M")
        self.assertEqual(base, snapshot)


if __name__ == "__main__":
    unittest.main()
