import unittest
from datetime import datetime, timezone

from src.simple.date import day_of_week_simple


class TestDayOfWeekSimpleSundayStart(unittest.TestCase):
    def test_hyphen_string_sunday_is_0(self):
        self.assertEqual(day_of_week_simple("2022-01-02"), 0)

    def test_slash_string_monday_is_1(self):
        self.assertEqual(day_of_week_simple("2022/01/03"), 1)

    def test_colon_string_tuesday_is_2(self):
        self.assertEqual(day_of_week_simple("2022:01:04"), 2)

    def test_dict_wednesday_is_3(self):
        self.assertEqual(day_of_week_simple({"year": 2022, "mon": 1, "day": 5}), 3)

    def test_datetime_saturday_is_6(self):
        dt = datetime(2022, 1, 8, tzinfo=timezone.utc)
        self.assertEqual(day_of_week_simple(dt), 6)

    def test_none_returns_a_weekday_number(self):
        self.assertIn(day_of_week_simple(), range(7))

    def test_unparseable_string_falls_back_to_today(self):
        self.assertEqual(day_of_week_simple("invalid"), day_of_week_simple())

    def test_string_with_non_numeric_part_falls_back_to_today(self):
        self.assertEqual(day_of_week_simple("2022-xx-02"), day_of_week_simple())

    def test_partial_dict_uses_current_values(self):
        self.assertIn(day_of_week_simple({"year": 2022}), range(7))


if __name__ == "__main__":
    unittest.main()
