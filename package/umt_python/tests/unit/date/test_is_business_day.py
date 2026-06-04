import unittest
from datetime import datetime

from src.date import is_business_day


class TestIsBusinessDay(unittest.TestCase):
    def test_returns_true_for_monday_with_no_holidays(self):
        self.assertTrue(is_business_day(datetime(2025, 4, 21)))

    def test_returns_false_for_saturday(self):
        self.assertFalse(is_business_day(datetime(2025, 4, 19)))

    def test_returns_false_for_sunday(self):
        self.assertFalse(is_business_day(datetime(2025, 4, 20)))

    def test_returns_false_for_weekday_listed_as_holiday(self):
        date = datetime(2025, 4, 21)
        holidays = [datetime(2025, 4, 21, 12, 0)]
        self.assertFalse(is_business_day(date, holidays))

    def test_returns_true_for_weekday_not_in_holiday_list(self):
        date = datetime(2025, 4, 22)
        holidays = [datetime(2025, 4, 21)]
        self.assertTrue(is_business_day(date, holidays))


if __name__ == "__main__":
    unittest.main()
