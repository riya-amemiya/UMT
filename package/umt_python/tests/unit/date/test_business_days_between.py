import unittest
from datetime import datetime

from src.date import add_business_days, business_days_between, is_same_day


class TestBusinessDaysBetween(unittest.TestCase):
    def test_returns_zero_for_same_day(self):
        self.assertEqual(
            business_days_between(datetime(2025, 4, 21, 9), datetime(2025, 4, 21, 18)),
            0,
        )

    def test_excludes_start_and_includes_end(self):
        self.assertEqual(
            business_days_between(datetime(2025, 4, 21), datetime(2025, 4, 22)), 1
        )
        self.assertEqual(
            business_days_between(datetime(2025, 4, 21), datetime(2025, 4, 25)), 4
        )

    def test_skips_weekends(self):
        self.assertEqual(
            business_days_between(datetime(2025, 4, 18), datetime(2025, 4, 21)), 1
        )
        self.assertEqual(
            business_days_between(datetime(2025, 4, 18), datetime(2025, 4, 20)), 0
        )
        self.assertEqual(
            business_days_between(datetime(2025, 4, 21), datetime(2025, 5, 2)), 9
        )

    def test_skips_holidays(self):
        self.assertEqual(
            business_days_between(
                datetime(2025, 4, 21),
                datetime(2025, 4, 25),
                [datetime(2025, 4, 23, 15)],
            ),
            3,
        )

    def test_returns_negative_count_when_end_before_start(self):
        self.assertEqual(
            business_days_between(datetime(2025, 4, 21), datetime(2025, 4, 18)), -1
        )
        self.assertEqual(
            business_days_between(datetime(2025, 4, 25), datetime(2025, 4, 21)), -4
        )

    def test_returns_zero_for_weekend_only_backward_range(self):
        self.assertEqual(
            business_days_between(datetime(2025, 4, 20), datetime(2025, 4, 19)), 0
        )

    def test_ignores_time_of_day(self):
        self.assertEqual(
            business_days_between(
                datetime(2025, 4, 21, 23, 59), datetime(2025, 4, 22, 0, 1)
            ),
            1,
        )

    def test_is_inverse_of_add_business_days(self):
        holidays = [datetime(2025, 4, 29)]
        start = datetime(2025, 4, 18, 9, 30)
        for amount in (-12, -5, -1, 1, 3, 7, 15):
            end = add_business_days(start, amount, holidays)
            count = business_days_between(start, end, holidays)
            self.assertEqual(count, amount)
            self.assertTrue(is_same_day(add_business_days(start, count, holidays), end))


if __name__ == "__main__":
    unittest.main()
