import unittest

from src.validate import min_length, string


class TestMinLength(unittest.TestCase):
    def test_returns_true_for_strings_at_or_above_the_minimum(self):
        rule = min_length(3)
        self.assertTrue(rule.validate("abc"))
        self.assertTrue(rule.validate("abcd"))

    def test_returns_false_for_shorter_strings(self):
        rule = min_length(3)
        self.assertFalse(rule.validate("ab"))
        self.assertFalse(rule.validate(""))

    def test_counts_utf16_code_units(self):
        emoji = chr(0x1F600)
        self.assertTrue(min_length(2).validate(emoji))
        self.assertFalse(min_length(3).validate(emoji))

    def test_carries_the_custom_message(self):
        self.assertEqual(min_length(3, "too short").message, "too short")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(min_length(3).message, "")

    def test_composes_with_string(self):
        validator = string([min_length(3, "too short")])
        self.assertTrue(validator("abc").validate)
        self.assertEqual(validator("ab").message, "too short")


if __name__ == "__main__":
    unittest.main()
