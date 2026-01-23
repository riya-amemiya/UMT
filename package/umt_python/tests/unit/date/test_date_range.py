import unittest
from datetime import datetime, timezone

from src.date import date_range


class TestDateRange(unittest.TestCase):
    def test_basic_cases(self):
        start = datetime(2025, 1, 1, tzinfo=timezone.utc)
        end = datetime(2025, 1, 3, tzinfo=timezone.utc)
        result = date_range(start, end)
        self.assertEqual(len(result), 3)
        self.assertEqual(result[0], datetime(2025, 1, 1, tzinfo=timezone.utc))
        self.assertEqual(result[1], datetime(2025, 1, 2, tzinfo=timezone.utc))
        self.assertEqual(result[2], datetime(2025, 1, 3, tzinfo=timezone.utc))

    def test_single_day(self):
        start = datetime(2025, 1, 1, tzinfo=timezone.utc)
        end = datetime(2025, 1, 1, tzinfo=timezone.utc)
        result = date_range(start, end)
        self.assertEqual(len(result), 1)

    def test_edge_cases(self):
        start = datetime(2025, 1, 5, tzinfo=timezone.utc)
        end = datetime(2025, 1, 1, tzinfo=timezone.utc)
        result = date_range(start, end)
        self.assertEqual(len(result), 0)

    def test_docstring_example(self):
        result = date_range(
            datetime(2025, 1, 1, tzinfo=timezone.utc),
            datetime(2025, 1, 3, tzinfo=timezone.utc),
        )
        self.assertEqual(len(result), 3)


if __name__ == "__main__":
    unittest.main()
