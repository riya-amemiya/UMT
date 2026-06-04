import unittest
from datetime import datetime

from src.date import end_of


def _js_day(date: datetime) -> int:
    return (date.weekday() + 1) % 7


class TestEndOf(unittest.TestCase):
    def test_sets_milliseconds_to_999_for_second(self):
        result = end_of(datetime(2025, 4, 15, 10, 30, 45, 0), "second")
        self.assertEqual(result.microsecond, 999_000)
        self.assertEqual(result.second, 45)

    def test_sets_to_59_999_for_minute(self):
        result = end_of(datetime(2025, 4, 15, 10, 30, 0, 0), "minute")
        self.assertEqual(result.second, 59)
        self.assertEqual(result.microsecond, 999_000)

    def test_sets_to_xx_59_59_999_for_hour(self):
        result = end_of(datetime(2025, 4, 15, 10, 0, 0, 0), "hour")
        self.assertEqual(result.minute, 59)
        self.assertEqual(result.second, 59)
        self.assertEqual(result.microsecond, 999_000)

    def test_sets_to_23_59_59_999_for_day(self):
        result = end_of(datetime(2025, 4, 15, 0, 0, 0, 0), "day")
        self.assertEqual(result.hour, 23)
        self.assertEqual(result.minute, 59)
        self.assertEqual(result.second, 59)
        self.assertEqual(result.microsecond, 999_000)

    def test_returns_saturday_end_for_week(self):
        result = end_of(datetime(2025, 4, 16), "week")
        self.assertEqual(_js_day(result), 6)
        self.assertEqual(result.hour, 23)

    def test_returns_last_day_for_month(self):
        result = end_of(datetime(2025, 4, 1), "month")
        self.assertEqual(result.day, 30)
        self.assertEqual(result.month, 4)

    def test_returns_feb_28_for_non_leap_february(self):
        result = end_of(datetime(2025, 2, 1), "month")
        self.assertEqual(result.day, 28)

    def test_returns_feb_29_for_leap_february(self):
        result = end_of(datetime(2024, 2, 1), "month")
        self.assertEqual(result.day, 29)

    def test_returns_last_day_of_q1_quarter(self):
        result = end_of(datetime(2025, 1, 1), "quarter")
        self.assertEqual(result.month, 3)
        self.assertEqual(result.day, 31)

    def test_returns_last_day_of_q2_quarter(self):
        result = end_of(datetime(2025, 4, 1), "quarter")
        self.assertEqual(result.month, 6)
        self.assertEqual(result.day, 30)

    def test_returns_last_day_of_q3_quarter(self):
        result = end_of(datetime(2025, 7, 1), "quarter")
        self.assertEqual(result.month, 9)
        self.assertEqual(result.day, 30)

    def test_returns_last_day_of_q4_quarter(self):
        result = end_of(datetime(2025, 10, 1), "quarter")
        self.assertEqual(result.month, 12)
        self.assertEqual(result.day, 31)

    def test_returns_dec_31_for_year(self):
        result = end_of(datetime(2025, 6, 15), "year")
        self.assertEqual(result.month, 12)
        self.assertEqual(result.day, 31)
        self.assertEqual(result.hour, 23)

    def test_returns_unchanged_for_unknown_unit(self):
        input_date = datetime(2025, 4, 15, 10, 30)
        result = end_of(input_date, "unknown")
        self.assertEqual(result, input_date)


if __name__ == "__main__":
    unittest.main()
