import unittest
from datetime import datetime

from src.date import days_in_month


class TestDaysInMonth(unittest.TestCase):
    def test_returns_31_for_31_day_months(self):
        for month in (1, 3, 5, 7, 8, 10, 12):
            self.assertEqual(days_in_month(datetime(2025, month, 15)), 31)

    def test_returns_30_for_30_day_months(self):
        for month in (4, 6, 9, 11):
            self.assertEqual(days_in_month(datetime(2025, month, 15)), 30)

    def test_returns_28_for_february_in_common_year(self):
        self.assertEqual(days_in_month(datetime(2025, 2, 1)), 28)
        self.assertEqual(days_in_month(datetime(2100, 2, 1)), 28)

    def test_returns_29_for_february_in_leap_year(self):
        self.assertEqual(days_in_month(datetime(2024, 2, 1)), 29)
        self.assertEqual(days_in_month(datetime(2000, 2, 1)), 29)

    def test_does_not_depend_on_day_or_time(self):
        self.assertEqual(days_in_month(datetime(2025, 1, 31, 23, 59, 59, 999000)), 31)
        self.assertEqual(days_in_month(datetime(2024, 2, 29, 12)), 29)


if __name__ == "__main__":
    unittest.main()
