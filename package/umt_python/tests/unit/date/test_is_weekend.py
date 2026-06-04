import unittest
from datetime import datetime

from src.date import is_weekend


class TestIsWeekend(unittest.TestCase):
    def test_returns_true_on_saturday(self):
        self.assertTrue(is_weekend(datetime(2025, 4, 19)))

    def test_returns_true_on_sunday(self):
        self.assertTrue(is_weekend(datetime(2025, 4, 20)))

    def test_returns_false_on_monday(self):
        self.assertFalse(is_weekend(datetime(2025, 4, 21)))

    def test_returns_false_on_friday(self):
        self.assertFalse(is_weekend(datetime(2025, 4, 18)))


if __name__ == "__main__":
    unittest.main()
