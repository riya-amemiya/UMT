import unittest
from datetime import datetime, timezone, timedelta

from src.date import date_format


class TestDateFormat(unittest.TestCase):
    def test_basic_iso_format(self):
        dt = datetime(2025, 4, 4, 12, 30, 45, 123000, tzinfo=timezone.utc)
        result = date_format(dt, "YYYY-MM-DD")
        self.assertEqual(result, "2025-04-04")

    def test_invalid_date_type(self):
        with self.assertRaises(TypeError) as context:
            date_format("2025-04-04", "YYYY-MM-DD")
        self.assertIn("Invalid Date in format", str(context.exception))

        with self.assertRaises(TypeError):
            date_format(None, "YYYY-MM-DD")

        with self.assertRaises(TypeError):
            date_format(12345, "YYYY-MM-DD")

    def test_year_formats(self):
        dt = datetime(2025, 4, 4, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "YYYY"), "2025")
        self.assertEqual(date_format(dt, "YY"), "25")

    def test_month_formats(self):
        dt = datetime(2025, 4, 4, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "MM"), "04")
        self.assertEqual(date_format(dt, "M"), "4")

        dt_double_digit = datetime(2025, 12, 4, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_double_digit, "MM"), "12")
        self.assertEqual(date_format(dt_double_digit, "M"), "12")

    def test_day_formats(self):
        dt = datetime(2025, 4, 4, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "DD"), "04")
        self.assertEqual(date_format(dt, "D"), "4")

        dt_double_digit = datetime(2025, 4, 25, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_double_digit, "DD"), "25")
        self.assertEqual(date_format(dt_double_digit, "D"), "25")

    def test_day_of_week(self):
        dt = datetime(2025, 4, 4, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "d"), "4")

    def test_24_hour_formats(self):
        dt = datetime(2025, 4, 4, 9, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "HH"), "09")
        self.assertEqual(date_format(dt, "H"), "9")

        dt_afternoon = datetime(2025, 4, 4, 15, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_afternoon, "HH"), "15")
        self.assertEqual(date_format(dt_afternoon, "H"), "15")

    def test_12_hour_formats(self):
        dt_morning = datetime(2025, 4, 4, 9, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_morning, "hh"), "09")
        self.assertEqual(date_format(dt_morning, "h"), "9")

        dt_afternoon = datetime(2025, 4, 4, 15, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_afternoon, "hh"), "03")
        self.assertEqual(date_format(dt_afternoon, "h"), "3")

        dt_midnight = datetime(2025, 4, 4, 0, 0, 0, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_midnight, "hh"), "12")
        self.assertEqual(date_format(dt_midnight, "h"), "12")

        dt_noon = datetime(2025, 4, 4, 12, 0, 0, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_noon, "hh"), "12")
        self.assertEqual(date_format(dt_noon, "h"), "12")

    def test_ampm_formats(self):
        dt_morning = datetime(2025, 4, 4, 9, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_morning, "A"), "AM")
        self.assertEqual(date_format(dt_morning, "a"), "am")

        dt_afternoon = datetime(2025, 4, 4, 15, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_afternoon, "A"), "PM")
        self.assertEqual(date_format(dt_afternoon, "a"), "pm")

    def test_minute_formats(self):
        dt = datetime(2025, 4, 4, 9, 5, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "mm"), "05")
        self.assertEqual(date_format(dt, "m"), "5")

        dt_double_digit = datetime(2025, 4, 4, 9, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_double_digit, "mm"), "30")
        self.assertEqual(date_format(dt_double_digit, "m"), "30")

    def test_second_formats(self):
        dt = datetime(2025, 4, 4, 9, 30, 5, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "ss"), "05")
        self.assertEqual(date_format(dt, "s"), "5")

        dt_double_digit = datetime(2025, 4, 4, 9, 30, 45, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_double_digit, "ss"), "45")
        self.assertEqual(date_format(dt_double_digit, "s"), "45")

    def test_millisecond_format(self):
        dt = datetime(2025, 4, 4, 9, 30, 45, 123000, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt, "SSS"), "123")

        dt_small_ms = datetime(2025, 4, 4, 9, 30, 45, 5000, tzinfo=timezone.utc)
        self.assertEqual(date_format(dt_small_ms, "SSS"), "005")

    def test_timezone_formats(self):
        jst = timezone(timedelta(hours=9))
        dt = datetime(2025, 4, 4, 9, 30, 45, tzinfo=jst)
        self.assertEqual(date_format(dt, "Z"), "+09:00")
        self.assertEqual(date_format(dt, "ZZ"), "+0900")

        utc = timezone.utc
        dt_utc = datetime(2025, 4, 4, 9, 30, 45, tzinfo=utc)
        self.assertEqual(date_format(dt_utc, "Z"), "+00:00")
        self.assertEqual(date_format(dt_utc, "ZZ"), "+0000")

    def test_complex_format_string(self):
        jst = timezone(timedelta(hours=9))
        dt = datetime(2025, 4, 4, 15, 30, 45, 123000, tzinfo=jst)
        result = date_format(dt, "YYYY-MM-DDTHH:mm:ss.SSSZ")
        self.assertEqual(result, "2025-04-04T15:30:45.123+09:00")

    def test_escaped_characters(self):
        dt = datetime(2025, 4, 4, tzinfo=timezone.utc)
        result = date_format(dt, "[Today is] YYYY-MM-DD")
        self.assertEqual(result, "Today is 2025-04-04")

        result = date_format(dt, "[YYYY] YYYY [MM] MM")
        self.assertEqual(result, "YYYY 2025 MM 04")

    def test_unknown_token_preserved(self):
        dt = datetime(2025, 4, 4, tzinfo=timezone.utc)
        result = date_format(dt, "YYYY-XX-DD")

        self.assertIn("2025", result)
        self.assertIn("04", result)

    def test_default_format(self):
        dt = datetime(2025, 4, 4, 12, 30, 45, tzinfo=timezone.utc)
        result = date_format(dt)
        self.assertIn("2025-04-04", result)
        self.assertIn("12:30:45", result)

    def test_docstring_example(self):
        self.assertEqual(date_format(datetime(2025, 4, 4), "YYYY-MM-DD"), "2025-04-04")


if __name__ == "__main__":
    unittest.main()
