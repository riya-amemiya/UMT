import unittest
from datetime import datetime

from src.date import get_quarter


class TestGetQuarter(unittest.TestCase):
    def test_returns_1_for_january_through_march(self):
        self.assertEqual(get_quarter(datetime(2025, 1, 1)), 1)
        self.assertEqual(get_quarter(datetime(2025, 3, 31)), 1)

    def test_returns_2_for_april_through_june(self):
        self.assertEqual(get_quarter(datetime(2025, 4, 1)), 2)
        self.assertEqual(get_quarter(datetime(2025, 6, 30)), 2)

    def test_returns_3_for_july_through_september(self):
        self.assertEqual(get_quarter(datetime(2025, 7, 1)), 3)
        self.assertEqual(get_quarter(datetime(2025, 9, 30)), 3)

    def test_returns_4_for_october_through_december(self):
        self.assertEqual(get_quarter(datetime(2025, 10, 1)), 4)
        self.assertEqual(get_quarter(datetime(2025, 12, 31)), 4)


if __name__ == "__main__":
    unittest.main()
