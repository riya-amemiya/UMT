import unittest

from src.validate import BooleanReturnType, boolean


class TestBoolean(unittest.TestCase):
    def test_validates_boolean_values(self):
        validator = boolean()
        self.assertTrue(validator(True).validate)
        self.assertTrue(validator(False).validate)

    def test_rejects_non_boolean_values(self):
        validator = boolean()
        self.assertFalse(validator(1).validate)
        self.assertFalse(validator(0).validate)
        self.assertFalse(validator("true").validate)
        self.assertFalse(validator(None).validate)

    def test_returns_custom_message_on_type_failure(self):
        validator = boolean("must be a boolean")
        self.assertEqual(validator("true").message, "must be a boolean")

    def test_returns_empty_message_on_success(self):
        validator = boolean("must be a boolean")
        self.assertEqual(validator(True).message, "")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(boolean()(1).message, "")

    def test_type_tag(self):
        validator = boolean()
        self.assertEqual(validator(True).type, "boolean")
        self.assertEqual(validator(1).type, "boolean")

    def test_returns_boolean_return_type(self):
        self.assertIsInstance(boolean()(True), BooleanReturnType)


if __name__ == "__main__":
    unittest.main()
