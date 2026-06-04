import unittest

from src.validate import any_validator


class TestAnyValidator(unittest.TestCase):
    def test_accepts_any_value(self):
        validator = any_validator()
        self.assertTrue(validator(0).validate)
        self.assertTrue(validator("").validate)
        self.assertTrue(validator({}).validate)
        self.assertTrue(validator(None).validate)
        self.assertTrue(validator([]).validate)

    def test_exposes_any_tag_through_type_field(self):
        validator = any_validator()
        self.assertEqual(validator(42).type, "any")

    def test_message_is_empty(self):
        validator = any_validator()
        self.assertEqual(validator("anything").message, "")

    def test_docstring_example(self):
        validator = any_validator()
        self.assertTrue(validator(123).validate)
        self.assertEqual(validator(None).type, "any")


if __name__ == "__main__":
    unittest.main()
