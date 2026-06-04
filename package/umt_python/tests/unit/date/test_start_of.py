import unittest
from datetime import datetime

from src.date import start_of


def _js_day(date: datetime) -> int:
    return (date.weekday() + 1) % 7


class TestStartOf(unittest.TestCase):
    def test_zeros_milliseconds_for_second(self):
        result = start_of(datetime(2025, 4, 15, 10, 30, 45, 123_000), "second")
        self.assertEqual(result.microsecond, 0)
        self.assertEqual(result.second, 45)

    def test_zeros_seconds_for_minute(self):
        result = start_of(datetime(2025, 4, 15, 10, 30, 45, 123_000), "minute")
        self.assertEqual(result.second, 0)
        self.assertEqual(result.microsecond, 0)

    def test_zeros_minutes_for_hour(self):
        result = start_of(datetime(2025, 4, 15, 10, 30, 45, 123_000), "hour")
        self.assertEqual(result.minute, 0)
        self.assertEqual(result.second, 0)
        self.assertEqual(result.microsecond, 0)

    def test_zeros_hours_for_day(self):
        result = start_of(datetime(2025, 4, 15, 10, 30), "day")
        self.assertEqual(result.hour, 0)
        self.assertEqual(result.minute, 0)
        self.assertEqual(result.second, 0)
        self.assertEqual(result.microsecond, 0)

    def test_returns_sunday_for_week(self):
        result = start_of(datetime(2025, 4, 16), "week")
        self.assertEqual(_js_day(result), 0)
        self.assertEqual(result.hour, 0)

    def test_returns_first_day_for_month(self):
        result = start_of(datetime(2025, 4, 15), "month")
        self.assertEqual(result.day, 1)
        self.assertEqual(result.month, 4)
        self.assertEqual(result.hour, 0)

    def test_returns_first_day_of_quarter_for_january(self):
        result = start_of(datetime(2025, 1, 31), "quarter")
        self.assertEqual(result.month, 1)
        self.assertEqual(result.day, 1)

    def test_returns_first_day_of_quarter_for_may(self):
        result = start_of(datetime(2025, 5, 31), "quarter")
        self.assertEqual(result.month, 4)
        self.assertEqual(result.day, 1)

    def test_returns_first_day_of_quarter_for_august(self):
        result = start_of(datetime(2025, 8, 31), "quarter")
        self.assertEqual(result.month, 7)
        self.assertEqual(result.day, 1)

    def test_returns_first_day_of_quarter_for_november(self):
        result = start_of(datetime(2025, 11, 30), "quarter")
        self.assertEqual(result.month, 10)
        self.assertEqual(result.day, 1)

    def test_returns_jan_1_for_year(self):
        result = start_of(datetime(2025, 6, 15), "year")
        self.assertEqual(result.month, 1)
        self.assertEqual(result.day, 1)
        self.assertEqual(result.hour, 0)

    def test_returns_unchanged_for_unknown_unit(self):
        input_date = datetime(2025, 4, 15, 10, 30)
        result = start_of(input_date, "unknown")
        self.assertEqual(result, input_date)


if __name__ == "__main__":
    unittest.main()
