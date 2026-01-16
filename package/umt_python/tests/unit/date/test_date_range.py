import unittest
from datetime import datetime

from src.date import date_range


class TestDateRange(unittest.TestCase):
    def test_basic_cases(self):
        start = datetime(2025, 1, 1)
        end = datetime(2025, 1, 3)
        result = date_range(start, end)
        self.assertEqual(len(result), 3)
        self.assertEqual(result[0], datetime(2025, 1, 1))
        self.assertEqual(result[1], datetime(2025, 1, 2))
        self.assertEqual(result[2], datetime(2025, 1, 3))

    def test_single_day(self):
        start = datetime(2025, 1, 1)
        end = datetime(2025, 1, 1)
        result = date_range(start, end)
        self.assertEqual(len(result), 1)

    def test_edge_cases(self):
        start = datetime(2025, 1, 5)
        end = datetime(2025, 1, 1)
        result = date_range(start, end)
        self.assertEqual(len(result), 0)

    def test_docstring_example(self):
        result = date_range(datetime(2025, 1, 1), datetime(2025, 1, 3))
        self.assertEqual(len(result), 3)


if __name__ == "__main__":
    unittest.main()
