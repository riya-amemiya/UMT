import unittest
from datetime import datetime, timedelta

from src.date import format_relative


class TestFormatRelative(unittest.TestCase):
    def setUp(self):
        self.base = datetime(2025, 4, 15, 12, 0, 0)

    def test_returns_now_for_delta_below_one_second(self):
        target = self.base + timedelta(milliseconds=500)
        self.assertEqual(format_relative(target, self.base, "en"), "now")

    def test_formats_seconds_in_the_past(self):
        target = self.base - timedelta(seconds=5)
        self.assertEqual(format_relative(target, self.base, "en"), "5 seconds ago")

    def test_formats_minutes_in_the_future(self):
        target = self.base + timedelta(minutes=5)
        self.assertEqual(format_relative(target, self.base, "en"), "in 5 minutes")

    def test_formats_hours(self):
        target = self.base - timedelta(hours=3)
        self.assertEqual(format_relative(target, self.base, "en"), "3 hours ago")

    def test_formats_days(self):
        target = self.base + timedelta(days=1)
        self.assertEqual(format_relative(target, self.base, "en"), "tomorrow")

    def test_formats_weeks(self):
        target = self.base - timedelta(days=14)
        self.assertEqual(format_relative(target, self.base, "en"), "2 weeks ago")

    def test_formats_months(self):
        target = self.base + timedelta(days=90)
        self.assertEqual(format_relative(target, self.base, "en"), "in 3 months")

    def test_formats_years(self):
        target = self.base - timedelta(days=2 * 365)
        self.assertEqual(format_relative(target, self.base, "en"), "2 years ago")

    def test_uses_default_base_date_when_omitted(self):
        target = datetime.now() + timedelta(seconds=5)
        result = format_relative(target, None, "en")
        self.assertIsInstance(result, str)
        self.assertGreater(len(result), 0)

    def test_formats_japanese_locale(self):
        self.assertEqual(
            format_relative(self.base + timedelta(days=1), self.base, "ja"), "明日"
        )
        self.assertEqual(
            format_relative(self.base - timedelta(seconds=5), self.base, "ja"),
            "5 秒前",
        )
        self.assertEqual(
            format_relative(self.base + timedelta(days=90), self.base, "ja"),
            "3 か月後",
        )


if __name__ == "__main__":
    unittest.main()
