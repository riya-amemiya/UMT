import unittest
from datetime import datetime

from src.date import add_business_days


class TestAddBusinessDays(unittest.TestCase):
    def test_adds_one_business_day_from_friday_to_monday(self):
        result = add_business_days(datetime(2025, 4, 18, 9, 30), 1)
        self.assertEqual(result, datetime(2025, 4, 21, 9, 30))

    def test_adds_one_business_day_from_saturday_to_monday(self):
        result = add_business_days(datetime(2025, 4, 19), 1)
        self.assertEqual(result.day, 21)

    def test_subtracts_one_business_day_from_monday_to_friday(self):
        result = add_business_days(datetime(2025, 4, 21), -1)
        self.assertEqual(result.day, 18)

    def test_returns_a_clone_when_amount_is_zero(self):
        saturday = datetime(2025, 4, 19, 12, 0)
        result = add_business_days(saturday, 0)
        self.assertEqual(result, saturday)
        self.assertIsNot(result, saturday)

    def test_skips_holidays_when_walking_forward(self):
        result = add_business_days(datetime(2025, 4, 21), 1, [datetime(2025, 4, 22)])
        self.assertEqual(result.day, 23)

    def test_skips_holidays_when_walking_backward(self):
        result = add_business_days(datetime(2025, 4, 23), -1, [datetime(2025, 4, 22)])
        self.assertEqual(result.day, 21)

    def test_does_not_mutate_the_input_date(self):
        friday = datetime(2025, 4, 18)
        add_business_days(friday, 1)
        self.assertEqual(friday, datetime(2025, 4, 18))


if __name__ == "__main__":
    unittest.main()
