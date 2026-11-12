import unittest
from datetime import datetime

from src.date import is_same


class TestIsSame(unittest.TestCase):
    def test_returns_true_for_identical_timestamps_without_unit(self):
        date = datetime(2025, 4, 15, 10, 30, 45, 123000)
        self.assertTrue(is_same(date, datetime(2025, 4, 15, 10, 30, 45, 123000)))

    def test_returns_false_for_different_milliseconds_without_unit(self):
        self.assertFalse(
            is_same(
                datetime(2025, 4, 15, 10, 30, 45, 123000),
                datetime(2025, 4, 15, 10, 30, 45, 124000),
            )
        )

    def test_returns_true_for_same_second_ignoring_milliseconds(self):
        self.assertTrue(
            is_same(
                datetime(2025, 4, 15, 10, 30, 45, 0),
                datetime(2025, 4, 15, 10, 30, 45, 999000),
                "second",
            )
        )

    def test_returns_false_for_different_seconds(self):
        self.assertFalse(
            is_same(
                datetime(2025, 4, 15, 10, 30, 45),
                datetime(2025, 4, 15, 10, 30, 46),
                "second",
            )
        )

    def test_returns_true_for_same_minute_ignoring_seconds(self):
        self.assertTrue(
            is_same(
                datetime(2025, 4, 15, 10, 30, 0),
                datetime(2025, 4, 15, 10, 30, 59),
                "minute",
            )
        )

    def test_returns_true_for_same_hour_ignoring_minutes(self):
        self.assertTrue(
            is_same(
                datetime(2025, 4, 15, 10, 0),
                datetime(2025, 4, 15, 10, 59),
                "hour",
            )
        )

    def test_returns_true_for_same_day_with_different_times(self):
        self.assertTrue(
            is_same(datetime(2025, 4, 15, 1, 0), datetime(2025, 4, 15, 23, 59), "day")
        )

    def test_returns_false_for_adjacent_days(self):
        self.assertFalse(is_same(datetime(2025, 4, 15), datetime(2025, 4, 16), "day"))

    def test_returns_true_for_same_week_sunday_start(self):
        self.assertTrue(is_same(datetime(2025, 4, 13), datetime(2025, 4, 19), "week"))

    def test_returns_false_across_week_boundary(self):
        self.assertFalse(is_same(datetime(2025, 4, 19), datetime(2025, 4, 20), "week"))

    def test_returns_true_for_same_month_with_different_days(self):
        self.assertTrue(is_same(datetime(2025, 4, 1), datetime(2025, 4, 30), "month"))

    def test_returns_false_for_different_months(self):
        self.assertFalse(is_same(datetime(2025, 4, 15), datetime(2025, 5, 15), "month"))

    def test_returns_true_for_same_quarter(self):
        self.assertTrue(is_same(datetime(2025, 4, 1), datetime(2025, 6, 30), "quarter"))

    def test_returns_false_for_different_quarters(self):
        self.assertFalse(
            is_same(datetime(2025, 3, 31), datetime(2025, 4, 1), "quarter")
        )

    def test_returns_true_for_same_year_with_different_months(self):
        self.assertTrue(is_same(datetime(2025, 1, 1), datetime(2025, 12, 31), "year"))

    def test_returns_false_for_different_years(self):
        self.assertFalse(is_same(datetime(2024, 4, 15), datetime(2025, 4, 15), "year"))

    def test_docstring_example(self):
        self.assertTrue(
            is_same(datetime(2025, 4, 15, 1), datetime(2025, 4, 15, 23), "day")
        )
        self.assertFalse(is_same(datetime(2025, 4, 15), datetime(2025, 5, 15), "month"))
        self.assertTrue(is_same(datetime(2025, 1, 1), datetime(2025, 1, 1)))


if __name__ == "__main__":
    unittest.main()
