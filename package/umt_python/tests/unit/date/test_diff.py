import unittest
from datetime import datetime

from src.date import diff


class TestDiff(unittest.TestCase):
    def test_computes_day_difference(self):
        self.assertEqual(diff(datetime(2025, 12, 31), datetime(2025, 1, 1), "d"), 364)

    def test_computes_hour_difference(self):
        left = datetime(2025, 1, 1, 5, 0, 0)
        right = datetime(2025, 1, 1, 0, 0, 0)
        self.assertEqual(diff(left, right, "h"), 5)

    def test_computes_minute_difference(self):
        left = datetime(2025, 1, 1, 0, 30, 0)
        right = datetime(2025, 1, 1, 0, 0, 0)
        self.assertEqual(diff(left, right, "m"), 30)

    def test_computes_second_difference(self):
        left = datetime(2025, 1, 1, 0, 0, 30)
        right = datetime(2025, 1, 1, 0, 0, 0)
        self.assertEqual(diff(left, right, "s"), 30)

    def test_computes_millisecond_difference(self):
        left = datetime(2025, 1, 1, 0, 0, 0, 500_000)
        right = datetime(2025, 1, 1, 0, 0, 0)
        self.assertEqual(diff(left, right, "ms"), 500)

    def test_computes_week_difference(self):
        left = datetime(2025, 1, 15)
        right = datetime(2025, 1, 1)
        self.assertEqual(diff(left, right, "w"), 2)

    def test_positive_month_difference_adjusts_incomplete_month(self):
        self.assertEqual(diff(datetime(2025, 3, 14), datetime(2025, 1, 15), "M"), 1)
        self.assertEqual(diff(datetime(2025, 3, 16), datetime(2025, 1, 15), "M"), 2)

    def test_negative_month_difference_adjusts_incomplete_month(self):
        self.assertEqual(diff(datetime(2025, 1, 15), datetime(2025, 3, 14), "M"), -1)
        self.assertEqual(diff(datetime(2025, 1, 15), datetime(2025, 3, 16), "M"), -2)

    def test_respects_time_of_day_for_positive_month_boundary(self):
        self.assertEqual(
            diff(
                datetime(2025, 2, 15, 10, 0, 0, 0),
                datetime(2025, 1, 15, 11, 0, 0, 0),
                "M",
            ),
            0,
        )

    def test_respects_time_of_day_for_negative_month_boundary(self):
        self.assertEqual(
            diff(
                datetime(2025, 1, 15, 11, 0, 0, 0),
                datetime(2025, 2, 15, 10, 0, 0, 0),
                "M",
            ),
            0,
        )

    def test_computes_year_difference(self):
        self.assertEqual(diff(datetime(2026, 1, 1), datetime(2025, 1, 1), "y"), 1)
        self.assertEqual(diff(datetime(2025, 1, 1), datetime(2026, 1, 1), "y"), -1)

    def test_returns_zero_for_identical_dates(self):
        date = datetime(2025, 1, 1)
        self.assertEqual(diff(date, date, "d"), 0)
        self.assertEqual(diff(date, date, "M"), 0)
        self.assertEqual(diff(date, date, "y"), 0)

    def test_truncates_fractional_fixed_unit_differences_toward_zero(self):
        left = datetime(1970, 1, 1, 0, 0, 1, 500_000)
        right = datetime(1970, 1, 1, 0, 0, 0)
        self.assertEqual(diff(left, right, "s"), 1)
        self.assertEqual(diff(right, left, "s"), -1)


if __name__ == "__main__":
    unittest.main()
