import unittest

from src.validate import max_length, string


class TestMaxLength(unittest.TestCase):
    def test_returns_true_for_strings_at_or_below_the_maximum(self):
        rule = max_length(3)
        self.assertTrue(rule.validate("abc"))
        self.assertTrue(rule.validate(""))

    def test_returns_false_for_longer_strings(self):
        self.assertFalse(max_length(3).validate("abcd"))

    def test_counts_utf16_code_units(self):
        emoji = chr(0x1F600)
        self.assertTrue(max_length(2).validate(emoji))
        self.assertFalse(max_length(1).validate(emoji))

    def test_carries_the_custom_message(self):
        self.assertEqual(max_length(3, "too long").message, "too long")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(max_length(3).message, "")

    def test_composes_with_string(self):
        validator = string([max_length(3, "too long")])
        self.assertTrue(validator("abc").validate)
        self.assertEqual(validator("abcd").message, "too long")


if __name__ == "__main__":
    unittest.main()
