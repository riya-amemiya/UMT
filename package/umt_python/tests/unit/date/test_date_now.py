import unittest
from datetime import datetime, timedelta

from src.date import date_now


class TestDateNow(unittest.TestCase):
    def test_basic_cases(self):
        result = date_now()
        self.assertIsNotNone(result)
        self.assertIsInstance(result, datetime)

    def test_timezone_offset(self):
        result_jst = date_now(9)
        result_utc = date_now(0)
        offset_jst = result_jst.utcoffset()
        offset_utc = result_utc.utcoffset()
        assert offset_jst is not None
        assert offset_utc is not None
        diff = offset_jst - offset_utc
        self.assertEqual(diff, timedelta(hours=9))


if __name__ == "__main__":
    unittest.main()
