import unittest

from src.date import get_day


class TestGetDay(unittest.TestCase):
    def test_basic_cases_japanese(self):
        self.assertEqual(get_day(0), "\u65e5")
        self.assertEqual(get_day(1), "\u6708")
        self.assertEqual(get_day(6), "\u571f")

    def test_english(self):
        self.assertEqual(get_day(0, "en"), "Sun")
        self.assertEqual(get_day(1, "en"), "Mon")
        self.assertEqual(get_day(2, "en"), "Tue")
        self.assertEqual(get_day(3, "en"), "Wed")
        self.assertEqual(get_day(4, "en"), "Thu")
        self.assertEqual(get_day(5, "en"), "Fri")
        self.assertEqual(get_day(6, "en"), "Sat")

    def test_french(self):
        self.assertEqual(get_day(0, "fr"), "Dim")
        self.assertEqual(get_day(1, "fr"), "Lun")

    def test_german(self):
        self.assertEqual(get_day(0, "de"), "So")
        self.assertEqual(get_day(1, "de"), "Mo")

    def test_korean(self):
        self.assertEqual(get_day(0, "ko"), "\uc77c")
        self.assertEqual(get_day(1, "ko"), "\uc6d4")

    def test_invalid_day_below_range(self):
        self.assertEqual(get_day(-1), "\u65e5")
        self.assertEqual(get_day(-1, "en"), "Sun")
        self.assertEqual(get_day(-100, "en"), "Sun")

    def test_invalid_day_above_range(self):
        self.assertEqual(get_day(7), "\u65e5")
        self.assertEqual(get_day(7, "en"), "Sun")
        self.assertEqual(get_day(100, "en"), "Sun")

    def test_docstring_examples(self):
        self.assertEqual(get_day(0), "\u65e5")
        self.assertEqual(get_day(0, "en"), "Sun")
        self.assertEqual(get_day(1, "fr"), "Lun")


if __name__ == "__main__":
    unittest.main()
