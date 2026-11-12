import unittest
from datetime import datetime

from src.date import is_after


class TestIsAfter(unittest.TestCase):
    def test_returns_true_when_later_without_unit(self):
        self.assertTrue(
            is_after(
                datetime(2025, 4, 15, 10, 30, 45, 124000),
                datetime(2025, 4, 15, 10, 30, 45, 123000),
            )
        )

    def test_returns_false_for_identical_timestamps_without_unit(self):
        date = datetime(2025, 4, 15, 10, 30, 45, 123000)
        self.assertFalse(is_after(date, datetime(2025, 4, 15, 10, 30, 45, 123000)))

    def test_returns_false_when_earlier_without_unit(self):
        self.assertFalse(is_after(datetime(2025, 4, 15), datetime(2025, 4, 16)))

    def test_returns_false_within_same_day(self):
        self.assertFalse(
            is_after(datetime(2025, 4, 15, 23, 59), datetime(2025, 4, 15, 1), "day")
        )

    def test_returns_true_for_next_day(self):
        self.assertTrue(
            is_after(datetime(2025, 4, 16, 0, 0), datetime(2025, 4, 15, 23, 59), "day")
        )

    def test_returns_false_within_same_sunday_start_week(self):
        self.assertFalse(is_after(datetime(2025, 4, 19), datetime(2025, 4, 13), "week"))

    def test_returns_true_across_week_boundary(self):
        self.assertTrue(is_after(datetime(2025, 4, 20), datetime(2025, 4, 19), "week"))

    def test_compares_month_quarter_and_year(self):
        self.assertTrue(is_after(datetime(2025, 5, 1), datetime(2025, 4, 30), "month"))
        self.assertFalse(
            is_after(datetime(2025, 6, 30), datetime(2025, 4, 1), "quarter")
        )
        self.assertTrue(is_after(datetime(2026, 1, 1), datetime(2025, 12, 31), "year"))

    def test_compares_second_minute_and_hour(self):
        self.assertFalse(
            is_after(
                datetime(2025, 4, 15, 10, 30, 45, 999000),
                datetime(2025, 4, 15, 10, 30, 45, 0),
                "second",
            )
        )
        self.assertTrue(
            is_after(
                datetime(2025, 4, 15, 10, 31),
                datetime(2025, 4, 15, 10, 30, 59),
                "minute",
            )
        )
        self.assertFalse(
            is_after(datetime(2025, 4, 15, 10, 59), datetime(2025, 4, 15, 10), "hour")
        )


if __name__ == "__main__":
    unittest.main()
