import unittest

from src.date import from_unix, to_unix


class TestToUnix(unittest.TestCase):
    def test_returns_floored_seconds_by_default(self):
        self.assertEqual(to_unix(from_unix(0)), 0)
        self.assertEqual(to_unix(from_unix(1_700_000_000_999, "ms")), 1_700_000_000)

    def test_returns_milliseconds_when_unit_is_ms(self):
        date = from_unix(1_700_000_000_123, "ms")
        self.assertEqual(to_unix(date, "ms"), 1_700_000_000_123)

    def test_returns_floored_seconds_when_unit_is_s(self):
        self.assertEqual(to_unix(from_unix(1500, "ms"), "s"), 1)


if __name__ == "__main__":
    unittest.main()
