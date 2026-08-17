import unittest
from datetime import datetime

from src.date import add_business_days, sub_business_days


class TestSubBusinessDays(unittest.TestCase):
    def test_subtracts_one_business_day_from_monday_to_friday(self):
        result = sub_business_days(datetime(2025, 4, 21, 9, 30), 1)
        self.assertEqual(result, datetime(2025, 4, 18, 9, 30))

    def test_matches_add_business_days_with_negated_amount(self):
        date = datetime(2025, 4, 21)
        holidays = [datetime(2025, 4, 18)]
        self.assertEqual(
            sub_business_days(date, 2, holidays),
            add_business_days(date, -2, holidays),
        )

    def test_returns_a_clone_when_amount_is_zero(self):
        date = datetime(2025, 4, 21, 8, 0)
        result = sub_business_days(date, 0)
        self.assertEqual(result, date)
        self.assertIsNot(result, date)


if __name__ == "__main__":
    unittest.main()
