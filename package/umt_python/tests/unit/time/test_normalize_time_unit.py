import unittest

from src.time import normalize_time_unit


class TestNormalizeTimeUnit(unittest.TestCase):
    def test_short_to_long(self):
        self.assertEqual(normalize_time_unit("ms", "long"), "milliseconds")
        self.assertEqual(normalize_time_unit("s", "long"), "seconds")
        self.assertEqual(normalize_time_unit("m", "long"), "minutes")
        self.assertEqual(normalize_time_unit("h", "long"), "hours")

    def test_long_to_short(self):
        self.assertEqual(normalize_time_unit("milliseconds", "short"), "ms")
        self.assertEqual(normalize_time_unit("seconds", "short"), "s")
        self.assertEqual(normalize_time_unit("minutes", "short"), "m")
        self.assertEqual(normalize_time_unit("hours", "short"), "h")

    def test_long_to_long(self):
        self.assertEqual(normalize_time_unit("milliseconds", "long"), "milliseconds")
        self.assertEqual(normalize_time_unit("seconds", "long"), "seconds")
        self.assertEqual(normalize_time_unit("minutes", "long"), "minutes")
        self.assertEqual(normalize_time_unit("hours", "long"), "hours")

    def test_short_to_short(self):
        self.assertEqual(normalize_time_unit("ms", "short"), "ms")
        self.assertEqual(normalize_time_unit("s", "short"), "s")
        self.assertEqual(normalize_time_unit("m", "short"), "m")
        self.assertEqual(normalize_time_unit("h", "short"), "h")

    def test_docstring_examples(self):
        self.assertEqual(normalize_time_unit("ms", "long"), "milliseconds")
        self.assertEqual(normalize_time_unit("hours", "short"), "h")


if __name__ == "__main__":
    unittest.main()
