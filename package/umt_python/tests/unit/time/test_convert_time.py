import unittest

from src.time import convert_time


class TestConvertTime(unittest.TestCase):
    def test_hours_to_minutes(self):
        self.assertEqual(convert_time(1, "hours", "minutes"), 60.0)
        self.assertEqual(convert_time(2, "hours", "minutes"), 120.0)

    def test_hours_to_seconds(self):
        self.assertEqual(convert_time(1, "hours", "seconds"), 3600.0)

    def test_hours_to_milliseconds(self):
        self.assertEqual(convert_time(1, "hours", "milliseconds"), 3600000.0)

    def test_minutes_to_hours(self):
        self.assertEqual(convert_time(60, "minutes", "hours"), 1.0)

    def test_minutes_to_seconds(self):
        self.assertEqual(convert_time(1, "minutes", "seconds"), 60.0)

    def test_seconds_to_minutes(self):
        self.assertEqual(convert_time(60, "seconds", "minutes"), 1.0)

    def test_seconds_to_milliseconds(self):
        self.assertEqual(convert_time(1, "seconds", "milliseconds"), 1000.0)

    def test_milliseconds_to_seconds(self):
        self.assertEqual(convert_time(1000, "milliseconds", "seconds"), 1.0)

    def test_string_value_input(self):
        self.assertEqual(convert_time("1000", "ms", "s"), 1.0)
        self.assertEqual(convert_time("60", "minutes", "hours"), 1.0)

    def test_short_unit_names(self):
        self.assertEqual(convert_time(1000, "ms", "s"), 1.0)
        self.assertEqual(convert_time(1, "h", "m"), 60.0)
        self.assertEqual(convert_time(60, "s", "m"), 1.0)

    def test_mixed_unit_names(self):
        self.assertEqual(convert_time(1, "hours", "m"), 60.0)
        self.assertEqual(convert_time(1000, "milliseconds", "s"), 1.0)

    def test_float_values(self):
        self.assertEqual(convert_time(0.5, "hours", "minutes"), 30.0)
        self.assertEqual(convert_time(1.5, "hours", "minutes"), 90.0)

    def test_docstring_examples(self):
        self.assertEqual(convert_time(1, "hours", "minutes"), 60.0)
        self.assertEqual(convert_time("1000", "ms", "s"), 1.0)


if __name__ == "__main__":
    unittest.main()
