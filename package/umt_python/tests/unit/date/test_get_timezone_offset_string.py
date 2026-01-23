import unittest
from datetime import datetime, timezone, timedelta, tzinfo

from src.date import get_timezone_offset_string


class TestGetTimezoneOffsetString(unittest.TestCase):
    def test_positive_timezone_offset(self):
        jst = timezone(timedelta(hours=9))
        dt = datetime(2021, 1, 1, tzinfo=jst)
        self.assertEqual(get_timezone_offset_string(dt), "+09:00")

    def test_negative_timezone_offset(self):
        pst = timezone(timedelta(hours=-8))
        dt = datetime(2021, 1, 1, tzinfo=pst)
        self.assertEqual(get_timezone_offset_string(dt), "-08:00")

    def test_utc_timezone(self):
        dt = datetime(2021, 1, 1, tzinfo=timezone.utc)
        self.assertEqual(get_timezone_offset_string(dt), "+00:00")

    def test_timezone_with_minutes(self):
        ist = timezone(timedelta(hours=5, minutes=30))
        dt = datetime(2021, 1, 1, tzinfo=ist)
        self.assertEqual(get_timezone_offset_string(dt), "+05:30")

        nst = timezone(timedelta(hours=-3, minutes=-30))
        dt = datetime(2021, 1, 1, tzinfo=nst)
        self.assertEqual(get_timezone_offset_string(dt), "-03:30")

    def test_naive_datetime(self):
        dt = datetime(2021, 1, 1)
        result = get_timezone_offset_string(dt)

        self.assertRegex(result, r"^[+-]\d{2}:\d{2}$")

    def test_timezone_with_none_offset(self):
        class NoneOffsetTz(tzinfo):
            def utcoffset(self, dt):
                return None

            def tzname(self, dt):
                return "NoneOffset"

            def dst(self, dt):
                return timedelta(0)

        dt = datetime(2021, 1, 1, tzinfo=NoneOffsetTz())
        result = get_timezone_offset_string(dt)
        self.assertEqual(result, "+00:00")

    def test_docstring_example(self):
        jst = timezone(timedelta(hours=9))
        self.assertEqual(
            get_timezone_offset_string(datetime(2021, 1, 1, tzinfo=jst)), "+09:00"
        )


if __name__ == "__main__":
    unittest.main()
