import unittest

from src.validate import unknown_validator


class TestUnknownValidator(unittest.TestCase):
    def test_accepts_any_value(self):
        validator = unknown_validator()
        self.assertTrue(validator(0).validate)
        self.assertTrue(validator("").validate)
        self.assertTrue(validator({}).validate)
        self.assertTrue(validator(None).validate)
        self.assertTrue(validator([]).validate)

    def test_exposes_unknown_tag_through_type_field(self):
        validator = unknown_validator()
        self.assertEqual(validator(42).type, "unknown")

    def test_message_is_empty(self):
        validator = unknown_validator()
        self.assertEqual(validator("anything").message, "")

    def test_docstring_example(self):
        validator = unknown_validator()
        self.assertTrue(validator(123).validate)
        self.assertEqual(validator(None).type, "unknown")


if __name__ == "__main__":
    unittest.main()
