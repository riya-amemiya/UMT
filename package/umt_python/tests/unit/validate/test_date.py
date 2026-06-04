import unittest
from datetime import datetime

from src.validate import date


class TestDate(unittest.TestCase):
    def test_accepts_valid_datetime_instances(self):
        validator = date()
        self.assertTrue(validator(datetime(2026, 5, 7)).validate)
        self.assertTrue(validator(datetime.now()).validate)

    def test_rejects_non_datetime_values(self):
        validator = date()
        self.assertFalse(validator("2026-05-07").validate)
        self.assertFalse(validator(0).validate)
        self.assertFalse(validator(None).validate)

    def test_returns_configured_message_on_failure(self):
        validator = date("must be a date")
        self.assertEqual(validator("nope").message, "must be a date")

    def test_exposes_value_through_type_field(self):
        validator = date()
        value = datetime(2026, 5, 7)
        self.assertIs(validator(value).type, value)

    def test_docstring_example(self):
        validator = date()
        self.assertTrue(validator(datetime(2026, 5, 7)).validate)
        self.assertFalse(validator("2026-05-07").validate)
        self.assertEqual(date("must be a date")("nope").message, "must be a date")


if __name__ == "__main__":
    unittest.main()
