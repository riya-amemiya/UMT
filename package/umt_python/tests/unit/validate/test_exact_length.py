import unittest

from src.validate import exact_length, string


class TestExactLength(unittest.TestCase):
    def test_returns_true_for_strings_with_the_exact_length(self):
        rule = exact_length(3)
        self.assertTrue(rule.validate("abc"))
        self.assertTrue(exact_length(0).validate(""))

    def test_returns_false_for_other_lengths(self):
        rule = exact_length(3)
        self.assertFalse(rule.validate("ab"))
        self.assertFalse(rule.validate("abcd"))
        self.assertFalse(rule.validate(""))

    def test_counts_utf16_code_units(self):
        emoji = chr(0x1F600)
        self.assertTrue(exact_length(2).validate(emoji))
        self.assertFalse(exact_length(1).validate(emoji))
        self.assertTrue(exact_length(1).validate(chr(0x3042)))

    def test_carries_the_custom_message(self):
        self.assertEqual(exact_length(3, "bad length").message, "bad length")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(exact_length(3).message, "")

    def test_composes_with_string(self):
        validator = string([exact_length(3, "bad length")])
        self.assertTrue(validator("abc").validate)
        self.assertEqual(validator("ab").message, "bad length")


if __name__ == "__main__":
    unittest.main()
