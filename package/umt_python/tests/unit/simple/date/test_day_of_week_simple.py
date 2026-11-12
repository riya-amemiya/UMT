import unittest
from datetime import datetime, timezone

from src.simple.date import day_of_week_simple


class TestDayOfWeekSimple(unittest.TestCase):
    """Tests ported from Rust: test_day_of_week_simple.rs

    Expected values follow the TypeScript source of truth,
    which returns 0=Sunday, 6=Saturday.
    """

    # String format tests (hyphen delimiter)
    def test_should_return_0_for_sunday(self):
        # 2022-01-02 is Sunday -> Python weekday() = 6
        self.assertEqual(day_of_week_simple("2022-01-02"), 0)

    def test_should_return_1_for_monday_slash(self):
        # 2022-01-03 is Monday -> Python weekday() = 0
        self.assertEqual(day_of_week_simple("2022/01/03"), 1)

    def test_should_return_2_for_tuesday_colon(self):
        # 2022-01-04 is Tuesday -> Python weekday() = 1
        self.assertEqual(day_of_week_simple("2022:01:04"), 2)

    def test_should_return_3_for_wednesday_dict(self):
        # 2022-01-05 is Wednesday -> Python weekday() = 2
        self.assertEqual(day_of_week_simple({"year": 2022, "mon": 1, "day": 5}), 3)

    def test_should_return_4_for_thursday(self):
        self.assertEqual(day_of_week_simple("2022-01-06"), 4)

    def test_should_return_5_for_friday(self):
        self.assertEqual(day_of_week_simple("2022-01-07"), 5)

    def test_should_return_6_for_saturday(self):
        self.assertEqual(day_of_week_simple("2022-01-08"), 6)

    # datetime object tests
    def test_datetime_sunday(self):
        dt = datetime(2022, 1, 2, tzinfo=timezone.utc)
        self.assertEqual(day_of_week_simple(dt), 0)

    def test_datetime_monday(self):
        dt = datetime(2022, 1, 3, tzinfo=timezone.utc)
        self.assertEqual(day_of_week_simple(dt), 1)

    def test_datetime_tuesday(self):
        dt = datetime(2022, 1, 4, tzinfo=timezone.utc)
        self.assertEqual(day_of_week_simple(dt), 2)

    # Known dates
    def test_known_date_2024_01_01_is_monday(self):
        self.assertEqual(day_of_week_simple("2024-01-01"), 1)

    def test_known_date_2024_12_25_is_wednesday(self):
        self.assertEqual(day_of_week_simple("2024-12-25"), 3)

    def test_known_date_2000_01_01_was_saturday(self):
        self.assertEqual(day_of_week_simple("2000-01-01"), 6)

    # Default properties test
    def test_with_none_returns_current_day(self):
        day = day_of_week_simple(None)
        self.assertIn(day, range(7))

    # Format variation tests
    def test_slash_delimiter_sunday(self):
        self.assertEqual(day_of_week_simple("2022/01/02"), 0)

    def test_colon_delimiter_sunday(self):
        self.assertEqual(day_of_week_simple("2022:01:02"), 0)
