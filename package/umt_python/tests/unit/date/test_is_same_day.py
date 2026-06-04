import unittest
from datetime import datetime

from src.date import is_same_day


class TestIsSameDay(unittest.TestCase):
    def test_returns_true_for_same_date_with_different_times(self):
        self.assertTrue(
            is_same_day(datetime(2025, 4, 15, 1, 0), datetime(2025, 4, 15, 23, 59))
        )

    def test_returns_false_for_adjacent_days(self):
        self.assertFalse(is_same_day(datetime(2025, 4, 15), datetime(2025, 4, 16)))

    def test_returns_false_for_different_month(self):
        self.assertFalse(is_same_day(datetime(2025, 4, 15), datetime(2025, 5, 15)))

    def test_returns_false_for_different_year(self):
        self.assertFalse(is_same_day(datetime(2024, 4, 15), datetime(2025, 4, 15)))


if __name__ == "__main__":
    unittest.main()
