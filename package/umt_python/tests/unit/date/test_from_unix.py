import unittest

from src.date import from_unix


class TestFromUnix(unittest.TestCase):
    def test_creates_datetime_from_seconds_by_default(self):
        self.assertEqual(from_unix(0).timestamp(), 0)
        self.assertEqual(from_unix(1_700_000_000).timestamp(), 1_700_000_000)

    def test_creates_datetime_from_milliseconds_when_unit_is_ms(self):
        self.assertEqual(
            from_unix(1_700_000_000_123, "ms").timestamp(), 1_700_000_000.123
        )

    def test_creates_datetime_from_seconds_when_unit_is_s(self):
        self.assertEqual(from_unix(1_700_000_000, "s").timestamp(), 1_700_000_000)


if __name__ == "__main__":
    unittest.main()
