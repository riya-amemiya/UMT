import unittest
from datetime import datetime

from src.date import start_of, week_of_year


class TestWeekOfYear(unittest.TestCase):
    def test_returns_1_for_january_1(self):
        self.assertEqual(week_of_year(datetime(2025, 1, 1)), 1)

    def test_keeps_dates_in_january_1_sunday_start_week_as_week_1(self):
        self.assertEqual(week_of_year(datetime(2025, 1, 4)), 1)

    def test_increments_on_the_following_sunday(self):
        self.assertEqual(week_of_year(datetime(2025, 1, 5)), 2)

    def test_matches_start_of_week_boundaries_across_a_year(self):
        date = datetime(2025, 4, 16)
        year_start_week = start_of(datetime(2025, 1, 1), "week")
        current_week = start_of(date, "week")
        expected = (
            round((current_week - year_start_week).total_seconds() / 86_400) // 7 + 1
        )
        self.assertEqual(week_of_year(date), expected)


if __name__ == "__main__":
    unittest.main()
