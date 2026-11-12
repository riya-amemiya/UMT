import re
import unittest

from src.validate import regex_match, string


class TestRegexMatch(unittest.TestCase):
    def test_returns_true_for_matching_strings(self):
        rule = regex_match(re.compile(r"^[0-9]+$"))
        self.assertTrue(rule.validate("123"))

    def test_returns_false_for_non_matching_strings(self):
        rule = regex_match(re.compile(r"^[0-9]+$"))
        self.assertFalse(rule.validate("12a"))
        self.assertFalse(rule.validate(""))

    def test_searches_anywhere_in_the_string(self):
        rule = regex_match(re.compile(r"[0-9]"))
        self.assertTrue(rule.validate("abc1def"))
        self.assertFalse(rule.validate("abcdef"))

    def test_carries_the_custom_message(self):
        rule = regex_match(re.compile("a"), "must contain a")
        self.assertEqual(rule.message, "must contain a")

    def test_defaults_the_message_to_empty_string(self):
        self.assertEqual(regex_match(re.compile("a")).message, "")

    def test_composes_with_string(self):
        validator = string([regex_match(re.compile("^a"), "must start with a")])
        self.assertTrue(validator("abc").validate)
        self.assertEqual(validator("bcd").message, "must start with a")


if __name__ == "__main__":
    unittest.main()
