import unittest
from datetime import datetime

from src.date import new_date_int, new_date_string


class TestNewDateInt(unittest.TestCase):
    def test_basic_date(self):
        result = new_date_int(2021, 1, 1, 0)
        self.assertEqual(result.year, 2021)
        self.assertEqual(result.month, 1)
        self.assertEqual(result.day, 1)
        self.assertEqual(result.hour, 0)

    def test_full_datetime(self):
        result = new_date_int(2021, 6, 15, 12, 30, 45, 500)
        self.assertEqual(result.year, 2021)
        self.assertEqual(result.month, 6)
        self.assertEqual(result.day, 15)
        self.assertEqual(result.hour, 12)
        self.assertEqual(result.minute, 30)
        self.assertEqual(result.second, 45)
        self.assertEqual(result.microsecond, 500000)

    def test_default_hours_uses_local_timezone(self):
        result = new_date_int(2021, 1, 1)
        self.assertIsInstance(result, datetime)

        self.assertGreaterEqual(result.hour, 0)
        self.assertLessEqual(result.hour, 23)

    def test_returns_datetime_object(self):
        result = new_date_int(2021, 1, 1, 0)
        self.assertIsInstance(result, datetime)


class TestNewDateString(unittest.TestCase):
    def test_basic_date_string(self):
        result = new_date_string("2021-01-01")
        self.assertEqual(result.year, 2021)
        self.assertEqual(result.month, 1)
        self.assertEqual(result.day, 1)

    def test_full_datetime_string(self):
        result = new_date_string("2021-06-15", "12", "30", "45", "500", "09")
        self.assertEqual(result.year, 2021)
        self.assertEqual(result.month, 6)
        self.assertEqual(result.day, 15)
        self.assertEqual(result.hour, 12)
        self.assertEqual(result.minute, 30)
        self.assertEqual(result.second, 45)

    def test_with_time_components(self):
        result = new_date_string("2021-01-01", "23", "59", "59", "999")
        self.assertEqual(result.hour, 23)
        self.assertEqual(result.minute, 59)
        self.assertEqual(result.second, 59)

    def test_returns_datetime_object(self):
        result = new_date_string("2021-01-01")
        self.assertIsInstance(result, datetime)


if __name__ == "__main__":
    unittest.main()
