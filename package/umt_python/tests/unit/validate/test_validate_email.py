import unittest

from src.validate import ParseEmailOptions, string, validate_email


class TestValidateEmail(unittest.TestCase):
    def test_returns_true_for_valid_emails(self):
        rule = validate_email()
        self.assertTrue(rule.validate("user@example.com"))
        self.assertTrue(rule.validate("user.name+tag@sub.example.com"))

    def test_returns_false_for_invalid_emails(self):
        rule = validate_email()
        self.assertFalse(rule.validate("invalid-email"))
        self.assertFalse(rule.validate("user@"))
        self.assertFalse(rule.validate(""))

    def test_defaults_to_the_basic_level(self):
        self.assertTrue(validate_email().validate("user@localhost"))

    def test_uses_the_given_level(self):
        rule = validate_email("bad email", ParseEmailOptions(level="rfc2822"))
        self.assertFalse(rule.validate("user@localhost"))
        self.assertTrue(rule.validate("user@example.com"))

    def test_carries_the_custom_message(self):
        self.assertEqual(validate_email("bad email").message, "bad email")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(validate_email().message, "")

    def test_composes_with_string(self):
        validator = string([validate_email("bad email")])
        self.assertTrue(validator("user@example.com").validate)
        self.assertEqual(validator("nope").message, "bad email")


if __name__ == "__main__":
    unittest.main()
