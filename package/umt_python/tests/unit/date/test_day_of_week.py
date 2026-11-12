import unittest
from datetime import datetime, timedelta, timezone

from src.date import day_of_week


class TestDayOfWeek(unittest.TestCase):
    def test_basic_cases(self):
        self.assertEqual(day_of_week(2000, 1, 1), 6)
        self.assertEqual(day_of_week(2025, 1, 1), 3)

    def test_edge_cases(self):
        self.assertEqual(day_of_week(2024, 2, 29), 4)

    def test_docstring_example(self):
        self.assertEqual(day_of_week(2000, 1, 1), 6)


class TestDayOfWeekSundayStart(unittest.TestCase):
    def test_counts_from_sunday(self):
        self.assertEqual(day_of_week(2020, 1, 1), 3)
        self.assertEqual(day_of_week(2020, 1, 2), 4)
        self.assertEqual(day_of_week(2000, 1, 1), 6)
        self.assertEqual(day_of_week(2025, 4, 13), 0)

    def test_defaults_to_current_date(self):
        target_tz = timezone(timedelta(hours=9))
        today = datetime.now(target_tz)
        self.assertIn(
            day_of_week(), {(today.weekday() + 1) % 7, (today.weekday() + 2) % 7}
        )
        self.assertIn(day_of_week(time_difference=0), range(7))

    def test_partial_arguments_use_current_year_and_month(self):
        today = datetime.now(timezone(timedelta(hours=9)))
        self.assertIn(day_of_week(day=2), range(7))
        self.assertEqual(
            day_of_week(mon=1, day=2),
            (datetime(today.year, 1, 2).weekday() + 1) % 7,
        )


if __name__ == "__main__":
    unittest.main()
