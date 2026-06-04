import unittest

from src.validate import never_validator


class TestNeverValidator(unittest.TestCase):
    def test_rejects_every_value(self):
        validator = never_validator()
        self.assertFalse(validator(0).validate)
        self.assertFalse(validator("").validate)
        self.assertFalse(validator(None).validate)
        self.assertFalse(validator({}).validate)
        self.assertFalse(validator([]).validate)

    def test_exposes_never_tag_through_type_field(self):
        validator = never_validator()
        self.assertEqual(validator(42).type, "never")

    def test_returns_the_configured_message_on_failure(self):
        validator = never_validator("must never be set")
        self.assertEqual(validator(0).message, "must never be set")

    def test_message_defaults_to_empty(self):
        validator = never_validator()
        self.assertEqual(validator("anything").message, "")

    def test_docstring_example(self):
        validator = never_validator()
        self.assertFalse(validator(123).validate)
        self.assertEqual(validator(None).type, "never")


if __name__ == "__main__":
    unittest.main()
